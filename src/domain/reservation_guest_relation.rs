#[derive(Debug, Clone, PartialEq)]
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
}

#[derive(Debug, Clone ,PartialEq)]
pub struct ReservationGuestRelation {
    pub reservation_id: String,
    pub guest_id: String,
    pub relation_type: ReservationGuestRelationType,
}