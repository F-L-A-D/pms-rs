use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct StayResponse {
    pub id: Uuid,
    pub status: String,
}
