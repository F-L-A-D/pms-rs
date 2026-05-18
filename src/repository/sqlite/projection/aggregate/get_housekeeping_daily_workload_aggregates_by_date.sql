SELECT
    service_date,
    room_class,
    total_tracked_rooms,
    dirty_rooms,
    cleaning_rooms,
    cleaned_rooms,
    inspected_rooms,
    occupied_rooms,
    vacant_rooms,
    out_of_order_rooms,
    projection_version,
    updated_at
FROM housekeeping_daily_workload_aggregates
WHERE service_date = ?
ORDER BY room_class
