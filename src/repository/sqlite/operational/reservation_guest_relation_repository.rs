use sqlx::{
    Row,
    Sqlite,
    Transaction,
};

use crate::domain::reservation_guest_relation::{
    ReservationGuestRelation,
    ReservationGuestRelationType,
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
        .bind(relation.guest_id.to_string())
        .bind(
            relation
                .relation_type
                .as_str()
        )
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn list_by_reservation_id(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: &str,
    ) -> Result<Vec<ReservationGuestRelation>, String> {

        let rows =
            sqlx::query(
                r#"
                SELECT
                    reservation_id,
                    guest_id,
                    relation_type
                FROM reservation_guest_relations
                WHERE reservation_id = ?1
                "#
            )
            .bind(reservation_id)
            .fetch_all(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

        Ok(
            rows
                .into_iter()
                .map(Self::row_to_relation)
                .collect()
        )
    }

    fn row_to_relation(
        row: sqlx::sqlite::SqliteRow,
    ) -> ReservationGuestRelation {

        ReservationGuestRelation {
            reservation_id:
                row.get("reservation_id"),

            guest_id:
                uuid::Uuid::parse_str(
                    row.get::<String, _>("guest_id")
                        .as_str()
                )
                .unwrap(),

            relation_type:
                ReservationGuestRelationType
                    ::from_str(
                        row.get::<String, _>(
                            "relation_type"
                        )
                        .as_str()
                    )
                    .unwrap(),
        }
    }
}