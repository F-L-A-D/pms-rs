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
            operation::operation_change_event_repository::SqliteOperationChangeEventRepository,
            reservation::reservation_repository::SqliteReservationRepository,
            room::{
                room_daily_state_repository::SqliteRoomDailyStateRepository,
                room_repository::SqliteRoomRepository,
            },
        },
    },
    usecase::audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
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

        let before = reservation.clone();

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

        let affected_service_dates = move_dates(&reservation, effective_date);

        for service_date in affected_service_dates.clone() {
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

        let before_json = reservation_json(&before).to_string();
        let after_json = room_move_json(
            &reservation,
            old_room_id,
            new_room_id,
            effective_date,
            &affected_service_dates,
        )
        .to_string();

        let changed_fields_json =
            serde_json::to_string(&room_move_changed_fields(old_room_id, new_room_id))
                .map_err(infra)?;

        let context = OperationContext::api_system();

        let change_event = OperationChangeEvent {
            id: Uuid::new_v4(),
            operation_id: context.operation_id,
            aggregate_type: "reservation".to_string(),
            aggregate_id: reservation.id,
            operation_type: OperationType::RoomMoved,
            actor: context.actor.clone(),
            actor_id: context.actor_id.clone(),
            source: context.source,
            before_json: Some(before_json.clone()),
            after_json: after_json.clone(),
            changed_fields_json: changed_fields_json.clone(),
            occurred_at: chrono::Utc::now(),
        };

        SqliteOperationChangeEventRepository::save(&mut tx, &change_event).await?;

        record_audit_log(
            &mut tx,
            &context,
            RecordAuditLogInput {
                aggregate_type: "reservation".to_string(),
                aggregate_id: reservation.id,
                action: "stay.room_move".to_string(),
                before_json: Some(before_json),
                after_json,
                changed_fields_json,
                reason: None,
            },
        )
        .await?;

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

fn room_move_changed_fields(
    old_room_id: Uuid,
    new_room_id: Uuid,
) -> Vec<ChangedField> {
    vec![ChangedField::new(
        "room_id",
        Some(old_room_id.to_string()),
        Some(new_room_id.to_string()),
    )]
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

fn room_move_json(
    reservation: &Reservation,
    old_room_id: Uuid,
    new_room_id: Uuid,
    effective_date: NaiveDate,
    affected_service_dates: &[NaiveDate],
) -> serde_json::Value {
    serde_json::json!({
        "id": reservation.id,
        "external_id": reservation.external_id,
        "check_in": reservation.check_in,
        "check_out": reservation.check_out,
        "reservation_status": reservation.reservation_status,
        "stay_status": reservation.stay_status,
        "room_class": reservation.room_class,
        "room_id": reservation.room_id,
        "old_room_id": old_room_id,
        "new_room_id": new_room_id,
        "effective_date": effective_date,
        "affected_service_dates": affected_service_dates
            .iter()
            .map(|service_date| service_date.to_string())
            .collect::<Vec<_>>(),
        "booking_channel": reservation.booking_channel,
        "plan_code": reservation.plan_code,
    })
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