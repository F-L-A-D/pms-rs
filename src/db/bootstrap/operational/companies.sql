CREATE TABLE IF NOT EXISTS companies (
    id TEXT PRIMARY KEY,
    legal_name TEXT NOT NULL,
    tax_id TEXT NULL,
    is_active INTEGER NOT NULL,
    created_at TEXT NOT NULL
);
