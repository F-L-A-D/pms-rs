use chrono::NaiveDate;

use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::{
        entity::reservation::{Reservation, ReservationStatus, StayStatus},
        semantic::{
            guest_timeline_event::TimelineEventType,
            reservation_transition::{ReservationTransition, ReservationTransitionType},
            room_daily_state::{RoomDailyOccupancyStatus, RoomDailyState},
        },
    },
    error::app_error::{conflict, infra, not_found, validation, AppResult},
    projection::{
        invalidation::{
            projection_invalidation::{ProjectionInvalidation, ProjectionRefreshTarget},
            projection_scope::ProjectionScope,
        },
        orchestrator::refresh_projection_chain::refresh_projection_chain,
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::{
        behavioral::reservation_transition_repository::SqliteReservationTransitionRepository,
        operational::{
            reservation_repository::SqliteReservationRepository,
            room_daily_state_repository::SqliteRoomDailyStateRepository,
            room_repository::SqliteRoomRepository,
        },
    },
    usecase::timeline::command::record_event::record_event,
};

pub async fn execute(
    db: &Db,
    reservation_id: Uuid,
    new_room_id: Uuid,
    effective_date: NaiveDate,
) -> AppResult<()> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut reservation = SqliteReservationRepository::find_by_id(&mut tx, reservation_id)
            .await?
            .ok_or_else(|| not_found("reservation not found"))?;

        if reservation.reservation_status != ReservationStatus::Confirmed {
            return Err(conflict("reservation inactive"));
        }

        if reservation.stay_status != Some(StayStatus::CheckedIn) {
            return Err(conflict("room move requires checked-in stay"));
        }

        if !reservation.nights().contains(&effective_date) {
            return Err(validation(
                "effective_date must be within reservation nights",
            ));
        }

        let old_room_id = reservation
            .room_id
            .ok_or_else(|| conflict("room not assigned"))?;

        if old_room_id == new_room_id {
            return Err(conflict("new room must differ from current room"));
        }

        let new_room = SqliteRoomRepository::find_by_id(&mut tx, new_room_id)
            .await?
            .ok_or_else(|| not_found("room not found"))?;

        if !new_room.is_active {
            return Err(conflict("room inactive"));
        }

        for service_date in move_dates(&reservation, effective_date) {
            let mut new_room_state =
                match SqliteRoomDailyStateRepository::find_by_room_and_service_date(
                    &mut tx,
                    new_room_id,
                    service_date,
                )
                .await?
                {
                    Some(state) => state,
                    None => RoomDailyState::new(new_room_id, service_date),
                };

            if new_room_state.occupancy_status == RoomDailyOccupancyStatus::OutOfOrder {
                return Err(conflict("new room out of order"));
            }

            if new_room_state.occupancy_status == RoomDailyOccupancyStatus::Occupied {
                return Err(conflict("new room occupied"));
            }

            new_room_state.set_occupancy_status(RoomDailyOccupancyStatus::Occupied);
            SqliteRoomDailyStateRepository::save(&mut tx, &new_room_state).await?;

            let mut old_room_state =
                match SqliteRoomDailyStateRepository::find_by_room_and_service_date(
                    &mut tx,
                    old_room_id,
                    service_date,
                )
                .await?
                {
                    Some(state) => state,
                    None => RoomDailyState::new(old_room_id, service_date),
                };

            old_room_state.set_occupancy_status(RoomDailyOccupancyStatus::Vacant);

            if service_date == effective_date {
                old_room_state.mark_dirty();
            }

            SqliteRoomDailyStateRepository::save(&mut tx, &old_room_state).await?;

            refresh_room_date(&mut tx, service_date).await?;
        }

        reservation.room_id = Some(new_room_id);

        SqliteReservationRepository::modify(&mut tx, &mut reservation).await?;

        SqliteReservationTransitionRepository::save(
            &mut tx,
            &ReservationTransition {
                id: Uuid::new_v4(),
                reservation_id: reservation.id,
                transition_type: ReservationTransitionType::RoomMoved,
                field_name: "room_id".to_string(),
                before_value: old_room_id.to_string(),
                after_value: new_room_id.to_string(),
                occurred_at: chrono::Utc::now(),
            },
        )
        .await?;

        if let Some(guest_id) = reservation.primary_participant().map(|p| p.guest_id) {
            record_event(
                &mut tx,
                guest_id,
                TimelineEventType::RoomMoved,
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

fn move_dates(reservation: &Reservation, effective_date: NaiveDate) -> Vec<NaiveDate> {
    reservation
        .nights()
        .into_iter()
        .filter(|service_date| *service_date >= effective_date)
        .collect()
}

async fn refresh_room_date(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    service_date: NaiveDate,
) -> AppResult<()> {
    refresh_projection_chain(
        tx,
        ProjectionInvalidation::new(
            ProjectionNode::HousekeepingDailyWorkloadAggregate,
            ProjectionScope::Inventory,
            ProjectionRefreshTarget::RoomDate {
                date: service_date.to_string(),
            },
        ),
    )
    .await?;

    refresh_projection_chain(
        tx,
        ProjectionInvalidation::new(
            ProjectionNode::DailyRoomClassKpiAggregate,
            ProjectionScope::Inventory,
            ProjectionRefreshTarget::KpiDate {
                date: service_date.to_string(),
            },
        ),
    )
    .await?;

    refresh_projection_chain(
        tx,
        ProjectionInvalidation::new(
            ProjectionNode::DailyHotelKpiAggregate,
            ProjectionScope::Inventory,
            ProjectionRefreshTarget::KpiDate {
                date: service_date.to_string(),
            },
        ),
    )
    .await?;

    refresh_projection_chain(
        tx,
        ProjectionInvalidation::new(
            ProjectionNode::MonthlyRoomClassKpiAggregate,
            ProjectionScope::Inventory,
            ProjectionRefreshTarget::KpiMonth {
                year_month: service_date.format("%Y-%m").to_string(),
            },
        ),
    )
    .await?;

    refresh_projection_chain(
        tx,
        ProjectionInvalidation::new(
            ProjectionNode::MonthlyHotelKpiAggregate,
            ProjectionScope::Inventory,
            ProjectionRefreshTarget::KpiMonth {
                year_month: service_date.format("%Y-%m").to_string(),
            },
        ),
    )
    .await?;

    Ok(())
}
