use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize)]
pub struct CreateReservationRequest {
    pub id: String,
    pub check_in: String,
    pub nights: u32,
    pub room_class: String,
    pub primary_guest_id: Option<String>,
}

#[derive(Deserialize)]
pub struct ModifyReservationRequest {
    pub check_in: String,
    pub nights: u32,
    pub room_class: String,
}

#[derive(Serialize)]
pub struct ReservationResponse {
    pub id: String,
    pub status: String,
}