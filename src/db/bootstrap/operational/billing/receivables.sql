CREATE TABLE IF NOT EXISTS receivables (
    id TEXT PRIMARY KEY,
    invoice_id TEXT NOT NULL,
    outstanding_amount TEXT NOT NULL,
    due_date TEXT NOT NULL,
    status TEXT NOT NULL,

    FOREIGN KEY(invoice_id)
        REFERENCES invoices(id)
);
