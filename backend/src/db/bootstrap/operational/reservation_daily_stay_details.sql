CREATE TABLE IF NOT EXISTS reservation_daily_stay_details (
    reservation_id TEXT NOT NULL,
    service_date TEXT NOT NULL,
    room_class TEXT NOT NULL,
    plan_code TEXT,
    adult_count INTEGER NOT NULL,
    child_count INTEGER NOT NULL,
    sleep_sharing_child_count INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (reservation_id, service_date)
);
