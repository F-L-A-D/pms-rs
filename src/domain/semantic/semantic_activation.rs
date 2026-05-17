use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangePatternType {
    ReservationCreated,
    ReservationDateChanged,
    ReservationRoomClassChanged,
    ReservationDailyPlanChanged,
    ReservationGuestCompositionChanged,
    ReservationRevenueAllocationChanged,
    ReservationStayShapeChanged,
    ReservationCancelled,
    ReservationUpdated,
}

impl ChangePatternType {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::ReservationCreated => "reservation_created",
            Self::ReservationDateChanged => "reservation_date_changed",
            Self::ReservationRoomClassChanged => "reservation_room_class_changed",
            Self::ReservationDailyPlanChanged => "reservation_daily_plan_changed",
            Self::ReservationGuestCompositionChanged => "reservation_guest_composition_changed",
            Self::ReservationRevenueAllocationChanged => "reservation_revenue_allocation_changed",
            Self::ReservationStayShapeChanged => "reservation_stay_shape_changed",
            Self::ReservationCancelled => "reservation_cancelled",
            Self::ReservationUpdated => "reservation_updated",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "reservation_created" => Some(Self::ReservationCreated),
            "reservation_date_changed" => Some(Self::ReservationDateChanged),
            "reservation_room_class_changed" => Some(Self::ReservationRoomClassChanged),
            "reservation_daily_plan_changed" => Some(Self::ReservationDailyPlanChanged),
            "reservation_guest_composition_changed" => {
                Some(Self::ReservationGuestCompositionChanged)
            }
            "reservation_revenue_allocation_changed" => {
                Some(Self::ReservationRevenueAllocationChanged)
            }
            "reservation_stay_shape_changed" => Some(Self::ReservationStayShapeChanged),
            "reservation_cancelled" => Some(Self::ReservationCancelled),
            "reservation_updated" => Some(Self::ReservationUpdated),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticActivationKey {
    ReservationCreated,
    StayShapeChanged,
    InventoryRelevantChange,
    KpiRelevantChange,
    GuestRelevantChange,
    BillingRelevantChange,
    HousekeepingRelevantChange,
    ReservationCancelled,
    ReservationUpdated,
}

impl SemanticActivationKey {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::ReservationCreated => "reservation_created",
            Self::StayShapeChanged => "stay_shape_changed",
            Self::InventoryRelevantChange => "inventory_relevant_change",
            Self::KpiRelevantChange => "kpi_relevant_change",
            Self::GuestRelevantChange => "guest_relevant_change",
            Self::BillingRelevantChange => "billing_relevant_change",
            Self::HousekeepingRelevantChange => "housekeeping_relevant_change",
            Self::ReservationCancelled => "reservation_cancelled",
            Self::ReservationUpdated => "reservation_updated",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "reservation_created" => Some(Self::ReservationCreated),
            "stay_shape_changed" => Some(Self::StayShapeChanged),
            "inventory_relevant_change" => Some(Self::InventoryRelevantChange),
            "kpi_relevant_change" => Some(Self::KpiRelevantChange),
            "guest_relevant_change" => Some(Self::GuestRelevantChange),
            "billing_relevant_change" => Some(Self::BillingRelevantChange),
            "housekeeping_relevant_change" => Some(Self::HousekeepingRelevantChange),
            "reservation_cancelled" => Some(Self::ReservationCancelled),
            "reservation_updated" => Some(Self::ReservationUpdated),
            _ => None,
        }
    }
}
