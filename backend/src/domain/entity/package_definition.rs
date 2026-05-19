use serde::{Deserialize, Serialize};

use crate::domain::semantic::reservation_booking::ReservationRevenueCategory;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RatePlanDefinition {
    pub plan_code: String,
    pub display_name: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageDefinition {
    pub package_code: String,
    pub display_name: String,
    pub revenue_category: ReservationRevenueCategory,
    pub department_code: String,
    pub account_code: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RatePlanPackage {
    pub plan_code: String,
    pub package_code: String,
    pub sort_order: i64,
}
