CREATE TABLE IF NOT EXISTS payment_refunds (
    id TEXT PRIMARY KEY NOT NULL,
    payment_id TEXT NOT NULL,
    amount TEXT NOT NULL,
    reason TEXT,
    refunded_at TEXT NOT NULL,
    created_at TEXT NOT NULL,

    FOREIGN KEY (payment_id) REFERENCES payments(id)
);