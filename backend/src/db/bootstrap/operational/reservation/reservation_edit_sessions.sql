CREATE TABLE IF NOT EXISTS reservation_edit_sessions (
    id TEXT PRIMARY KEY,
    reservation_id TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    actor_label TEXT,
    opened_at TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    closed_at TEXT,
    FOREIGN KEY (reservation_id) REFERENCES reservations(id)
);
