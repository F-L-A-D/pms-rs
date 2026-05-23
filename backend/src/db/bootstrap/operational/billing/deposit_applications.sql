CREATE TABLE IF NOT EXISTS deposit_applications (
    id TEXT PRIMARY KEY NOT NULL,
    deposit_id TEXT NOT NULL,
    receivable_id TEXT NOT NULL,
    amount TEXT NOT NULL,
    applied_at TEXT NOT NULL,
    reversed_at TEXT,
    FOREIGN KEY (deposit_id) REFERENCES deposits(id),
    FOREIGN KEY (receivable_id) REFERENCES receivables(id)
);