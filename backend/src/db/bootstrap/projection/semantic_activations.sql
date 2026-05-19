CREATE TABLE IF NOT EXISTS semantic_activations (
    event_id TEXT PRIMARY KEY,
    activation_key TEXT NOT NULL,
    activation_score TEXT NOT NULL,
    confidence_score TEXT NOT NULL,
    is_active INTEGER NOT NULL,
    projection_version INTEGER NOT NULL,
    updated_at TEXT NOT NULL
);
