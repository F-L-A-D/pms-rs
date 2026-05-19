SELECT
    COUNT(r.id) AS total_stays,

    CAST(
        COALESCE(
            SUM(
                julianday(r.check_out)
                - julianday(r.check_in)
            ),
            0
        ) AS INTEGER
    ) AS total_nights,

    0 AS total_spending,

    MAX(r.check_out)
        AS last_stay_at

FROM reservation_guest_relations rgr

INNER JOIN reservations r
    ON r.id = rgr.reservation_id

WHERE rgr.guest_id = ?
AND rgr.relation_type = 'primary'
