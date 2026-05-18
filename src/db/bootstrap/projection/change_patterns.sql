CREATE TABLE IF NOT EXISTS change_patterns (
    event_id TEXT PRIMARY KEY,
    pattern_type TEXT NOT NULL,
    operation_type TEXT NOT NULL,
    changed_fields_json TEXT NOT NULL,
    projection_version INTEGER NOT NULL,
    updated_at TEXT NOT NULL
);
