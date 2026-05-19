CREATE TABLE IF NOT EXISTS package_definitions (
    package_code TEXT PRIMARY KEY,
    display_name TEXT NOT NULL,
    revenue_category TEXT NOT NULL,
    department_code TEXT NOT NULL,
    account_code TEXT NOT NULL,
    is_active INTEGER NOT NULL
);
