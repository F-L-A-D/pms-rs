SELECT
    guest_id,
    is_active,
    projection_version,
    updated_at
FROM guest_activities
WHERE guest_id = ?