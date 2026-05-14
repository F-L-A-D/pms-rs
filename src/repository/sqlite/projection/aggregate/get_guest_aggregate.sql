SELECT 
    guest_id,
    total_stays,
    total_nights,
    total_spending,
    last_stay_at,
    projection_version,
    updated_at
FROM guest_aggregates
WHERE guest_id = ?