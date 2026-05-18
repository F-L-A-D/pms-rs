use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomRequest {
    pub room_no: String,
    pub room_class: String,
    #[serde(default)]
    pub capacity: Option<u32>,
    #[serde(default = "default_area_sqm")]
    pub area_sqm: Decimal,
    #[serde(default = "default_is_physical")]
    pub is_physical: bool,
}

fn default_area_sqm() -> Decimal {
    Decimal::ZERO
}

fn default_is_physical() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoomRequest {
    pub room_no: Option<String>,
    pub room_class: Option<String>,
    pub capacity: Option<Option<u32>>,
    pub area_sqm: Option<Decimal>,
    pub is_physical: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoomActivationRequest {
    pub is_active: bool,
}
