CREATE TABLE IF NOT EXISTS reservations (
    id TEXT PRIMARY KEY,
    external_id TEXT,
    check_in TEXT NOT NULL,
    check_out TEXT NOT NULL,
    reservation_status TEXT NOT NULL,
    stay_status TEXT,
    room_class TEXT NOT NULL,
    room_id TEXT,
    booking_channel TEXT NOT NULL DEFAULT 'direct',
    source_channel TEXT,
    plan_code TEXT,
    version INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_reservations_check_in
ON reservations(check_in);

CREATE INDEX IF NOT EXISTS idx_reservations_check_out
ON reservations(check_out);

CREATE INDEX IF NOT EXISTS idx_reservations_status
ON reservations(reservation_status);

CREATE INDEX IF NOT EXISTS idx_reservations_stay_status
ON reservations(stay_status);

CREATE INDEX IF NOT EXISTS idx_reservations_room_class
ON reservations(room_class);

CREATE INDEX IF NOT EXISTS idx_reservations_room_id
ON reservations(room_id);

CREATE INDEX IF NOT EXISTS idx_reservations_external_id
ON reservations(external_id);

CREATE INDEX IF NOT EXISTS idx_reservations_booking_channel
ON reservations(booking_channel);

CREATE INDEX IF NOT EXISTS idx_reservations_source_channel
ON reservations(source_channel);