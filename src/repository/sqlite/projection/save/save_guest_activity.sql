INSERT INTO guest_activities (
            guest_id,
            is_active,
            projection_version,
            updated_at
        )
        VALUES (
            ?, ?, ?, ?
        )
        ON CONFLICT(guest_id)
        DO UPDATE SET
            is_active = excluded.is_active,
            projection_version = excluded.projection_version,
            updated_at = excluded.updated_at
            