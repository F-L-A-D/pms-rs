CREATE TABLE IF NOT EXISTS night_audit_room_charge_postings (
    id TEXT PRIMARY KEY,
    business_date_id TEXT NOT NULL,
    business_date TEXT NOT NULL,
    reservation_id TEXT NOT NULL,
    folio_id TEXT NOT NULL,
    service_date TEXT NOT NULL,
    amount TEXT NOT NULL,
    folio_entry_id TEXT NOT NULL,
    posted_at TEXT NOT NULL,
    UNIQUE (reservation_id, service_date),
    FOREIGN KEY (business_date_id) REFERENCES business_dates(id),
    FOREIGN KEY (reservation_id) REFERENCES reservations(id),
    FOREIGN KEY (folio_id) REFERENCES folios(id),
    FOREIGN KEY (folio_entry_id) REFERENCES folio_entries(id)
);
