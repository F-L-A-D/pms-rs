CREATE TABLE IF NOT EXISTS settlement_transitions (
    id TEXT PRIMARY KEY,
    receivable_id TEXT NOT NULL,
    transition_type TEXT NOT NULL,
    amount INTEGER NOT NULL,
    occurred_at TEXT NOT NULL
);