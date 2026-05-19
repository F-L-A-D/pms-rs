CREATE TABLE IF NOT EXISTS invoices (
    id TEXT PRIMARY KEY,
    folio_id TEXT NOT NULL UNIQUE,
    billing_account_id TEXT NOT NULL,
    invoice_number TEXT NOT NULL,
    issued_amount TEXT NOT NULL,
    due_date TEXT NOT NULL,
    issued_at TEXT NOT NULL,
    status TEXT NOT NULL,

    FOREIGN KEY(folio_id)
        REFERENCES folios(id),
    FOREIGN KEY(billing_account_id)
        REFERENCES billing_accounts(id)
);

CREATE UNIQUE INDEX IF NOT EXISTS
    idx_invoices_invoice_number_unique
ON invoices(invoice_number);