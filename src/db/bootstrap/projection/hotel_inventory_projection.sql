CREATE TABLE IF NOT EXISTS hotel_inventory_projection (
    date TEXT PRIMARY KEY,
    reserved_rooms INTEGER NOT NULL
);