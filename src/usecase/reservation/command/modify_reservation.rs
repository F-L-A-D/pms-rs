use uuid::Uuid;

use chrono::NaiveDate;

use rust_decimal::Decimal;

use crate::{
    api::dto::input::reservation::ModifyReservationInput,
    db::connection::Db,
    domain::{
        entity::reservation::Reservation,
        semantic::operation_change_event::{ChangedField, OperationChangeEvent, OperationType},
        semantic::operation_context::OperationContext,
        semantic::reservation_booking::{
            ReservationDailyRevenueAllocation, ReservationDailyStayDetail,
        },
        semantic::reservation_semantics::{
            detect_reservation_timeline_events, detect_reservation_transition_changes,
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
    repository::sqlite::{
        behavioral::reservation_transition_repository::SqliteReservationTransitionRepository,
        operational::{
            operation_change_event_repository::SqliteOperationChangeEventRepository,
            reservation_daily_revenue_allocation_repository::SqliteReservationDailyRevenueAllocationRepository,
            reservation_daily_stay_detail_repository::SqliteReservationDailyStayDetailRepository,
            reservation_repository::SqliteReservationRepository,
        },
    },
    usecase::timeline::command::record_event::record_event,
};

pub async fn execute(
    db: &Db,
    id: Uuid,
    input: ModifyReservationInput,
    context: OperationContext,
) -> AppResult<Reservation> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let mut reservation = SqliteReservationRepository::find_by_id(&mut tx, id)
            .await?
            .ok_or(not_found("reservation not found"))?;

        let before = reservation.clone();

        reservation.check_in = input.check_in.unwrap_or(reservation.check_in);

        reservation.check_out = input.check_out.unwrap_or(reservation.check_out);

        reservation.room_class = input.room_class.unwrap_or(reservation.room_class.clone());

        if reservation.check_in > reservation.check_out {
            return Err(validation("check_in must be <= check_out"));
        }

        let timeline_event_types = detect_reservation_timeline_events(&before, &reservation);
        let transition_changes = detect_reservation_transition_changes(&before, &reservation);

        reservation.daily_stay_details = build_legacy_daily_stay_details(&reservation);
        reservation.daily_revenue_allocations =
            build_legacy_daily_revenue_allocations(&reservation);

        SqliteReservationRepository::modify(&mut tx, &reservation).await?;

        SqliteReservationDailyStayDetailRepository::delete_by_reservation_id(
            &mut tx,
            reservation.id,
        )
        .await?;

        for detail in &reservation.daily_stay_details {
            SqliteReservationDailyStayDetailRepository::save(&mut tx, detail).await?;
        }

        SqliteReservationDailyRevenueAllocationRepository::delete_by_reservation_id(
            &mut tx,
            reservation.id,
        )
        .await?;

        for allocation in &reservation.daily_revenue_allocations {
            SqliteReservationDailyRevenueAllocationRepository::save(&mut tx, allocation).await?;
        }

        let changed_fields = changed_fields(&before, &reservation);
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
            changed_fields_json: serde_json::to_string(&changed_fields).map_err(infra)?,
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

        for change in transition_changes {
            SqliteReservationTransitionRepository::save(
                &mut tx,
                &change.into_transition(reservation.id),
            )
            .await?;
        }

        for participant in &reservation.participants {
            for event_type in &timeline_event_types {
                record_event(
                    &mut tx,
                    participant.guest_id,
                    event_type.clone(),
                    reservation.id,
                )
                .await?;
            }
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

        for service_date in affected_inventory_dates(&before, &reservation) {
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

fn changed_fields(before: &Reservation, after: &Reservation) -> Vec<ChangedField> {
    let mut fields = Vec::new();

    if before.check_in != after.check_in {
        fields.push(ChangedField::new(
            "check_in",
            Some(before.check_in.to_string()),
            Some(after.check_in.to_string()),
        ));
    }
    if before.check_out != after.check_out {
        fields.push(ChangedField::new(
            "check_out",
            Some(before.check_out.to_string()),
            Some(after.check_out.to_string()),
        ));
    }
    if before.room_class != after.room_class {
        fields.push(ChangedField::new(
            "room_class",
            Some(before.room_class.clone()),
            Some(after.room_class.clone()),
        ));
    }
    if before.daily_stay_details != after.daily_stay_details {
        fields.push(ChangedField::new(
            "daily_details",
            Some(before.daily_stay_details.len().to_string()),
            Some(after.daily_stay_details.len().to_string()),
        ));
    }
    if before.daily_revenue_allocations != after.daily_revenue_allocations {
        fields.push(ChangedField::new(
            "daily_revenue_allocations",
            Some(before.daily_revenue_allocations.len().to_string()),
            Some(after.daily_revenue_allocations.len().to_string()),
        ));
    }

    fields
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

fn affected_inventory_dates(before: &Reservation, after: &Reservation) -> Vec<NaiveDate> {
    let mut dates = before.nights();

    for date in after.nights() {
        if !dates.contains(&date) {
            dates.push(date);
        }
    }

    dates
}

fn build_legacy_daily_stay_details(reservation: &Reservation) -> Vec<ReservationDailyStayDetail> {
    reservation
        .nights()
        .into_iter()
        .map(|service_date| ReservationDailyStayDetail {
            reservation_id: reservation.id,
            service_date,
            room_class: reservation.room_class.clone(),
            plan_code: reservation.plan_code.clone(),
            adult_count: 1,
            child_count: 0,
        })
        .collect()
}

fn build_legacy_daily_revenue_allocations(
    reservation: &Reservation,
) -> Vec<ReservationDailyRevenueAllocation> {
    let nights = reservation.nights();

    if nights.is_empty() {
        return vec![];
    }

    let divisor = Decimal::from(nights.len() as i64);

    nights
        .into_iter()
        .flat_map(|service_date| {
            reservation.package_breakdowns.iter().map(move |breakdown| {
                ReservationDailyRevenueAllocation {
                    reservation_id: reservation.id,
                    service_date,
                    package_code: breakdown.package_code.clone(),
                    revenue_category: breakdown.revenue_category,
                    amount: breakdown.amount / divisor,
                }
            })
        })
        .collect()
}
