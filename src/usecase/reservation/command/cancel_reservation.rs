use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::{
        entity::reservation::{Reservation, ReservationStatus},
        semantic::{
            guest_timeline_event::TimelineEventType,
            operation_change_event::{OperationChangeEvent, OperationType},
            operation_context::OperationContext,
        },
    },
    error::app_error::{infra, not_found, AppResult},
    projection::{
        invalidation::{
            projection_invalidation::{ProjectionInvalidation, ProjectionRefreshTarget},
            projection_scope::ProjectionScope,
        },
        orchestrator::refresh_projection_chain::refresh_projection_chain,
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::operational::{
        operation_change_event_repository::SqliteOperationChangeEventRepository,
        reservation_repository::SqliteReservationRepository,
    },
    usecase::timeline::command::record_event::record_event,
};

pub async fn cancel_reservation(
    db: &Db,
    id: Uuid,
    context: OperationContext,
) -> AppResult<Reservation> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut reservation = SqliteReservationRepository::find_by_id(&mut tx, id)
            .await?
            .ok_or(not_found("reservation not found"))?;

        let before = reservation.clone();

        if reservation.reservation_status == ReservationStatus::Cancelled {
            return Ok(reservation);
        }

        reservation.reservation_status = ReservationStatus::Cancelled;

        reservation.stay_status = None;

        let primary_guest_id = reservation.primary_participant().map(|p| p.guest_id);

        SqliteReservationRepository::modify(&mut tx, &reservation).await?;

        let change_event = OperationChangeEvent {
            id: Uuid::new_v4(),
            operation_id: context.operation_id,
            aggregate_type: "reservation".to_string(),
            aggregate_id: reservation.id,
            operation_type: OperationType::Cancel,
            actor: context.actor,
            actor_id: context.actor_id.clone(),
            source: context.source,
            before_json: Some(reservation_json(&before).to_string()),
            after_json: reservation_json(&reservation).to_string(),
            changed_fields_json: serde_json::json!(["reservation_status", "stay_status"])
                .to_string(),
            occurred_at: chrono::Utc::now(),
        };

        SqliteOperationChangeEventRepository::save(&mut tx, &change_event).await?;

        refresh_projection_chain(
            &mut tx,
            ProjectionInvalidation::new(
                ProjectionNode::ChangePattern,
                ProjectionScope::Timeline,
                ProjectionRefreshTarget::OperationEvent {
                    event_id: change_event.id,
                },
            ),
        )
        .await?;

        if let Some(guest_id) = primary_guest_id {
            record_event(
                &mut tx,
                guest_id,
                TimelineEventType::ReservationCancelled,
                reservation.id,
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

fn reservation_json(reservation: &Reservation) -> serde_json::Value {
    serde_json::json!({
        "id": reservation.id,
        "external_id": reservation.external_id,
        "check_in": reservation.check_in,
        "check_out": reservation.check_out,
        "reservation_status": reservation.reservation_status,
        "stay_status": reservation.stay_status,
        "room_class": reservation.room_class,
        "room_id": reservation.room_id,
        "booking_channel": reservation.booking_channel,
        "plan_code": reservation.plan_code,
    })
}
