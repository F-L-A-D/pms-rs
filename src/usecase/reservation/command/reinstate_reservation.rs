use chrono::NaiveDate;

use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::{
        entity::reservation::{Reservation, ReservationStatus, StayStatus},
        semantic::{
            guest_timeline_event::TimelineEventType,
            operation_change_event::{ChangedField, OperationChangeEvent, OperationType},
            operation_context::OperationContext,
            reservation_transition::{ReservationTransition, ReservationTransitionType},
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
    repository::sqlite::{
        behavioral::reservation_transition_repository::SqliteReservationTransitionRepository,
        operational::{
            operation_change_event_repository::SqliteOperationChangeEventRepository,
            reservation_repository::SqliteReservationRepository,
        },
    },
    usecase::timeline::command::record_event::record_event,
};

pub async fn execute(db: &Db, id: Uuid, context: OperationContext) -> AppResult<Reservation> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut reservation = SqliteReservationRepository::find_by_id(&mut tx, id)
            .await?
            .ok_or(not_found("reservation not found"))?;

        let before = reservation.clone();

        if reservation.reservation_status == ReservationStatus::Confirmed {
            return Ok(reservation);
        }

        if !matches!(
            reservation.reservation_status,
            ReservationStatus::NoShow | ReservationStatus::Cancelled
        ) {
            return Err(conflict(
                "only no-show or cancelled reservations can be reinstated",
            ));
        }

        reservation.reservation_status = ReservationStatus::Confirmed;
        reservation.stay_status = Some(StayStatus::Confirmed);

        SqliteReservationRepository::modify(&mut tx, &reservation).await?;

        let change_event = OperationChangeEvent {
            id: Uuid::new_v4(),
            operation_id: context.operation_id,
            aggregate_type: "reservation".to_string(),
            aggregate_id: reservation.id,
            operation_type: OperationType::Modify,
            actor: context.actor,
            actor_id: context.actor_id.clone(),
            source: context.source,
            before_json: Some(reservation_json(&before).to_string()),
            after_json: reservation_json(&reservation).to_string(),
            changed_fields_json: serde_json::to_string(&status_changed_fields(
                &before,
                &reservation,
            ))
            .map_err(infra)?,
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

        SqliteReservationTransitionRepository::save(
            &mut tx,
            &ReservationTransition {
                id: Uuid::new_v4(),
                reservation_id: reservation.id,
                transition_type: ReservationTransitionType::ReservationReinstated,
                field_name: "reservation_status".to_string(),
                before_value: before.reservation_status.to_snake().to_string(),
                after_value: reservation.reservation_status.to_snake().to_string(),
                occurred_at: chrono::Utc::now(),
            },
        )
        .await?;

        for participant in &reservation.participants {
            record_event(
                &mut tx,
                participant.guest_id,
                TimelineEventType::ReservationReinstated,
                reservation.id,
            )
            .await?;

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

        refresh_inventory_and_kpis(&mut tx, &reservation).await?;

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

fn status_changed_fields(before: &Reservation, after: &Reservation) -> Vec<ChangedField> {
    vec![
        ChangedField::new(
            "reservation_status",
            Some(before.reservation_status.to_snake().to_string()),
            Some(after.reservation_status.to_snake().to_string()),
        ),
        ChangedField::new(
            "stay_status",
            before
                .stay_status
                .as_ref()
                .map(|status| status.to_snake().to_string()),
            after
                .stay_status
                .as_ref()
                .map(|status| status.to_snake().to_string()),
        ),
    ]
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

async fn refresh_inventory_and_kpis(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    reservation: &Reservation,
) -> AppResult<()> {
    for service_date in reservation.nights() {
        refresh_inventory_date(tx, service_date).await?;
    }

    Ok(())
}

async fn refresh_inventory_date(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    service_date: NaiveDate,
) -> AppResult<()> {
    refresh_projection_chain(
        tx,
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
