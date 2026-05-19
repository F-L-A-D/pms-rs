CREATE TABLE IF NOT EXISTS operation_change_events (
    id TEXT PRIMARY KEY,
    operation_id TEXT NOT NULL,
    aggregate_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    operation_type TEXT NOT NULL,
    actor TEXT NOT NULL,
    actor_id TEXT,
    source TEXT NOT NULL,
    before_json TEXT,
    after_json TEXT NOT NULL,
    changed_fields_json TEXT NOT NULL,
    occurred_at TEXT NOT NULL
);
