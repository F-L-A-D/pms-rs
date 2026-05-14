CREATE TABLE IF NOT EXISTS guest_activities(
    guest_id TEXT PRIMARY KEY,
    is_active INTEGER NOT NULL,
    projection_version INTEGER NOT NULL,
    updated_at TEXT NOT NULL
);