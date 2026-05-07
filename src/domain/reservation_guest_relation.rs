use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReservationGuestRelationType {
    Primary,
    Accompany,
}

impl ReservationGuestRelationType {

    pub fn as_str(&self) -> &str {

        match self {
            Self::Primary => "PRIMARY",
            Self::Accompany => "ACCOMPANY",
        }
    }

    pub fn from_str(
        value: &str,
    ) -> Result<Self, String> {

        match value {
            "PRIMARY" => Ok(Self::Primary),
            "ACCOMPANY" => Ok(Self::Accompany),
            _ => Err(
                format!(
                    "invalid relation type: {}",
                    value
                )
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReservationGuestRelation {
    pub reservation_id: String,
    pub guest_id: String,
    pub relation_type: ReservationGuestRelationType,
}

impl ReservationGuestRelation {

    pub fn is_primary(&self) -> bool {

        self.relation_type
            == ReservationGuestRelationType::Primary
    }
}