use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::reservation_guest_relation::{
        ReservationGuestRelation, ReservationGuestRelationType,
    },
    error::app_error::{infra, AppResult},
};

pub struct SqliteReservationGuestRelationRepository;

impl SqliteReservationGuestRelationRepository {
    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        relation: &ReservationGuestRelation,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO reservation_guest_relations (
                reservation_id,
                guest_id,
                relation_type
            )
            VALUES (?1, ?2, ?3)
            "#,
        )
        .bind(relation.reservation_id.to_string())
        .bind(relation.guest_id.to_string())
        .bind(relation.relation_type.to_snake())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    fn row_to_relation(row: &sqlx::sqlite::SqliteRow) -> AppResult<ReservationGuestRelation> {
        Ok(ReservationGuestRelation {
            reservation_id: Uuid::parse_str(row.get::<String, _>("reservation_id").as_str())
                .map_err(infra)?,

            guest_id: Uuid::parse_str(row.get::<String, _>("guest_id").as_str()).map_err(infra)?,

            relation_type: ReservationGuestRelationType::from_snake(
                row.get::<String, _>("relation_type").as_str(),
            )
            .ok_or_else(|| infra("invalid reservation guest relation type"))?,
        })
    }

    pub async fn list_by_reservation_id(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: &Uuid,
    ) -> AppResult<Vec<ReservationGuestRelation>> {
        let rows = sqlx::query(
            r#"
            SELECT
                reservation_id,
                guest_id,
                relation_type
            FROM reservation_guest_relations
            WHERE reservation_id = ?1
            "#,
        )
        .bind(reservation_id.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        let relations = rows
            .into_iter()
            .map(|row| Self::row_to_relation(&row))
            .collect::<AppResult<Vec<_>>>()?;

        Ok(relations)
    }

    pub async fn list_by_guest_id(
        tx: &mut Transaction<'_, Sqlite>,
        guest_id: &Uuid,
    ) -> AppResult<Vec<ReservationGuestRelation>> {
        let rows = sqlx::query(
            r#"
            SELECT
                reservation_id,
                guest_id,
                relation_type
            FROM reservation_guest_relations
            WHERE guest_id = ?1
            "#,
        )
        .bind(guest_id.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        let relations = rows
            .into_iter()
            .map(|row| Self::row_to_relation(&row))
            .collect::<AppResult<Vec<_>>>()?;

        Ok(relations)
    }
}
