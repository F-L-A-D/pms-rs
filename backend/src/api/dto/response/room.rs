use chrono::NaiveDate;

use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::{
    api::dto::response::housekeeping::RoomDailyStateResponse,
    domain::{
        entity::{
            reservation::{Reservation, ReservationStatus, StayStatus},
            room::Room,
        },
        semantic::room_daily_state::RoomDailyState,
    },
};

#[derive(Debug, Deserialize, Serialize)]
pub struct RoomResponse {
    pub id: Uuid,
    pub room_no: String,
    pub room_class: String,
    pub capacity: Option<u32>,
    pub area_sqm: Decimal,
    pub is_physical: bool,
    pub is_active: bool,
}

impl From<Room> for RoomResponse {
    fn from(room: Room) -> Self {
        Self {
            id: room.id,

            room_no: room.room_no,

            room_class: room.room_class,

            capacity: room.capacity,

            area_sqm: room.area_sqm,

            is_physical: room.is_physical,

            is_active: room.is_active,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RoomAssignmentVisibilityResponse {
    pub assignment_status: String,
    pub reservation_id: Option<Uuid>,
    pub external_id: Option<String>,
    pub reservation_status: Option<String>,
    pub stay_status: Option<String>,
    pub check_in: Option<NaiveDate>,
    pub check_out: Option<NaiveDate>,
    pub warning: Option<String>,
}

impl RoomAssignmentVisibilityResponse {
    pub fn unassigned() -> Self {
        Self {
            assignment_status: "unassigned".to_string(),

            reservation_id: None,

            external_id: None,

            reservation_status: None,

            stay_status: None,

            check_in: None,

            check_out: None,

            warning: None,
        }
    }

    pub fn assigned(reservation: Reservation) -> Self {
        Self {
            assignment_status: "assigned".to_string(),

            reservation_id: Some(reservation.id),

            external_id: reservation.external_id,

            reservation_status: Some(
                reservation
                    .reservation_status
                    .to_snake()
                    .to_string(),
            ),

            stay_status: reservation
                .stay_status
                .map(|status| status.to_snake().to_string()),

            check_in: Some(reservation.check_in),

            check_out: Some(reservation.check_out),

            warning: None,
        }
    }

    pub fn no_show_warning(reservation: Reservation) -> Self {
        Self {
            assignment_status: "unassigned".to_string(),

            reservation_id: Some(reservation.id),

            external_id: reservation.external_id,

            reservation_status: Some(
                reservation
                    .reservation_status
                    .to_snake()
                    .to_string(),
            ),

            stay_status: reservation
                .stay_status
                .map(|status| status.to_snake().to_string()),

            check_in: Some(reservation.check_in),

            check_out: Some(reservation.check_out),

            warning: Some(
                "no_show_reservation_still_linked_to_room".to_string(),
            ),
        }
    }

    pub fn from_reservation_status(
        reservation: Reservation,
    ) -> Option<Self> {
        if reservation.reservation_status == ReservationStatus::Cancelled {
            return None;
        }

        if reservation.stay_status == Some(StayStatus::CheckedOut) {
            return None;
        }

        if reservation.reservation_status == ReservationStatus::NoShow
            || reservation.stay_status == Some(StayStatus::NoShow)
        {
            return Some(Self::no_show_warning(reservation));
        }

        if reservation.reservation_status == ReservationStatus::Confirmed
            && matches!(
                reservation.stay_status,
                Some(StayStatus::Confirmed) | Some(StayStatus::CheckedIn)
            )
        {
            return Some(Self::assigned(reservation));
        }

        None
    }
}

#[derive(Debug, Serialize)]
pub struct RoomDetailResponse {
    pub id: Uuid,
    pub room_no: String,
    pub room_class: String,
    pub capacity: Option<u32>,
    pub area_sqm: Decimal,
    pub is_physical: bool,
    pub is_active: bool,
    pub daily_state: Option<RoomDailyStateResponse>,
    pub assignment: RoomAssignmentVisibilityResponse,
}

impl RoomDetailResponse {
    pub fn from_parts(
        room: Room,
        daily_state: Option<RoomDailyState>,
        assignment: RoomAssignmentVisibilityResponse,
    ) -> Self {
        Self {
            id: room.id,

            room_no: room.room_no,

            room_class: room.room_class,

            capacity: room.capacity,

            area_sqm: room.area_sqm,

            is_physical: room.is_physical,

            is_active: room.is_active,

            daily_state: daily_state.map(RoomDailyStateResponse::from),

            assignment,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RoomListItemResponse {
    pub id: Uuid,
    pub room_no: String,
    pub room_class: String,
    pub capacity: Option<u32>,
    pub area_sqm: Decimal,
    pub is_physical: bool,
    pub is_active: bool,
    pub daily_state: Option<RoomDailyStateResponse>,
    pub assignment: RoomAssignmentVisibilityResponse,
}

impl RoomListItemResponse {
    pub fn from_parts(
        room: Room,
        daily_state: Option<RoomDailyState>,
        assignment: RoomAssignmentVisibilityResponse,
    ) -> Self {
        Self {
            id: room.id,

            room_no: room.room_no,

            room_class: room.room_class,

            capacity: room.capacity,

            area_sqm: room.area_sqm,

            is_physical: room.is_physical,

            is_active: room.is_active,

            daily_state: daily_state.map(RoomDailyStateResponse::from),

            assignment,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RoomListResponse {
    pub rooms: Vec<RoomListItemResponse>,
}

impl From<Vec<RoomListItemResponse>> for RoomListResponse {
    fn from(rooms: Vec<RoomListItemResponse>) -> Self {
        Self { rooms }
    }
}