CREATE TABLE IF NOT EXISTS deposits (
    id TEXT PRIMARY KEY,
    folio_id TEXT NOT NULL,
    amount TEXT NOT NULL,
    unapplied_amount TEXT NOT NULL,
    refunded_amount TEXT NOT NULL,
    status TEXT NOT NULL,
    method TEXT NOT NULL,
    external_reference TEXT NULL,
    received_at TEXT NOT NULL,

    FOREIGN KEY(folio_id)
        REFERENCES folios(id)
);