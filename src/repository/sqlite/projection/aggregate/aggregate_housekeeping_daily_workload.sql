SELECT
    r.room_class AS room_class,
    COUNT(*) AS total_tracked_rooms,
    SUM(CASE WHEN rds.housekeeping_status = 'dirty' THEN 1 ELSE 0 END) AS dirty_rooms,
    SUM(CASE WHEN rds.housekeeping_status = 'cleaning' THEN 1 ELSE 0 END) AS cleaning_rooms,
    SUM(CASE WHEN rds.housekeeping_status = 'cleaned' THEN 1 ELSE 0 END) AS cleaned_rooms,
    SUM(CASE WHEN rds.housekeeping_status = 'inspected' THEN 1 ELSE 0 END) AS inspected_rooms,
    SUM(CASE WHEN rds.occupancy_status = 'occupied' THEN 1 ELSE 0 END) AS occupied_rooms,
    SUM(CASE WHEN rds.occupancy_status = 'vacant' THEN 1 ELSE 0 END) AS vacant_rooms,
    SUM(CASE WHEN rds.occupancy_status = 'out_of_order' THEN 1 ELSE 0 END) AS out_of_order_rooms
FROM room_daily_states rds
INNER JOIN rooms r
    ON r.id = rds.room_id
WHERE rds.service_date = ?1
  AND r.is_physical = 1
GROUP BY r.room_class
ORDER BY r.room_class
