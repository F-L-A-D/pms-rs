use std::collections::HashSet;

use rust_decimal::Decimal;

use uuid::Uuid;

use crate::{
    api::dto::input::reservation::CreateReservationInput,
    db::connection::Db,
    domain::{
        entity::reservation::Reservation,
        semantic::{
            guest_timeline_event::TimelineEventType,
            operation_change_event::{ChangedField, OperationChangeEvent, OperationType},
            operation_context::OperationContext,
            reservation_booking::{
                ReservationDailyRevenueAllocation, ReservationDailyStayDetail,
                ReservationPackageBreakdown,
            },
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
        operation_change_event_repository::SqliteOperationChangeEventRepository,
        package_definition_repository::SqlitePackageDefinitionRepository,
        reservation_daily_revenue_allocation_repository::SqliteReservationDailyRevenueAllocationRepository,
        reservation_daily_stay_detail_repository::SqliteReservationDailyStayDetailRepository,
        reservation_guest_relation_repository::SqliteReservationGuestRelationRepository,
        reservation_package_breakdown_repository::SqliteReservationPackageBreakdownRepository,
        reservation_repository::SqliteReservationRepository,
    },
    usecase::timeline::command::record_event::record_event,
};

pub async fn execute(
    db: &Db,
    input: CreateReservationInput,
    context: OperationContext,
) -> AppResult<Reservation> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let daily_inputs = input.daily_details;
        let package_inputs = input.package_breakdowns;
        let participant_inputs = input.participants;

        for participant in &participant_inputs {
            let guest = SqliteGuestRepository::find_by_id(&mut tx, participant.guest_id).await?;

            if guest.is_none() {
                return Err(not_found(format!(
                    "guest not found: {}",
                    participant.guest_id,
                )));
            }
        }

        let reservation_id = Uuid::new_v4();

        let participants = participant_inputs
            .into_iter()
            .map(|p| ReservationGuestRelation {
                reservation_id: reservation_id,

                guest_id: p.guest_id,

                relation_type: p.relation_type,
            })
            .collect();

        let mut reservation = Reservation::new(
            reservation_id,
            input.external_id,
            input.check_in,
            input.check_out,
            input.room_class,
            input.booking_channel,
            input.plan_code,
            package_inputs
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

        resolve_package_catalog_revenue_categories(&mut tx, &mut reservation).await?;

        let daily_stay_details = build_daily_stay_details(&reservation, &daily_inputs)?;
        let daily_revenue_allocations = build_daily_revenue_allocations(
            &reservation,
            &daily_inputs,
            &reservation.package_breakdowns,
        )?;

        reservation.daily_stay_details = daily_stay_details;
        reservation.daily_revenue_allocations = daily_revenue_allocations;

        resolve_package_catalog_revenue_categories(&mut tx, &mut reservation).await?;

        SqliteReservationRepository::save(&mut tx, &reservation).await?;

        for participant in &reservation.participants {
            SqliteReservationGuestRelationRepository::save(&mut tx, participant).await?;
        }

        for breakdown in &reservation.package_breakdowns {
            SqliteReservationPackageBreakdownRepository::save(&mut tx, breakdown).await?;
        }

        for detail in &reservation.daily_stay_details {
            SqliteReservationDailyStayDetailRepository::save(&mut tx, detail).await?;
        }

        for allocation in &reservation.daily_revenue_allocations {
            SqliteReservationDailyRevenueAllocationRepository::save(&mut tx, allocation).await?;
        }

        let change_event = OperationChangeEvent {
            id: Uuid::new_v4(),
            operation_id: context.operation_id,
            aggregate_type: "reservation".to_string(),
            aggregate_id: reservation.id,
            operation_type: OperationType::Create,
            actor: context.actor,
            actor_id: context.actor_id.clone(),
            source: context.source,
            before_json: None,
            after_json: reservation_json(&reservation).to_string(),
            changed_fields_json: serde_json::to_string(&created_changed_fields(&reservation))
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

async fn resolve_package_catalog_revenue_categories(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    reservation: &mut Reservation,
) -> AppResult<()> {
    for breakdown in &mut reservation.package_breakdowns {
        if let Some(package) =
            SqlitePackageDefinitionRepository::find_by_package_code(tx, &breakdown.package_code)
                .await?
        {
            if !package.is_active {
                return Err(validation(format!(
                    "package inactive: {}",
                    breakdown.package_code
                )));
            }

            breakdown.revenue_category = package.revenue_category;
        }
    }

    for allocation in &mut reservation.daily_revenue_allocations {
        if let Some(package) =
            SqlitePackageDefinitionRepository::find_by_package_code(tx, &allocation.package_code)
                .await?
        {
            if !package.is_active {
                return Err(validation(format!(
                    "package inactive: {}",
                    allocation.package_code
                )));
            }

            allocation.revenue_category = package.revenue_category;
            allocation.department_code = Some(package.department_code);
            allocation.account_code = Some(package.account_code);
        }
    }

    Ok(())
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

fn created_changed_fields(reservation: &Reservation) -> Vec<ChangedField> {
    vec![
        ChangedField::new(
            "external_id",
            None,
            reservation
                .external_id
                .as_ref()
                .map(|value| value.to_string()),
        ),
        ChangedField::new("check_in", None, Some(reservation.check_in.to_string())),
        ChangedField::new("check_out", None, Some(reservation.check_out.to_string())),
        ChangedField::new("room_class", None, Some(reservation.room_class.clone())),
        ChangedField::new(
            "booking_channel",
            None,
            Some(reservation.booking_channel.to_snake().to_string()),
        ),
        ChangedField::new("plan_code", None, reservation.plan_code.clone()),
        ChangedField::new(
            "package_breakdowns",
            None,
            Some(reservation.package_breakdowns.len().to_string()),
        ),
        ChangedField::new(
            "daily_details",
            None,
            Some(reservation.daily_stay_details.len().to_string()),
        ),
        ChangedField::new(
            "daily_revenue_allocations",
            None,
            Some(reservation.daily_revenue_allocations.len().to_string()),
        ),
        ChangedField::new(
            "participants",
            None,
            Some(reservation.participants.len().to_string()),
        ),
    ]
}

fn build_daily_stay_details(
    reservation: &Reservation,
    inputs: &[crate::api::dto::input::reservation::ReservationDailyDetailInput],
) -> AppResult<Vec<ReservationDailyStayDetail>> {
    if inputs.is_empty() {
        return Ok(reservation
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
            .collect());
    }

    let nights = reservation.nights();
    let mut seen = HashSet::new();
    let mut details = Vec::new();

    for input in inputs {
        if !nights.contains(&input.service_date) {
            return Err(validation(format!(
                "daily detail service_date out of reservation stay range: {}",
                input.service_date
            )));
        }

        if !seen.insert(input.service_date) {
            return Err(validation(format!(
                "duplicate daily detail service_date: {}",
                input.service_date
            )));
        }

        if input.adult_count < 0 || input.child_count < 0 {
            return Err(validation("daily detail guest counts must be non-negative"));
        }

        details.push(ReservationDailyStayDetail {
            reservation_id: reservation.id,
            service_date: input.service_date,
            room_class: input.room_class.clone(),
            plan_code: input.plan_code.clone(),
            adult_count: input.adult_count,
            child_count: input.child_count,
        });
    }

    if seen.len() != nights.len() {
        return Err(validation(
            "daily details must cover every reservation night",
        ));
    }

    details.sort_by_key(|detail| detail.service_date);

    Ok(details)
}

fn build_daily_revenue_allocations(
    reservation: &Reservation,
    daily_inputs: &[crate::api::dto::input::reservation::ReservationDailyDetailInput],
    package_breakdowns: &[ReservationPackageBreakdown],
) -> AppResult<Vec<ReservationDailyRevenueAllocation>> {
    let has_daily_breakdowns = daily_inputs
        .iter()
        .any(|detail| !detail.package_breakdowns.is_empty());

    if has_daily_breakdowns {
        return Ok(daily_inputs
            .iter()
            .flat_map(|detail| {
                detail.package_breakdowns.iter().map(|breakdown| {
                    ReservationDailyRevenueAllocation {
                        reservation_id: reservation.id,
                        service_date: detail.service_date,
                        package_code: breakdown.package_code.clone(),
                        revenue_category: breakdown.revenue_category,
                        department_code: None,
                        account_code: None,
                        amount: breakdown.amount,
                    }
                })
            })
            .collect());
    }

    let nights = reservation.nights();

    if nights.is_empty() {
        return Ok(vec![]);
    }

    let divisor = Decimal::from(nights.len() as i64);

    Ok(nights
        .into_iter()
        .flat_map(|service_date| {
            package_breakdowns
                .iter()
                .map(move |breakdown| ReservationDailyRevenueAllocation {
                    reservation_id: reservation.id,
                    service_date,
                    package_code: breakdown.package_code.clone(),
                    revenue_category: breakdown.revenue_category,
                    department_code: None,
                    account_code: None,
                    amount: breakdown.amount / divisor,
                })
        })
        .collect())
}
