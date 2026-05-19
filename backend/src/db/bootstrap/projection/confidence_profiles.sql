CREATE TABLE IF NOT EXISTS confidence_profiles (
    event_id TEXT PRIMARY KEY,
    confidence_score TEXT NOT NULL,
    reasons_json TEXT NOT NULL,
    projection_version INTEGER NOT NULL,
    updated_at TEXT NOT NULL
);
