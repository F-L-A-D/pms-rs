WITH room_classes AS (
    SELECT DISTINCT room_class
    FROM rooms
    WHERE is_physical = 1
      AND is_active = 1

    UNION

    SELECT DISTINCT room_class
    FROM reservations
    WHERE check_in <= ?1
      AND check_out > ?1
)
SELECT
    rc.room_class AS room_class,

    (
        SELECT COUNT(*)
        FROM rooms r
        WHERE r.room_class = rc.room_class
          AND r.is_physical = 1
          AND r.is_active = 1
    ) AS total_rooms,

    (
        SELECT COUNT(DISTINCT rds.room_id)
        FROM room_daily_states rds
        INNER JOIN rooms r
            ON r.id = rds.room_id
        WHERE r.room_class = rc.room_class
          AND r.is_physical = 1
          AND r.is_active = 1
          AND rds.service_date = ?1
          AND rds.occupancy_status = 'out_of_order'
    ) AS out_of_order_rooms,

    (
        SELECT COUNT(*)
        FROM reservations r
        WHERE r.room_class = rc.room_class
          AND r.check_in <= ?1
          AND r.check_out > ?1
          AND r.reservation_status = 'confirmed'
    ) AS sold_room_nights,

    (
        SELECT COUNT(DISTINCT rds.room_id)
        FROM room_daily_states rds
        INNER JOIN rooms r
            ON r.id = rds.room_id
        WHERE r.room_class = rc.room_class
          AND r.is_physical = 1
          AND r.is_active = 1
          AND rds.service_date = ?1
          AND rds.occupancy_status = 'occupied'
    ) AS occupied_rooms
FROM room_classes rc
ORDER BY rc.room_class
