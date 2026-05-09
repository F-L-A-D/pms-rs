CREATE TABLE IF NOT EXISTS inventory_projection (
    date TEXT NOT NULL,
    room_class TEXT NOT NULL,
    reserved_rooms INTEGER NOT NULL,
    PRIMARY KEY(date, room_class)
);