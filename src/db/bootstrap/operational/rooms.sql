CREATE TABLE IF NOT EXISTS rooms (
    id TEXT PRIMARY KEY,
    room_no TEXT NOT NULL,
    room_class TEXT NOT NULL,
    occupancy_status TEXT NOT NULL,
    housekeeping_status TEXT NOT NULL,
    UNIQUE(room_no)
);