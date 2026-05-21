CREATE TABLE IF NOT EXISTS folios (
    id TEXT PRIMARY KEY,
    reservation_id TEXT NOT NULL,
    billing_account_id TEXT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_folios_reservation
ON folios(reservation_id);