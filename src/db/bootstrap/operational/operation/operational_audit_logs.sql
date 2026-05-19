CREATE TABLE IF NOT EXISTS operational_audit_logs (
    id TEXT PRIMARY KEY,
    operation_id TEXT NOT NULL,
    aggregate_type TEXT NOT NULL,
    aggregate_id TEXT NOT NULL,
    action TEXT NOT NULL,
    actor TEXT NOT NULL,
    actor_id TEXT,
    source TEXT NOT NULL,
    before_json TEXT,
    after_json TEXT,
    changed_fields_json TEXT NOT NULL,
    reason TEXT,
    occurred_at TEXT NOT NULL
);
