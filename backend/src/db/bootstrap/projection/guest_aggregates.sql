CREATE TABLE IF NOT EXISTS guest_aggregates (
    guest_id TEXT PRIMARY KEY,
    total_stays INTEGER NOT NULL,
    total_nights INTEGER NOT NULL,
    total_spending INTEGER NOT NULL,
    last_stay_at TEXT,
    projection_version INTEGER NOT NULL,
    updated_at TEXT NOT NULL
);