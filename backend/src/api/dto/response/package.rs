use serde::{Deserialize, Serialize};

use crate::domain::semantic::reservation_booking::ReservationRevenueCategory;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatePlanResponse {
    pub plan_code: String,
    pub display_name: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDefinitionResponse {
    pub package_code: String,
    pub display_name: String,
    pub revenue_category: ReservationRevenueCategory,
    pub department_code: String,
    pub account_code: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatePlanPackageResponse {
    pub plan_code: String,
    pub package_code: String,
    pub sort_order: i64,
}
