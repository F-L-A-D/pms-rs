CREATE TABLE reservation_search_projections (
    reservation_id TEXT PRIMARY KEY,
    external_id TEXT NOT NULL,
    primary_guest_name TEXT NOT NULL,
    participant_names TEXT NOT NULL,
    check_in TEXT NOT NULL,
    check_out TEXT NOT NULL,
    room_class TEXT NOT NULL,
    room_id TEXT,
    reservation_status TEXT NOT NULL,
    stay_status TEXT,
    projection_version INTEGER NOT NULL,
    updated_at TEXT NOT NULL
);