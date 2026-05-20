CREATE TABLE IF NOT EXISTS reservation_guest_relations (
    reservation_id TEXT NOT NULL,
    guest_id TEXT NOT NULL,
    relation_type TEXT NOT NULL,
    UNIQUE(reservation_id, guest_id)
);

CREATE INDEX IF NOT EXISTS idx_reservation_guest_relations_reservation
ON reservation_guest_relations(reservation_id);

CREATE INDEX IF NOT EXISTS idx_reservation_guest_relations_guest
ON reservation_guest_relations(guest_id);

CREATE INDEX IF NOT EXISTS idx_reservation_guest_relations_type
ON reservation_guest_relations(relation_type);