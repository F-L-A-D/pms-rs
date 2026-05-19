SELECT
    service_date,
    room_class,
    total_rooms,
    out_of_order_rooms,
    reservable_rooms,
    sold_room_nights,
    occupied_rooms,
    room_revenue,
    food_and_beverage_revenue,
    other_revenue,
    tax_amount,
    total_revenue,
    occupancy_rate,
    adr,
    revpar,
    projection_version,
    updated_at
FROM daily_room_class_kpi_aggregates
WHERE service_date = ?
ORDER BY room_class
