use crate::domain::reservation::Reservation;

pub trait ReservationRepository {
    fn save(&mut self, reservation: Reservation);
    fn find_by_id(&self, id: &str) -> Option<Reservation>;
}