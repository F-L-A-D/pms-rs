SELECT
    r.check_in,
    r.check_out,
    rpb.revenue_category,
    rpb.amount
FROM reservations r
INNER JOIN reservation_package_breakdowns rpb
    ON r.id = rpb.reservation_id
WHERE r.check_in <= ?1
  AND r.check_out > ?1
  AND r.room_class = ?2
  AND r.reservation_status = 'confirmed'
ORDER BY r.id, rpb.package_code, rpb.revenue_category
