CREATE TABLE IF NOT EXISTS reservation_daily_revenue_allocations (
    reservation_id TEXT NOT NULL,
    service_date TEXT NOT NULL,
    package_code TEXT NOT NULL,
    revenue_category TEXT NOT NULL,
    amount TEXT NOT NULL,
    PRIMARY KEY (reservation_id, service_date, package_code, revenue_category)
);
