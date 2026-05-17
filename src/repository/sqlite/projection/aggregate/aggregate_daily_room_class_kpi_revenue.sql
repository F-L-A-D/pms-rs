SELECT
    rda.revenue_category,
    rda.amount
FROM reservation_daily_revenue_allocations rda
INNER JOIN reservations r
    ON r.id = rda.reservation_id
INNER JOIN reservation_daily_stay_details rsd
    ON rsd.reservation_id = rda.reservation_id
   AND rsd.service_date = rda.service_date
WHERE rda.service_date = ?1
  AND rsd.room_class = ?2
  AND r.reservation_status = 'confirmed'
ORDER BY r.id, rda.package_code, rda.revenue_category
