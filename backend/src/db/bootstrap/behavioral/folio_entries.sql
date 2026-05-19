CREATE TABLE IF NOT EXISTS folio_entries (
    id TEXT PRIMARY KEY,
    folio_id TEXT NOT NULL,
    entry_type TEXT NOT NULL,
    amount TEXT NOT NULL,
    occurred_at TEXT NOT NULL,
    description TEXT
);
