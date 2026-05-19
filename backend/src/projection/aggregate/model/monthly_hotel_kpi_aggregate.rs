use chrono::{DateTime, Utc};

use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct MonthlyHotelKpiAggregate {
    pub year_month: String,
    pub total_room_nights: i64,
    pub out_of_order_room_nights: i64,
    pub reservable_room_nights: i64,
    pub sold_room_nights: i64,
    pub occupied_room_nights: i64,
    pub room_revenue: Decimal,
    pub food_and_beverage_revenue: Decimal,
    pub other_revenue: Decimal,
    pub tax_amount: Decimal,
    pub total_revenue: Decimal,
    pub occupancy_rate: Decimal,
    pub adr: Decimal,
    pub revpar: Decimal,
    pub projection_version: i32,
    pub updated_at: DateTime<Utc>,
}

impl PartialEq for MonthlyHotelKpiAggregate {
    fn eq(&self, other: &Self) -> bool {
        self.year_month == other.year_month
            && self.total_room_nights == other.total_room_nights
            && self.out_of_order_room_nights == other.out_of_order_room_nights
            && self.reservable_room_nights == other.reservable_room_nights
            && self.sold_room_nights == other.sold_room_nights
            && self.occupied_room_nights == other.occupied_room_nights
            && self.room_revenue == other.room_revenue
            && self.food_and_beverage_revenue == other.food_and_beverage_revenue
            && self.other_revenue == other.other_revenue
            && self.tax_amount == other.tax_amount
            && self.total_revenue == other.total_revenue
            && self.occupancy_rate == other.occupancy_rate
            && self.adr == other.adr
            && self.revpar == other.revpar
            && self.projection_version == other.projection_version
    }
}
