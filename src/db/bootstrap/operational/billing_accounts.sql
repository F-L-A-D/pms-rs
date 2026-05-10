CREATE TABLE IF NOT EXISTS billing_accounts (
    id TEXT PRIMARY KEY,
    company_id TEXT NULL,
    name TEXT NOT NULL,
    status TEXT NOT NULL,
    
    FOREIGN KEY(company_id)
        REFERENCES companies(id)
);