CREATE TABLE IF NOT EXISTS reservations (
    id TEXT PRIMARY KEY,
    external_id TEXT,
    check_in TEXT NOT NULL,
    check_out TEXT NOT NULL,
    reservation_status TEXT NOT NULL,
    stay_status TEXT,
    room_class TEXT NOT NULL,
    room_id TEXT,
    created_at TEXT NOT NULL,
    channel TEXT
);