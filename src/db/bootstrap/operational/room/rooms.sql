CREATE TABLE IF NOT EXISTS rooms (
    id TEXT PRIMARY KEY,
    room_no TEXT NOT NULL,
    room_class TEXT NOT NULL,
    capacity INTEGER NULL,
    area_sqm TEXT NOT NULL,
    is_physical INTEGER NOT NULL,
    is_active INTEGER NOT NULL
);