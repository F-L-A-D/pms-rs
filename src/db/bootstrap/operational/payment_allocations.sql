CREATE TABLE IF NOT EXISTS payment_allocations (
    id TEXT PRIMARY KEY,
    payment_id TEXT NOT NULL,
    receivable_id TEXT NOT NULL,
    amount TEXT NOT NULL,
    allocated_at TEXT NOT NULL,
    reversed_at TEXT,

    FOREIGN KEY(payment_id)
        REFERENCES payments(id),

    FOREIGN KEY(receivable_id)
        REFERENCES receivables(id)
);
