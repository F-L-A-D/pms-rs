CREATE TABLE IF NOT EXISTS reservation_sleep_sharing_children (
    reservation_id TEXT NOT NULL,
    service_date TEXT NOT NULL,
    display_order INTEGER NOT NULL,
    name TEXT,
    age INTEGER,
    gender TEXT,
    PRIMARY KEY (reservation_id, service_date, display_order)
);
