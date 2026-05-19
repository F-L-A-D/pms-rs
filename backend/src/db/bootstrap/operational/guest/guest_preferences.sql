CREATE TABLE IF NOT EXISTS guest_preferences (
    id TEXT PRIMARY KEY,
    guest_id TEXT NOT NULL,
    preference_type TEXT NOT NULL,
    value TEXT NOT NULL,
    notes TEXT,
    created_at TEXT NOT NULL
);
