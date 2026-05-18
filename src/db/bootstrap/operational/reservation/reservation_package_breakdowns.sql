CREATE TABLE IF NOT EXISTS reservation_package_breakdowns (
    reservation_id TEXT NOT NULL,
    package_code TEXT NOT NULL,
    revenue_category TEXT NOT NULL,
    amount TEXT NOT NULL,
    PRIMARY KEY (reservation_id, package_code, revenue_category)
);
