use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::{
        entity::reservation::{ReservationStatus, StayStatus},
        semantic::{
            guest_timeline_event::TimelineEventType,
            room_daily_state::{RoomDailyOccupancyStatus, RoomDailyState},
        },
    },
    error::app_error::{conflict, infra, not_found, AppResult},
    projection::{
        invalidation::{
            projection_invalidation::{ProjectionInvalidation, ProjectionRefreshTarget},
            projection_scope::ProjectionScope,
        },
        orchestrator::refresh_projection_chain::refresh_projection_chain,
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::operational::{
        reservation_repository::SqliteReservationRepository,
        room_daily_state_repository::SqliteRoomDailyStateRepository,
    },
    usecase::timeline::command::record_event::record_event,
};

pub async fn execute(db: &Db, reservation_id: Uuid) -> AppResult<()> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut reservation = SqliteReservationRepository::find_by_id(&mut tx, reservation_id)
            .await?
            .ok_or_else(|| not_found("reservation not found"))?;

        if reservation.reservation_status != ReservationStatus::Confirmed {
            return Err(conflict("reservation inactive"));
        }

        if reservation.stay_status != Some(StayStatus::CheckedIn) {
            return Err(conflict("invalid stay status"));
        }

        let room_id = reservation
            .room_id
            .ok_or_else(|| conflict("room not assigned"))?;

        let mut room_state = match SqliteRoomDailyStateRepository::find_by_room_and_service_date(
            &mut tx,
            room_id,
            reservation.check_out,
        )
        .await?
        {
            Some(state) => state,
            None => RoomDailyState::new(room_id, reservation.check_out),
        };

        room_state.set_occupancy_status(RoomDailyOccupancyStatus::Vacant);
        room_state.mark_dirty();

        SqliteRoomDailyStateRepository::save(&mut tx, &room_state).await?;

        refresh_projection_chain(
            &mut tx,
            ProjectionInvalidation::new(
                ProjectionNode::HousekeepingDailyWorkloadAggregate,
                ProjectionScope::Inventory,
                ProjectionRefreshTarget::RoomDate {
                    date: reservation.check_out.to_string(),
                },
            ),
        )
        .await?;

        refresh_projection_chain(
            &mut tx,
            ProjectionInvalidation::new(
                ProjectionNode::DailyRoomClassKpiAggregate,
                ProjectionScope::Inventory,
                ProjectionRefreshTarget::KpiDate {
                    date: reservation.check_out.to_string(),
                },
            ),
        )
        .await?;

        refresh_projection_chain(
            &mut tx,
            ProjectionInvalidation::new(
                ProjectionNode::DailyHotelKpiAggregate,
                ProjectionScope::Inventory,
                ProjectionRefreshTarget::KpiDate {
                    date: reservation.check_out.to_string(),
                },
            ),
        )
        .await?;

        refresh_projection_chain(
            &mut tx,
            ProjectionInvalidation::new(
                ProjectionNode::MonthlyRoomClassKpiAggregate,
                ProjectionScope::Inventory,
                ProjectionRefreshTarget::KpiMonth {
                    year_month: reservation.check_out.format("%Y-%m").to_string(),
                },
            ),
        )
        .await?;

        refresh_projection_chain(
            &mut tx,
            ProjectionInvalidation::new(
                ProjectionNode::MonthlyHotelKpiAggregate,
                ProjectionScope::Inventory,
                ProjectionRefreshTarget::KpiMonth {
                    year_month: reservation.check_out.format("%Y-%m").to_string(),
                },
            ),
        )
        .await?;

        reservation.stay_status = Some(StayStatus::CheckedOut);

        SqliteReservationRepository::modify(&mut tx, &reservation).await?;

        if let Some(guest_id) = reservation.primary_participant().map(|p| p.guest_id) {
            record_event(
                &mut tx,
                guest_id,
                TimelineEventType::CheckedOut,
                reservation.id,
            )
            .await?;
        }

        Ok(())
    }
    .await;

    match result {
        Ok(()) => {
            tx.commit().await.map_err(infra)?;

            Ok(())
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}
