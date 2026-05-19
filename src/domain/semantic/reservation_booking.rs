use rust_decimal::Decimal;

use serde::{Deserialize, Serialize};

use chrono::NaiveDate;

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReservationBookingChannel {
    Direct,
    Ota,
    Corporate,
    TravelAgent,
    WalkIn,
    Other,
}

impl ReservationBookingChannel {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Ota => "ota",
            Self::Corporate => "corporate",
            Self::TravelAgent => "travel_agent",
            Self::WalkIn => "walk_in",
            Self::Other => "other",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "direct" => Some(Self::Direct),
            "ota" => Some(Self::Ota),
            "corporate" => Some(Self::Corporate),
            "travel_agent" => Some(Self::TravelAgent),
            "walk_in" => Some(Self::WalkIn),
            "other" => Some(Self::Other),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReservationRevenueCategory {
    Room,
    FoodAndBeverage,
    Other,
    Tax,
}

impl ReservationRevenueCategory {
    pub fn to_snake(&self) -> &'static str {
        match self {
            Self::Room => "room",
            Self::FoodAndBeverage => "food_and_beverage",
            Self::Other => "other",
            Self::Tax => "tax",
        }
    }

    pub fn from_snake(value: &str) -> Option<Self> {
        match value {
            "room" => Some(Self::Room),
            "food_and_beverage" => Some(Self::FoodAndBeverage),
            "other" => Some(Self::Other),
            "tax" => Some(Self::Tax),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReservationPackageBreakdown {
    pub reservation_id: Uuid,
    pub package_code: String,
    pub revenue_category: ReservationRevenueCategory,
    pub amount: Decimal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReservationDailyStayDetail {
    pub reservation_id: Uuid,
    pub service_date: NaiveDate,
    pub room_class: String,
    pub plan_code: Option<String>,
    pub adult_count: i64,
    pub child_count: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReservationDailyRevenueAllocation {
    pub reservation_id: Uuid,
    pub service_date: NaiveDate,
    pub package_code: String,
    pub revenue_category: ReservationRevenueCategory,
    pub department_code: Option<String>,
    pub account_code: Option<String>,
    pub amount: Decimal,
}
