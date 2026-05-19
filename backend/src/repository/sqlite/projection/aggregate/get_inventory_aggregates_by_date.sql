SELECT
    service_date,
    room_class,
    total_rooms,
    out_of_order_rooms,
    reservable_rooms,
    confirmed_reservations,
    pending_reservations,
    cancelled_reservations,
    available_rooms,
    available_rooms_including_pending,
    projection_version,
    updated_at
FROM inventory_aggregates
WHERE service_date = ?
ORDER BY room_class
