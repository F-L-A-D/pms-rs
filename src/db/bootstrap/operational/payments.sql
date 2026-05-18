CREATE TABLE IF NOT EXISTS payments (
    id TEXT PRIMARY KEY,
    folio_id TEXT NOT NULL,
    amount TEXT NOT NULL,
    method TEXT NOT NULL,
    external_reference TEXT NULL,
    paid_at TEXT NOT NULL,

    FOREIGN KEY(folio_id)
        REFERENCES folios(id)
);
