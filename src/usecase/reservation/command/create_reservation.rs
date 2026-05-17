use uuid::Uuid;

use crate::{
    api::dto::input::reservation::CreateReservationInput,
    db::connection::Db,
    domain::{
        entity::reservation::Reservation,
        semantic::{
            guest_timeline_event::TimelineEventType,
            reservation_booking::ReservationPackageBreakdown,
            reservation_guest_relation::ReservationGuestRelation,
        },
    },
    error::app_error::{infra, not_found, validation, AppResult},
    projection::{
        invalidation::{
            projection_invalidation::{ProjectionInvalidation, ProjectionRefreshTarget},
            projection_scope::ProjectionScope,
        },
        orchestrator::refresh_projection_chain::refresh_projection_chain,
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::operational::{
        guest_repository::SqliteGuestRepository,
        reservation_guest_relation_repository::SqliteReservationGuestRelationRepository,
        reservation_package_breakdown_repository::SqliteReservationPackageBreakdownRepository,
        reservation_repository::SqliteReservationRepository,
    },
    usecase::timeline::command::record_event::record_event,
};

pub async fn execute(db: &Db, input: CreateReservationInput) -> AppResult<Reservation> {
    let mut tx = db.begin_tx().await;

    let result = async {
        for participant in &input.participants {
            let guest = SqliteGuestRepository::find_by_id(&mut tx, participant.guest_id).await?;

            if guest.is_none() {
                return Err(not_found(format!(
                    "guest not found: {}",
                    participant.guest_id,
                )));
            }
        }

        let reservation_id = Uuid::new_v4();

        let participants = input
            .participants
            .into_iter()
            .map(|p| ReservationGuestRelation {
                reservation_id: reservation_id,

                guest_id: p.guest_id,

                relation_type: p.relation_type,
            })
            .collect();

        let reservation = Reservation::new(
            reservation_id,
            input.external_id,
            input.check_in,
            input.check_out,
            input.room_class,
            input.booking_channel,
            input.plan_code,
            input
                .package_breakdowns
                .into_iter()
                .map(|breakdown| ReservationPackageBreakdown {
                    reservation_id,
                    package_code: breakdown.package_code,
                    revenue_category: breakdown.revenue_category,
                    amount: breakdown.amount,
                })
                .collect(),
            participants,
        )
        .map_err(validation)?;

        SqliteReservationRepository::save(&mut tx, &reservation).await?;

        for participant in &reservation.participants {
            SqliteReservationGuestRelationRepository::save(&mut tx, participant).await?;
        }

        for breakdown in &reservation.package_breakdowns {
            SqliteReservationPackageBreakdownRepository::save(&mut tx, breakdown).await?;
        }

        let primary_guest_id = reservation.primary_participant().map(|p| p.guest_id);

        if let Some(guest_id) = primary_guest_id {
            record_event(
                &mut tx,
                guest_id,
                TimelineEventType::ReservationCreated,
                reservation_id,
            )
            .await?;
        }

        for participant in &reservation.participants {
            refresh_projection_chain(
                &mut tx,
                ProjectionInvalidation::new(
                    ProjectionNode::GuestAggregate,
                    ProjectionScope::Guest,
                    ProjectionRefreshTarget::Guest {
                        guest_id: participant.guest_id,
                    },
                ),
            )
            .await?;
        }

        for service_date in reservation.nights() {
            refresh_projection_chain(
                &mut tx,
                ProjectionInvalidation::new(
                    ProjectionNode::InventoryAggregate,
                    ProjectionScope::Inventory,
                    ProjectionRefreshTarget::InventoryDate {
                        date: service_date.to_string(),
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
                        date: service_date.to_string(),
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
                        date: service_date.to_string(),
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
                        year_month: service_date.format("%Y-%m").to_string(),
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
                        year_month: service_date.format("%Y-%m").to_string(),
                    },
                ),
            )
            .await?;
        }

        Ok(reservation)
    }
    .await;

    match result {
        Ok(reservation) => {
            tx.commit().await.map_err(infra)?;

            Ok(reservation)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}
