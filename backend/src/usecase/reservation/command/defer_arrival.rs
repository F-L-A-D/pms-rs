use chrono::Utc;

use uuid::Uuid;

use crate::{
    api::dto::input::reservation::DeferArrivalInput,
    db::connection::Db,
    domain::{
        entity::{
            folio_entry::{FolioEntry, FolioEntryType},
            night_audit_room_charge_posting::NightAuditRoomChargePosting,
            reservation::{Reservation, ReservationStatus, StayStatus},
        },
        semantic::{
            operation_change_event::{ChangedField, OperationChangeEvent, OperationType},
            operation_context::OperationContext,
            reservation_transition::{ReservationTransition, ReservationTransitionType},
        },
    },
    error::app_error::{conflict, infra, not_found, AppResult},
    repository::sqlite::{
        behavioral::reservation_transition_repository::SqliteReservationTransitionRepository,
        operational::{
            billing::folio_entry_repository::SqliteFolioEntryRepository,
            business_date::night_audit_room_charge_posting_repository::SqliteNightAuditRoomChargePostingRepository,
            operation::operation_change_event_repository::SqliteOperationChangeEventRepository,
            reservation::{
                reservation_daily_revenue_allocation_repository::SqliteReservationDailyRevenueAllocationRepository,
                reservation_daily_stay_detail_repository::SqliteReservationDailyStayDetailRepository,
                reservation_repository::SqliteReservationRepository,
            },
        },
    },
    usecase::{
        audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
        business_date::{
            night_audit_worklist::{open_folio_id, room_revenue_amount},
            validation::ensure_active_business_date_closing,
        },
    },
};

pub async fn execute(
    db: &Db,
    input: DeferArrivalInput,
    context: OperationContext,
) -> AppResult<Reservation> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let business_date = ensure_active_business_date_closing(&mut tx).await?;
        let next_business_date = business_date.business_date.succ_opt().unwrap();

        let mut reservation =
            SqliteReservationRepository::find_by_id(&mut tx, input.reservation_id)
                .await?
                .ok_or_else(|| not_found("reservation not found"))?;

        let before = reservation.clone();

        if reservation.reservation_status != ReservationStatus::Confirmed
            || reservation.stay_status != Some(StayStatus::Confirmed)
        {
            return Err(conflict("only confirmed arrivals can be deferred"));
        }

        if reservation.check_in != business_date.business_date {
            return Err(conflict(
                "reservation check-in must match closing business date",
            ));
        }

        if reservation.check_out <= next_business_date {
            return Err(conflict(
                "arrival can only be deferred for multi-night stays",
            ));
        }

        if input.post_room_charge {
            post_retained_room_charge(&mut tx, &business_date, &reservation).await?;
        }

        reservation.check_in = next_business_date;
        reservation.daily_stay_details.retain(|detail| {
            detail.service_date >= next_business_date && detail.service_date < reservation.check_out
        });
        reservation.daily_revenue_allocations.retain(|allocation| {
            allocation.service_date >= next_business_date
                && allocation.service_date < reservation.check_out
        });

        SqliteReservationRepository::modify(&mut tx, &mut reservation).await?;

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

        record_defer_arrival_audit(&mut tx, &context, &before, &reservation).await?;

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

async fn post_retained_room_charge(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    business_date: &crate::domain::entity::business_date::BusinessDate,
    reservation: &Reservation,
) -> AppResult<()> {
    if SqliteNightAuditRoomChargePostingRepository::find_by_reservation_and_service_date(
        tx,
        reservation.id,
        business_date.business_date,
    )
    .await?
    .is_some()
    {
        return Ok(());
    }

    let folio_id = open_folio_id(tx, reservation.id)
        .await?
        .ok_or_else(|| conflict("open folio not found"))?;
    let amount = room_revenue_amount(tx, reservation.id, business_date.business_date).await?;

    if amount <= rust_decimal::Decimal::ZERO {
        return Ok(());
    }

    let now = Utc::now();
    let folio_entry = FolioEntry {
        id: Uuid::new_v4(),
        folio_id,
        entry_type: FolioEntryType::RoomCharge,
        amount,
        occurred_at: now,
        memo: Some(format!(
            "Night audit retained room charge for deferred arrival {}",
            business_date.business_date
        )),
    };

    SqliteFolioEntryRepository::save(tx, &folio_entry).await?;

    let posting = NightAuditRoomChargePosting {
        id: Uuid::new_v4(),
        business_date_id: business_date.id,
        business_date: business_date.business_date,
        reservation_id: reservation.id,
        folio_id,
        service_date: business_date.business_date,
        amount,
        folio_entry_id: folio_entry.id,
        posted_at: now,
    };

    SqliteNightAuditRoomChargePostingRepository::save(tx, &posting).await
}

async fn record_defer_arrival_audit(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    context: &OperationContext,
    before: &Reservation,
    after: &Reservation,
) -> AppResult<()> {
    let changed_fields_json = serde_json::to_string(&vec![ChangedField::new(
        "check_in",
        Some(before.check_in.to_string()),
        Some(after.check_in.to_string()),
    )])
    .map_err(infra)?;

    let before_json = reservation_json(before).to_string();
    let after_json = reservation_json(after).to_string();

    let change_event = OperationChangeEvent {
        id: Uuid::new_v4(),
        operation_id: context.operation_id,
        aggregate_type: "reservation".to_string(),
        aggregate_id: after.id,
        operation_type: OperationType::Modify,
        actor: context.actor.clone(),
        actor_id: context.actor_id.clone(),
        source: context.source,
        before_json: Some(before_json.clone()),
        after_json: after_json.clone(),
        changed_fields_json: changed_fields_json.clone(),
        occurred_at: Utc::now(),
    };

    SqliteOperationChangeEventRepository::save(tx, &change_event).await?;

    record_audit_log(
        tx,
        context,
        RecordAuditLogInput {
            aggregate_type: "reservation".to_string(),
            aggregate_id: after.id,
            action: "reservation.defer_arrival".to_string(),
            before_json: Some(before_json),
            after_json,
            changed_fields_json,
            reason: None,
        },
    )
    .await?;

    SqliteReservationTransitionRepository::save(
        tx,
        &ReservationTransition {
            id: Uuid::new_v4(),
            reservation_id: after.id,
            transition_type: ReservationTransitionType::CheckInChanged,
            field_name: "check_in".to_string(),
            before_value: before.check_in.to_string(),
            after_value: after.check_in.to_string(),
            occurred_at: Utc::now(),
        },
    )
    .await
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
