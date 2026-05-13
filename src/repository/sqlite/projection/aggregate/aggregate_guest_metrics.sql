SELECT
    COUNT(r.id) AS total_stays,

    COALESCE(
        SUM(
            julianday(r.check_out_date)
            - julianday(r.check_in_date)
        ),
        0
    ) AS total_nights,

    COALESCE(
        SUM(f.total_amount),
        0
    ) AS total_spending,

    MAX(r.check_out_date)
        AS last_stay_at

FROM reservations r

LEFT JOIN folios f
    ON f.reservation_id = r.id

WHERE r.guest_id = ?