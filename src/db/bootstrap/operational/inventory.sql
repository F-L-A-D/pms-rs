CREATE TABLE IF NOT EXISTS inventory (
    date TEXT PRIMARY KEY,
    total_rooms INTEGER NOT NULL,
    reserved_rooms INTEGER NOT NULL
);