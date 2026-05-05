use std::collections::HashMap;

use crate::domain::reservation::Reservation;
use crate::repository::reservation_repository::ReservationRepository;

pub struct InMemoryReservationRepository {
    store: HashMap<String, Reservation>,
}

impl InMemoryReservationRepository {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl ReservationRepository for InMemoryReservationRepository {
    fn save(&mut self, reservation: Reservation) {
        self.store.insert(reservation.id.clone(), reservation);
    }

    fn find_by_id(&self, id: &str) -> Option<Reservation> {
        self.store.get(id).cloned()
    }
}