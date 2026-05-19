use rust_decimal::Decimal;

#[derive(Debug)]
pub struct DailyRoomClassKpiAggregateRow {
    pub room_class: String,
    pub total_rooms: i64,
    pub out_of_order_rooms: i64,
    pub sold_room_nights: i64,
    pub occupied_rooms: i64,
    pub room_revenue: Decimal,
    pub food_and_beverage_revenue: Decimal,
    pub other_revenue: Decimal,
    pub tax_amount: Decimal,
}
