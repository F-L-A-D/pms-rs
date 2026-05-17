CREATE TABLE IF NOT EXISTS reservation_transitions (
    id TEXT PRIMARY KEY,
    reservation_id TEXT NOT NULL,
    transition_type TEXT NOT NULL,
    field_name TEXT NOT NULL,
    before_value TEXT NOT NULL,
    after_value TEXT NOT NULL,
    occurred_at TEXT NOT NULL
);
