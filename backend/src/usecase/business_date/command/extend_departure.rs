use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::{
        entity::reservation::{Reservation, ReservationStatus, StayStatus},
        semantic::{
            guest_timeline_event::TimelineEventType,
            operation_change_event::{ChangedField, OperationChangeEvent, OperationType},
            operation_context::OperationContext,
            reservation_booking::{ReservationDailyRevenueAllocation, ReservationDailyStayDetail},
            reservation_transition::{ReservationTransition, ReservationTransitionType},
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
    repository::sqlite::{
        behavioral::reservation_transition_repository::SqliteReservationTransitionRepository,
        operational::{
            operation::operation_change_event_repository::SqliteOperationChangeEventRepository,
            reservation::{
                reservation_daily_revenue_allocation_repository::SqliteReservationDailyRevenueAllocationRepository,
                reservation_daily_stay_detail_repository::SqliteReservationDailyStayDetailRepository,
                reservation_repository::SqliteReservationRepository,
            },
            room::room_daily_state_repository::SqliteRoomDailyStateRepository,
        },
    },
    usecase::{
        audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
        business_date::validation::ensure_active_business_date_closing,
        timeline::command::record_event::record_event,
    },
};

pub async fn execute(
    db: &Db,
    reservation_id: Uuid,
    context: OperationContext,
) -> AppResult<Reservation> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let business_date = ensure_active_business_date_closing(&mut tx).await?;
        let mut reservation = SqliteReservationRepository::find_by_id(&mut tx, reservation_id)
            .await?
            .ok_or(not_found("reservation not found"))?;

        if reservation.reservation_status != ReservationStatus::Confirmed
            || reservation.stay_status != Some(StayStatus::CheckedIn)
        {
            return Err(conflict(
                "only checked-in confirmed stays can be extended during night audit",
            ));
        }

        if reservation.check_out > business_date.business_date {
            return Err(conflict(
                "night audit departure extension must be due by current business date",
            ));
        }

        let room_id = reservation
            .room_id
            .ok_or_else(|| conflict("room not assigned"))?;

        let before = reservation.clone();
        let extension_night = business_date.business_date;
        let next_check_out = business_date.business_date + chrono::Duration::days(1);

        add_extension_daily_detail(&mut tx, &reservation, extension_night).await?;
        add_extension_revenue_allocations(&mut tx, &reservation, extension_night).await?;

        let mut room_state = match SqliteRoomDailyStateRepository::find_by_room_and_service_date(
            &mut tx,
            room_id,
            extension_night,
        )
        .await?
        {
            Some(state) => state,
            None => RoomDailyState::new(room_id, extension_night),
        };

        room_state.set_occupancy_status(RoomDailyOccupancyStatus::Occupied);

        SqliteRoomDailyStateRepository::save(&mut tx, &room_state).await?;

        reservation.check_out = next_check_out;

        SqliteReservationRepository::modify(&mut tx, &mut reservation).await?;

        let before_json = reservation_json(&before).to_string();
        let after_json = reservation_json(&reservation).to_string();
        let changed_fields_json =
            serde_json::to_string(&extend_changed_fields(&before, &reservation)).map_err(infra)?;

        let change_event = OperationChangeEvent {
            id: Uuid::new_v4(),
            operation_id: context.operation_id,
            aggregate_type: "reservation".to_string(),
            aggregate_id: reservation.id,
            operation_type: OperationType::Modify,
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
                action: "night_audit.extend_departure".to_string(),
                before_json: Some(before_json),
                after_json,
                changed_fields_json,
                reason: None,
            },
        )
        .await?;

        SqliteReservationTransitionRepository::save(
            &mut tx,
            &ReservationTransition {
                id: Uuid::new_v4(),
                reservation_id: reservation.id,
                transition_type: ReservationTransitionType::ReservationExtended,
                field_name: "check_out".to_string(),
                before_value: before.check_out.to_string(),
                after_value: reservation.check_out.to_string(),
                occurred_at: chrono::Utc::now(),
            },
        )
        .await?;

        for participant in &reservation.participants {
            record_event(
                &mut tx,
                participant.guest_id,
                TimelineEventType::ReservationExtended,
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

        refresh_inventory_date(&mut tx, extension_night).await?;
        refresh_inventory_date(&mut tx, next_check_out).await?;

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

async fn add_extension_daily_detail(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    reservation: &Reservation,
    service_date: chrono::NaiveDate,
) -> AppResult<()> {
    if reservation
        .daily_stay_details
        .iter()
        .any(|detail| detail.service_date == service_date)
    {
        return Ok(());
    }

    let previous = reservation
        .daily_stay_details
        .iter()
        .rev()
        .find(|detail| detail.service_date < service_date);

    let detail = match previous {
        Some(previous) => ReservationDailyStayDetail {
            reservation_id: reservation.id,
            service_date,
            room_class: previous.room_class.clone(),
            plan_code: previous.plan_code.clone(),
            adult_count: previous.adult_count,
            child_count: previous.child_count,
            sleep_sharing_child_count: previous.sleep_sharing_child_count,
            sleep_sharing_children: vec![],
        },
        None => ReservationDailyStayDetail {
            reservation_id: reservation.id,
            service_date,
            room_class: reservation.room_class.clone(),
            plan_code: reservation.plan_code.clone(),
            adult_count: 1,
            child_count: 0,
            sleep_sharing_child_count: 0,
            sleep_sharing_children: vec![],
        },
    };

    SqliteReservationDailyStayDetailRepository::save(tx, &detail).await
}

async fn add_extension_revenue_allocations(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    reservation: &Reservation,
    service_date: chrono::NaiveDate,
) -> AppResult<()> {
    if reservation
        .daily_revenue_allocations
        .iter()
        .any(|allocation| allocation.service_date == service_date)
    {
        return Ok(());
    }

    let previous_service_date = reservation
        .daily_revenue_allocations
        .iter()
        .filter(|allocation| allocation.service_date < service_date)
        .map(|allocation| allocation.service_date)
        .max();

    let allocations = match previous_service_date {
        Some(previous_service_date) => reservation
            .daily_revenue_allocations
            .iter()
            .filter(|allocation| allocation.service_date == previous_service_date)
            .map(|allocation| ReservationDailyRevenueAllocation {
                reservation_id: reservation.id,
                service_date,
                package_code: allocation.package_code.clone(),
                revenue_category: allocation.revenue_category,
                department_code: allocation.department_code.clone(),
                account_code: allocation.account_code.clone(),
                amount: allocation.amount,
            })
            .collect::<Vec<_>>(),
        None => reservation
            .package_breakdowns
            .iter()
            .map(|breakdown| ReservationDailyRevenueAllocation {
                reservation_id: reservation.id,
                service_date,
                package_code: breakdown.package_code.clone(),
                revenue_category: breakdown.revenue_category,
                department_code: None,
                account_code: None,
                amount: breakdown.amount,
            })
            .collect::<Vec<_>>(),
    };

    for allocation in allocations {
        SqliteReservationDailyRevenueAllocationRepository::save(tx, &allocation).await?;
    }

    Ok(())
}

fn extend_changed_fields(before: &Reservation, after: &Reservation) -> Vec<ChangedField> {
    vec![ChangedField::new(
        "check_out",
        Some(before.check_out.to_string()),
        Some(after.check_out.to_string()),
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
    })
}

async fn refresh_inventory_date(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    service_date: chrono::NaiveDate,
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
