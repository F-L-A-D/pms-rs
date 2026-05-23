CREATE TABLE IF NOT EXISTS deposit_refunds (
    id TEXT PRIMARY KEY NOT NULL,
    deposit_id TEXT NOT NULL,
    amount TEXT NOT NULL,
    reason TEXT,
    refunded_at TEXT NOT NULL,
    created_at TEXT NOT NULL,

    FOREIGN KEY (deposit_id) REFERENCES deposits(id)
);