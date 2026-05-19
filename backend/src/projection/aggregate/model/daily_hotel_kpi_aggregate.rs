use chrono::{DateTime, NaiveDate, Utc};

use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct DailyHotelKpiAggregate {
    pub service_date: NaiveDate,
    pub total_rooms: i64,
    pub out_of_order_rooms: i64,
    pub reservable_rooms: i64,
    pub sold_room_nights: i64,
    pub occupied_rooms: i64,
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

impl PartialEq for DailyHotelKpiAggregate {
    fn eq(&self, other: &Self) -> bool {
        self.service_date == other.service_date
            && self.total_rooms == other.total_rooms
            && self.out_of_order_rooms == other.out_of_order_rooms
            && self.reservable_rooms == other.reservable_rooms
            && self.sold_room_nights == other.sold_room_nights
            && self.occupied_rooms == other.occupied_rooms
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
