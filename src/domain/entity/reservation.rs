use chrono::{DateTime, NaiveDate, Utc};

use serde::{Deserialize, Serialize};

use std::collections::HashSet;

use uuid::Uuid;

use crate::domain::semantic::reservation_guest_relation::{
    ReservationGuestRelation, ReservationGuestRelationType,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ReservationStatus {
    Pending,
    Confirmed,
    Cancelled,
    NoShow,
    Completed,
}

impl ReservationStatus {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Pending => "pending",

            Self::Confirmed => "confirmed",

            Self::Cancelled => "cancelled",

            Self::NoShow => "no_show",

            Self::Completed => "completed",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "pending" => Some(Self::Pending),

            "confirmed" => Some(Self::Confirmed),

            "cancelled" => Some(Self::Cancelled),

            "no_show" => Some(Self::NoShow),

            "completed" => Some(Self::Completed),

            _ => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum StayStatus {
    Confirmed,
    CheckedIn,
    CheckedOut,
    NoShow,
}

impl StayStatus {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Confirmed => "confirmed",

            Self::CheckedIn => "checked_in",

            Self::CheckedOut => "checked_out",

            Self::NoShow => "no_show",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "confirmed" => Some(Self::Confirmed),

            "checked_in" => Some(Self::CheckedIn),

            "checked_out" => Some(Self::CheckedOut),

            "no_show" => Some(Self::NoShow),

            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Reservation {
    pub id: Uuid,
    pub external_id: Option<String>,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub reservation_status: ReservationStatus,
    pub stay_status: Option<StayStatus>,
    pub room_class: String,
    pub room_id: Option<Uuid>,
    pub participants: Vec<ReservationGuestRelation>,
    pub created_at: DateTime<Utc>,
}

impl Reservation {
    pub fn new(
        id: Uuid,
        external_id: Option<String>,
        check_in: NaiveDate,
        check_out: NaiveDate,
        room_class: String,
        participants: Vec<ReservationGuestRelation>,
    ) -> Result<Self, String> {
        if check_in > check_out {
            return Err("check_in must be before or equal to check_out".into());
        }

        if participants.is_empty() {
            return Err("at least one participant required".into());
        }

        let primary_count = participants
            .iter()
            .filter(|p| p.relation_type == ReservationGuestRelationType::Primary)
            .count();

        if primary_count != 1 {
            return Err("exactly one primary participant required".into());
        }

        let mut seen = HashSet::new();

        for participant in &participants {
            if !seen.insert(&participant.guest_id) {
                return Err("duplicate participant guest_id".into());
            }
        }

        let now = Utc::now();

        Ok(Self {
            id,
            external_id,
            check_in,
            check_out,
            reservation_status: ReservationStatus::Confirmed,
            stay_status: Some(StayStatus::Confirmed),
            room_class,
            room_id: None,
            participants,
            created_at: now,
        })
    }

    pub fn nights(&self) -> Vec<NaiveDate> {
        let mut dates = vec![];
        let mut current = self.check_in;

        while current < self.check_out {
            dates.push(current);
            current = current.succ_opt().unwrap();
        }
        dates
    }

    pub fn primary_participant(&self) -> Option<&ReservationGuestRelation> {
        self.participants.iter().find(|p| p.is_primary())
    }

    pub fn is_active(&self) -> bool {
        self.reservation_status == ReservationStatus::Confirmed
    }

    pub fn is_checked_in(&self) -> bool {
        self.stay_status == Some(StayStatus::CheckedIn)
    }

    pub fn contains_guest(&self, guest_id: Uuid) -> bool {
        self.participants.iter().any(|p| p.guest_id == guest_id)
    }

    pub fn overlaps(&self, from: NaiveDate, to: NaiveDate) -> bool {
        self.check_in < to && self.check_out > from
    }
}
