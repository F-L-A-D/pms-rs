WITH room_classes AS (
    SELECT DISTINCT room_class
    FROM rooms
    WHERE is_physical = 1
      AND is_active = 1

    UNION

    SELECT DISTINCT rsd.room_class
    FROM reservation_daily_stay_details rsd
    INNER JOIN reservations r
        ON r.id = rsd.reservation_id
    WHERE rsd.service_date = ?1
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
        FROM reservation_daily_stay_details rsd
        INNER JOIN reservations r
            ON r.id = rsd.reservation_id
        WHERE rsd.room_class = rc.room_class
          AND rsd.service_date = ?1
          AND r.reservation_status = 'confirmed'
    ) AS confirmed_reservations,

    (
        SELECT COUNT(*)
        FROM reservation_daily_stay_details rsd
        INNER JOIN reservations r
            ON r.id = rsd.reservation_id
        WHERE rsd.room_class = rc.room_class
          AND rsd.service_date = ?1
          AND r.reservation_status = 'pending'
    ) AS pending_reservations,

    (
        SELECT COUNT(*)
        FROM reservation_daily_stay_details rsd
        INNER JOIN reservations r
            ON r.id = rsd.reservation_id
        WHERE rsd.room_class = rc.room_class
          AND rsd.service_date = ?1
          AND r.reservation_status = 'cancelled'
    ) AS cancelled_reservations
FROM room_classes rc
ORDER BY rc.room_class
