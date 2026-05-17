CREATE TABLE IF NOT EXISTS housekeeping_daily_workload_aggregates (
    service_date TEXT NOT NULL,
    room_class TEXT NOT NULL,
    total_tracked_rooms INTEGER NOT NULL,
    dirty_rooms INTEGER NOT NULL,
    cleaning_rooms INTEGER NOT NULL,
    cleaned_rooms INTEGER NOT NULL,
    inspected_rooms INTEGER NOT NULL,
    occupied_rooms INTEGER NOT NULL,
    vacant_rooms INTEGER NOT NULL,
    out_of_order_rooms INTEGER NOT NULL,
    projection_version INTEGER NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (service_date, room_class)
);
