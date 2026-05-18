CREATE TABLE IF NOT EXISTS inventory_aggregates (
    service_date TEXT NOT NULL,
    room_class TEXT NOT NULL,
    total_rooms INTEGER NOT NULL,
    out_of_order_rooms INTEGER NOT NULL,
    reservable_rooms INTEGER NOT NULL,
    confirmed_reservations INTEGER NOT NULL,
    pending_reservations INTEGER NOT NULL,
    cancelled_reservations INTEGER NOT NULL,
    available_rooms INTEGER NOT NULL,
    available_rooms_including_pending INTEGER NOT NULL,
    projection_version INTEGER NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (service_date, room_class)
);
