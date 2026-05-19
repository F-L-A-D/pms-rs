INSERT INTO guest_aggregates (
            guest_id,
            total_stays,
            total_nights,
            total_spending,
            last_stay_at,
            projection_version,
            updated_at
        )
        VALUES (
            ?, ?, ?, ?, ?, ?, ?
        )
        ON CONFLICT(guest_id)
        DO UPDATE SET
            total_stays        = excluded.total_stays,
            total_nights       = excluded.total_nights,
            total_spending     = excluded.total_spending,
            last_stay_at       = excluded.last_stay_at,
            projection_version = excluded.projection_version,
            updated_at         = excluded.updated_at