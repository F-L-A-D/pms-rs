use chrono::NaiveDate;

use rust_decimal::Decimal;

use serde::Serialize;

use uuid::Uuid;

use crate::{
    api::dto::business_date::response::business_date_response::BusinessDateResponse,
    usecase::business_date::night_audit_worklist::{
        NightAuditReservationItem, NightAuditRoomChargeBlocker, NightAuditRoomChargeCandidate,
        NightAuditWorklist,
    },
};

#[derive(Clone, Debug, Serialize)]
pub struct NightAuditWorklistResponse {
    pub business_date: BusinessDateResponse,
    pub unresolved_arrivals: Vec<NightAuditReservationItemResponse>,
    pub unresolved_departures: Vec<NightAuditReservationItemResponse>,
    pub room_charge_candidates: Vec<NightAuditRoomChargeCandidateResponse>,
    pub room_charge_blockers: Vec<NightAuditRoomChargeBlockerResponse>,
}

#[derive(Clone, Debug, Serialize)]
pub struct NightAuditReservationItemResponse {
    pub reservation_id: Uuid,
    pub external_id: Option<String>,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub room_id: Option<Uuid>,
}

#[derive(Clone, Debug, Serialize)]
pub struct NightAuditRoomChargeCandidateResponse {
    pub reservation_id: Uuid,
    pub folio_id: Uuid,
    pub service_date: NaiveDate,
    pub amount: Decimal,
}

#[derive(Clone, Debug, Serialize)]
pub struct NightAuditRoomChargeBlockerResponse {
    pub reservation_id: Uuid,
    pub service_date: NaiveDate,
    pub amount: Decimal,
    pub reason: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct PostNightAuditRoomChargesResponse {
    pub posted_room_charges: Vec<NightAuditRoomChargeCandidateResponse>,
}

impl From<NightAuditWorklist> for NightAuditWorklistResponse {
    fn from(value: NightAuditWorklist) -> Self {
        Self {
            business_date: BusinessDateResponse::from(value.business_date),
            unresolved_arrivals: value
                .unresolved_arrivals
                .into_iter()
                .map(NightAuditReservationItemResponse::from)
                .collect(),
            unresolved_departures: value
                .unresolved_departures
                .into_iter()
                .map(NightAuditReservationItemResponse::from)
                .collect(),
            room_charge_candidates: value
                .room_charge_candidates
                .into_iter()
                .map(NightAuditRoomChargeCandidateResponse::from)
                .collect(),
            room_charge_blockers: value
                .room_charge_blockers
                .into_iter()
                .map(NightAuditRoomChargeBlockerResponse::from)
                .collect(),
        }
    }
}

impl From<NightAuditReservationItem> for NightAuditReservationItemResponse {
    fn from(value: NightAuditReservationItem) -> Self {
        Self {
            reservation_id: value.reservation_id,
            external_id: value.external_id,
            check_in: value.check_in,
            check_out: value.check_out,
            room_id: value.room_id,
        }
    }
}

impl From<NightAuditRoomChargeBlocker> for NightAuditRoomChargeBlockerResponse {
    fn from(value: NightAuditRoomChargeBlocker) -> Self {
        Self {
            reservation_id: value.reservation_id,
            service_date: value.service_date,
            amount: value.amount,
            reason: value.reason.to_snake().to_string(),
        }
    }
}

impl From<NightAuditRoomChargeCandidate> for NightAuditRoomChargeCandidateResponse {
    fn from(value: NightAuditRoomChargeCandidate) -> Self {
        Self {
            reservation_id: value.reservation_id,
            folio_id: value.folio_id,
            service_date: value.service_date,
            amount: value.amount,
        }
    }
}
