CREATE TABLE IF NOT EXISTS guests (
    id TEXT PRIMARY KEY,
    last_name TEXT NOT NULL,
    first_name TEXT NOT NULL,
    phone TEXT,
    email TEXT,
    nationality TEXT,
    birth_date TEXT,
    gender TEXT,
    membership_code TEXT,
    marketing_opt_in INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_guests_last_name
ON guests(last_name);

CREATE INDEX IF NOT EXISTS idx_guests_first_name
ON guests(first_name);