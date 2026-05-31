CREATE TABLE IF NOT EXISTS business_dates (
    id TEXT PRIMARY KEY NOT NULL,
    business_date TEXT NOT NULL UNIQUE,
    status TEXT NOT NULL,
    opened_at TEXT NOT NULL,
    closing_started_at TEXT,
    closed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    CHECK (status IN ('open', 'closing', 'closed'))
);

CREATE INDEX IF NOT EXISTS idx_business_dates_status
ON business_dates(status);

CREATE UNIQUE INDEX IF NOT EXISTS idx_business_dates_single_active
ON business_dates((1))
WHERE status IN ('open', 'closing');