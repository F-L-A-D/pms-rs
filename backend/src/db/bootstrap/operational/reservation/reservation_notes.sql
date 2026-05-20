CREATE TABLE IF NOT EXISTS reservation_notes (
    id TEXT PRIMARY KEY,
    reservation_id TEXT NOT NULL,
    kind TEXT NOT NULL,
    body TEXT NOT NULL,
    actor_id TEXT,
    created_at TEXT NOT NULL,
    deleted_at TEXT,
    deleted_by TEXT
);