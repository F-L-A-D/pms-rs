use uuid::Uuid;

use chrono::NaiveDate;

use rust_decimal::Decimal;

use std::collections::{BTreeMap, HashSet};

use crate::{
    api::dto::input::reservation::{
        ModifyReservationInput, ReservationDailyDetailInput,
        ReservationDailyRevenueAllocationInput, ReservationPackageBreakdownInput,
        ReservationParticipantInput,
    },
    db::connection::Db,
    domain::{
        entity::reservation::Reservation,
        semantic::operation_change_event::{ChangedField, OperationChangeEvent, OperationType},
        semantic::operation_context::OperationContext,
        semantic::reservation_booking::{
            ReservationDailyRevenueAllocation, ReservationDailyStayDetail,
            ReservationPackageBreakdown,
        },
        semantic::reservation_guest_relation::{
            ReservationGuestRelation, ReservationGuestRelationType,
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
            guest_repository::SqliteGuestRepository,
            operation_change_event_repository::SqliteOperationChangeEventRepository,
            package_definition_repository::SqlitePackageDefinitionRepository,
            reservation_daily_revenue_allocation_repository::SqliteReservationDailyRevenueAllocationRepository,
            reservation_daily_stay_detail_repository::SqliteReservationDailyStayDetailRepository,
            reservation_guest_relation_repository::SqliteReservationGuestRelationRepository,
            reservation_package_breakdown_repository::SqliteReservationPackageBreakdownRepository,
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
        let stay_shape_changed =
            input.check_in.is_some() || input.check_out.is_some() || input.room_class.is_some();
        let package_breakdowns_changed = input.package_breakdowns.is_some();

        reservation.check_in = input.check_in.unwrap_or(reservation.check_in);

        reservation.check_out = input.check_out.unwrap_or(reservation.check_out);

        reservation.room_class = input.room_class.unwrap_or(reservation.room_class.clone());

        if reservation.check_in > reservation.check_out {
            return Err(validation("check_in must be <= check_out"));
        }

        if let Some(package_inputs) = input.package_breakdowns {
            reservation.package_breakdowns =
                build_package_breakdowns(reservation.id, package_inputs);
        }

        let timeline_event_types = detect_reservation_timeline_events(&before, &reservation);
        let transition_changes = detect_reservation_transition_changes(&before, &reservation);

        let daily_inputs = input.daily_details;
        let allocation_inputs = input.daily_revenue_allocations;

        if let Some(daily_inputs) = daily_inputs {
            reservation.daily_stay_details = build_daily_stay_details(&reservation, &daily_inputs)?;
            reservation.daily_revenue_allocations = build_daily_revenue_allocations(
                &reservation,
                &daily_inputs,
                &reservation.package_breakdowns,
            )?;
        } else if stay_shape_changed {
            reservation.daily_stay_details = build_legacy_daily_stay_details(&reservation);
            reservation.daily_revenue_allocations =
                build_legacy_daily_revenue_allocations(&reservation);
        } else if package_breakdowns_changed {
            reservation.daily_revenue_allocations =
                build_legacy_daily_revenue_allocations(&reservation);
        }

        if let Some(allocation_inputs) = allocation_inputs {
            reservation.daily_revenue_allocations =
                build_explicit_daily_revenue_allocations(&reservation, allocation_inputs)?;
        }

        resolve_package_catalog_revenue_categories(&mut tx, &mut reservation).await?;

        if let Some(participant_inputs) = input.participants {
            reservation.participants =
                build_participants(&mut tx, reservation.id, participant_inputs).await?;
        }

        SqliteReservationRepository::modify(&mut tx, &reservation).await?;

        SqliteReservationPackageBreakdownRepository::delete_by_reservation_id(
            &mut tx,
            reservation.id,
        )
        .await?;

        for breakdown in &reservation.package_breakdowns {
            SqliteReservationPackageBreakdownRepository::save(&mut tx, breakdown).await?;
        }

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

        SqliteReservationGuestRelationRepository::delete_by_reservation_id(&mut tx, reservation.id)
            .await?;

        for participant in &reservation.participants {
            SqliteReservationGuestRelationRepository::save(&mut tx, participant).await?;
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

        for guest_id in affected_guest_ids(&before, &reservation) {
            refresh_projection_chain(
                &mut tx,
                ProjectionInvalidation::new(
                    ProjectionNode::GuestAggregate,
                    ProjectionScope::Guest,
                    ProjectionRefreshTarget::Guest { guest_id },
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
    if before.package_breakdowns != after.package_breakdowns {
        fields.push(ChangedField::new(
            "package_breakdowns",
            Some(before.package_breakdowns.len().to_string()),
            Some(after.package_breakdowns.len().to_string()),
        ));
        fields.extend(package_breakdown_changed_fields(before, after));
    }
    if before.daily_stay_details != after.daily_stay_details {
        fields.push(ChangedField::new(
            "daily_details",
            Some(before.daily_stay_details.len().to_string()),
            Some(after.daily_stay_details.len().to_string()),
        ));
        fields.extend(daily_stay_detail_changed_fields(before, after));
    }
    if before.daily_revenue_allocations != after.daily_revenue_allocations {
        fields.push(ChangedField::new(
            "daily_revenue_allocations",
            Some(before.daily_revenue_allocations.len().to_string()),
            Some(after.daily_revenue_allocations.len().to_string()),
        ));
        fields.extend(daily_revenue_allocation_changed_fields(before, after));
    }
    if before.participants != after.participants {
        fields.push(ChangedField::new(
            "participants",
            Some(before.participants.len().to_string()),
            Some(after.participants.len().to_string()),
        ));
        fields.extend(participant_changed_fields(before, after));
    }

    fields
}

fn participant_changed_fields(before: &Reservation, after: &Reservation) -> Vec<ChangedField> {
    let before_map = before
        .participants
        .iter()
        .map(|participant| {
            (
                participant.guest_id.to_string(),
                participant.relation_type.to_snake().to_string(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let after_map = after
        .participants
        .iter()
        .map(|participant| {
            (
                participant.guest_id.to_string(),
                participant.relation_type.to_snake().to_string(),
            )
        })
        .collect::<BTreeMap<_, _>>();

    changed_map_fields("participants", &before_map, &after_map)
}

fn package_breakdown_changed_fields(
    before: &Reservation,
    after: &Reservation,
) -> Vec<ChangedField> {
    let before_map = before
        .package_breakdowns
        .iter()
        .map(|breakdown| {
            (
                format!(
                    "{}.{}",
                    breakdown.package_code,
                    breakdown.revenue_category.to_snake()
                ),
                breakdown.amount.to_string(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let after_map = after
        .package_breakdowns
        .iter()
        .map(|breakdown| {
            (
                format!(
                    "{}.{}",
                    breakdown.package_code,
                    breakdown.revenue_category.to_snake()
                ),
                breakdown.amount.to_string(),
            )
        })
        .collect::<BTreeMap<_, _>>();

    changed_map_fields("package_breakdowns", &before_map, &after_map)
}

fn daily_stay_detail_changed_fields(
    before: &Reservation,
    after: &Reservation,
) -> Vec<ChangedField> {
    let before_map = before
        .daily_stay_details
        .iter()
        .flat_map(|detail| {
            let base = format!("daily_details.{}", detail.service_date);

            [
                (format!("{base}.room_class"), detail.room_class.clone()),
                (
                    format!("{base}.plan_code"),
                    detail.plan_code.clone().unwrap_or_default(),
                ),
                (
                    format!("{base}.adult_count"),
                    detail.adult_count.to_string(),
                ),
                (
                    format!("{base}.child_count"),
                    detail.child_count.to_string(),
                ),
            ]
        })
        .collect::<BTreeMap<_, _>>();
    let after_map = after
        .daily_stay_details
        .iter()
        .flat_map(|detail| {
            let base = format!("daily_details.{}", detail.service_date);

            [
                (format!("{base}.room_class"), detail.room_class.clone()),
                (
                    format!("{base}.plan_code"),
                    detail.plan_code.clone().unwrap_or_default(),
                ),
                (
                    format!("{base}.adult_count"),
                    detail.adult_count.to_string(),
                ),
                (
                    format!("{base}.child_count"),
                    detail.child_count.to_string(),
                ),
            ]
        })
        .collect::<BTreeMap<_, _>>();

    changed_map_fields("", &before_map, &after_map)
}

fn daily_revenue_allocation_changed_fields(
    before: &Reservation,
    after: &Reservation,
) -> Vec<ChangedField> {
    let before_map = before
        .daily_revenue_allocations
        .iter()
        .map(|allocation| {
            (
                format!(
                    "{}.{}.{}.amount",
                    allocation.service_date,
                    allocation.package_code,
                    allocation.revenue_category.to_snake()
                ),
                allocation.amount.to_string(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let after_map = after
        .daily_revenue_allocations
        .iter()
        .map(|allocation| {
            (
                format!(
                    "{}.{}.{}.amount",
                    allocation.service_date,
                    allocation.package_code,
                    allocation.revenue_category.to_snake()
                ),
                allocation.amount.to_string(),
            )
        })
        .collect::<BTreeMap<_, _>>();

    changed_map_fields("daily_revenue_allocations", &before_map, &after_map)
}

fn changed_map_fields(
    prefix: &str,
    before: &BTreeMap<String, String>,
    after: &BTreeMap<String, String>,
) -> Vec<ChangedField> {
    let mut keys = before.keys().chain(after.keys()).collect::<Vec<_>>();
    keys.sort();
    keys.dedup();

    keys.into_iter()
        .filter_map(|key| {
            let before_value = before.get(key);
            let after_value = after.get(key);

            if before_value == after_value {
                return None;
            }

            let field_name = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{prefix}.{key}")
            };

            Some(ChangedField::new(
                field_name,
                before_value.cloned(),
                after_value.cloned(),
            ))
        })
        .collect()
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

fn affected_guest_ids(before: &Reservation, after: &Reservation) -> Vec<Uuid> {
    let mut guest_ids = before
        .participants
        .iter()
        .map(|participant| participant.guest_id)
        .collect::<Vec<_>>();

    for participant in &after.participants {
        if !guest_ids.contains(&participant.guest_id) {
            guest_ids.push(participant.guest_id);
        }
    }

    guest_ids
}

async fn build_participants(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    reservation_id: Uuid,
    inputs: Vec<ReservationParticipantInput>,
) -> AppResult<Vec<ReservationGuestRelation>> {
    if inputs.is_empty() {
        return Err(validation("at least one participant required"));
    }

    let primary_count = inputs
        .iter()
        .filter(|participant| participant.relation_type == ReservationGuestRelationType::Primary)
        .count();

    if primary_count != 1 {
        return Err(validation("exactly one primary participant required"));
    }

    let mut seen = HashSet::new();
    let mut participants = Vec::new();

    for input in inputs {
        if !seen.insert(input.guest_id) {
            return Err(validation("duplicate participant guest_id"));
        }

        let guest = SqliteGuestRepository::find_by_id(tx, input.guest_id).await?;

        if guest.is_none() {
            return Err(not_found(format!("guest not found: {}", input.guest_id)));
        }

        participants.push(ReservationGuestRelation {
            reservation_id,
            guest_id: input.guest_id,
            relation_type: input.relation_type,
        });
    }

    Ok(participants)
}

fn build_package_breakdowns(
    reservation_id: Uuid,
    inputs: Vec<ReservationPackageBreakdownInput>,
) -> Vec<ReservationPackageBreakdown> {
    inputs
        .into_iter()
        .map(|breakdown| ReservationPackageBreakdown {
            reservation_id,
            package_code: breakdown.package_code,
            revenue_category: breakdown.revenue_category,
            amount: breakdown.amount,
        })
        .collect()
}

fn build_daily_stay_details(
    reservation: &Reservation,
    inputs: &[ReservationDailyDetailInput],
) -> AppResult<Vec<ReservationDailyStayDetail>> {
    if inputs.is_empty() {
        return Ok(build_legacy_daily_stay_details(reservation));
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
    daily_inputs: &[ReservationDailyDetailInput],
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

fn build_explicit_daily_revenue_allocations(
    reservation: &Reservation,
    inputs: Vec<ReservationDailyRevenueAllocationInput>,
) -> AppResult<Vec<ReservationDailyRevenueAllocation>> {
    let nights = reservation.nights();

    inputs
        .into_iter()
        .map(|allocation| {
            if !nights.contains(&allocation.service_date) {
                return Err(validation(format!(
                    "daily revenue allocation service_date out of reservation stay range: {}",
                    allocation.service_date
                )));
            }

            Ok(ReservationDailyRevenueAllocation {
                reservation_id: reservation.id,
                service_date: allocation.service_date,
                package_code: allocation.package_code,
                revenue_category: allocation.revenue_category,
                department_code: allocation.department_code,
                account_code: allocation.account_code,
                amount: allocation.amount,
            })
        })
        .collect()
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
                    department_code: None,
                    account_code: None,
                    amount: breakdown.amount / divisor,
                }
            })
        })
        .collect()
}
