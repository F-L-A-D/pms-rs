CREATE TABLE IF NOT EXISTS room_daily_states (
    room_id TEXT NOT NULL,
    service_date TEXT NOT NULL,
    occupancy_status TEXT NOT NULL,
    housekeeping_status TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (room_id, service_date),
    FOREIGN KEY (room_id) REFERENCES rooms(id)
);
