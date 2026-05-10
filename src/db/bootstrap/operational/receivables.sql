CREATE TABLE IF NOT EXISTS receivables (
    id TEXT PRIMARY KEY,
    invoice_id TEXT NOT NULL UNIQUE,
    outstanding_amount INTEGER NOT NULL,
    status TEXT NOT NULL,

    FOREIGN KEY(invoice_id)
        REFERENCES invoices(id)
);