CREATE TABLE IF NOT EXISTS guest_timeline_events (
    id TEXT PRIMARY KEY,
    guest_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    reference_id TEXT NOT NULL,
    occurred_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_guest_timeline_guest
ON guest_timeline_events (
    guest_id,
    occurred_at DESC
);