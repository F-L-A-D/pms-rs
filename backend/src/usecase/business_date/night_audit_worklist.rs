use chrono::NaiveDate;

use rust_decimal::Decimal;

use sqlx::{Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::{
        entity::{business_date::BusinessDate, folio::FolioStatus, reservation::Reservation},
        semantic::reservation_booking::ReservationRevenueCategory,
    },
    error::app_error::AppResult,
    repository::sqlite::operational::{
        billing::folio_repository::SqliteFolioRepository,
        business_date::night_audit_room_charge_posting_repository::SqliteNightAuditRoomChargePostingRepository,
        reservation::{
            reservation_daily_revenue_allocation_repository::SqliteReservationDailyRevenueAllocationRepository,
            reservation_repository::SqliteReservationRepository,
        },
    },
};

#[derive(Clone, Debug)]
pub struct NightAuditReservationItem {
    pub reservation_id: Uuid,
    pub external_id: Option<String>,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub room_id: Option<Uuid>,
}

#[derive(Clone, Debug)]
pub struct NightAuditRoomChargeCandidate {
    pub reservation_id: Uuid,
    pub folio_id: Uuid,
    pub service_date: NaiveDate,
    pub amount: Decimal,
}

#[derive(Clone, Debug)]
pub struct NightAuditWorklist {
    pub business_date: BusinessDate,
    pub unresolved_arrivals: Vec<NightAuditReservationItem>,
    pub unresolved_departures: Vec<NightAuditReservationItem>,
    pub room_charge_candidates: Vec<NightAuditRoomChargeCandidate>,
}

pub async fn collect_worklist(
    tx: &mut Transaction<'_, Sqlite>,
    business_date: BusinessDate,
) -> AppResult<NightAuditWorklist> {
    let unresolved_arrivals = SqliteReservationRepository::list_unresolved_arrivals_by_date(
        tx,
        business_date.business_date,
    )
    .await?
    .into_iter()
    .map(reservation_item)
    .collect();

    let unresolved_departures = SqliteReservationRepository::list_unresolved_departures_by_date(
        tx,
        business_date.business_date,
    )
    .await?
    .into_iter()
    .map(reservation_item)
    .collect();

    let room_charge_candidates = collect_room_charge_candidates(tx, &business_date).await?;

    Ok(NightAuditWorklist {
        business_date,
        unresolved_arrivals,
        unresolved_departures,
        room_charge_candidates,
    })
}

pub async fn collect_room_charge_candidates(
    tx: &mut Transaction<'_, Sqlite>,
    business_date: &BusinessDate,
) -> AppResult<Vec<NightAuditRoomChargeCandidate>> {
    let reservations =
        SqliteReservationRepository::list_checked_in_by_stay_date(tx, business_date.business_date)
            .await?;

    let mut candidates = vec![];

    for reservation in reservations {
        if SqliteNightAuditRoomChargePostingRepository::find_by_reservation_and_service_date(
            tx,
            reservation.id,
            business_date.business_date,
        )
        .await?
        .is_some()
        {
            continue;
        }

        let Some(folio_id) = open_folio_id(tx, reservation.id).await? else {
            continue;
        };

        let amount = room_revenue_amount(tx, reservation.id, business_date.business_date).await?;

        if amount <= Decimal::ZERO {
            continue;
        }

        candidates.push(NightAuditRoomChargeCandidate {
            reservation_id: reservation.id,
            folio_id,
            service_date: business_date.business_date,
            amount,
        });
    }

    Ok(candidates)
}

pub async fn open_folio_id(
    tx: &mut Transaction<'_, Sqlite>,
    reservation_id: Uuid,
) -> AppResult<Option<Uuid>> {
    let folios = SqliteFolioRepository::list_by_reservation_id(tx, reservation_id).await?;

    Ok(folios
        .into_iter()
        .find(|folio| folio.status == FolioStatus::Open)
        .map(|folio| folio.id))
}

pub async fn room_revenue_amount(
    tx: &mut Transaction<'_, Sqlite>,
    reservation_id: Uuid,
    service_date: NaiveDate,
) -> AppResult<Decimal> {
    let allocations =
        SqliteReservationDailyRevenueAllocationRepository::list_by_reservation_and_service_date(
            tx,
            reservation_id,
            service_date,
        )
        .await?;

    Ok(allocations
        .into_iter()
        .filter(|allocation| allocation.revenue_category == ReservationRevenueCategory::Room)
        .map(|allocation| allocation.amount)
        .sum())
}

fn reservation_item(reservation: Reservation) -> NightAuditReservationItem {
    NightAuditReservationItem {
        reservation_id: reservation.id,
        external_id: reservation.external_id,
        check_in: reservation.check_in,
        check_out: reservation.check_out,
        room_id: reservation.room_id,
    }
}
