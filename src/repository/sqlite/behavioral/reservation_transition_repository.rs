use chrono::{DateTime, Utc};

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::reservation_transition::{ReservationTransition, ReservationTransitionType},
    error::app_error::{infra, AppResult},
};

pub struct SqliteReservationTransitionRepository;

impl SqliteReservationTransitionRepository {
    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        transition: &ReservationTransition,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO reservation_transitions (
                id,
                reservation_id,
                transition_type,
                field_name,
                before_value,
                after_value,
                occurred_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
        )
        .bind(transition.id.to_string())
        .bind(transition.reservation_id.to_string())
        .bind(transition.transition_type.to_snake())
        .bind(&transition.field_name)
        .bind(&transition.before_value)
        .bind(&transition.after_value)
        .bind(transition.occurred_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_reservation_id(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: Uuid,
    ) -> AppResult<Vec<ReservationTransition>> {
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                reservation_id,
                transition_type,
                field_name,
                before_value,
                after_value,
                occurred_at
            FROM reservation_transitions
            WHERE reservation_id = ?1
            ORDER BY occurred_at ASC
            "#,
        )
        .bind(reservation_id.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.iter().map(Self::row_to_transition).collect()
    }

    fn row_to_transition(row: &sqlx::sqlite::SqliteRow) -> AppResult<ReservationTransition> {
        let transition_type =
            ReservationTransitionType::from_snake(row.get::<String, _>("transition_type").as_str())
                .ok_or_else(|| infra("invalid reservation transition type"))?;

        let occurred_at =
            DateTime::parse_from_rfc3339(row.get::<String, _>("occurred_at").as_str())
                .map_err(infra)?
                .with_timezone(&Utc);

        Ok(ReservationTransition {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str()).map_err(infra)?,

            reservation_id: Uuid::parse_str(row.get::<String, _>("reservation_id").as_str())
                .map_err(infra)?,

            transition_type,

            field_name: row.get("field_name"),

            before_value: row.get("before_value"),

            after_value: row.get("after_value"),

            occurred_at,
        })
    }
}
