CREATE TABLE IF NOT EXISTS reservation_guest_relations (
    reservation_id TEXT NOT NULL,
    guest_id TEXT NOT NULL,
    relation_type TEXT NOT NULL,
    UNIQUE(reservation_id, guest_id)
);