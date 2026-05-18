CREATE TABLE IF NOT EXISTS monthly_hotel_kpi_aggregates (
    year_month TEXT NOT NULL PRIMARY KEY,
    total_room_nights INTEGER NOT NULL,
    out_of_order_room_nights INTEGER NOT NULL,
    reservable_room_nights INTEGER NOT NULL,
    sold_room_nights INTEGER NOT NULL,
    occupied_room_nights INTEGER NOT NULL,
    room_revenue TEXT NOT NULL,
    food_and_beverage_revenue TEXT NOT NULL,
    other_revenue TEXT NOT NULL,
    tax_amount TEXT NOT NULL,
    total_revenue TEXT NOT NULL,
    occupancy_rate TEXT NOT NULL,
    adr TEXT NOT NULL,
    revpar TEXT NOT NULL,
    projection_version INTEGER NOT NULL,
    updated_at TEXT NOT NULL
);
