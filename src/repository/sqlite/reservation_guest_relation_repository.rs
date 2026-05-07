use sqlx::{Sqlite, Transaction};

use crate::domain::reservation_guest_relation::{
    ReservationGuestRelation,
};

pub struct SqliteReservationGuestRelationRepository;

impl SqliteReservationGuestRelationRepository {

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        relation: &ReservationGuestRelation,
    ) -> Result<(), String> {

        sqlx::query(
            r#"
            INSERT INTO reservation_guest_relations (
                reservation_id,
                guest_id,
                relation_type
            )
            VALUES (?1, ?2, ?3)
            "#
        )
        .bind(&relation.reservation_id)
        .bind(&relation.guest_id)
        .bind(relation.relation_type.as_str())
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}