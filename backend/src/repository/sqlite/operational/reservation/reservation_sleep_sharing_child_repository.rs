use chrono::NaiveDate;

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::{entity::guest::Gender, semantic::reservation_booking::ReservationSleepSharingChild},
    error::app_error::{infra, AppResult},
};

pub struct SqliteReservationSleepSharingChildRepository;

impl SqliteReservationSleepSharingChildRepository {
    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        child: &ReservationSleepSharingChild,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO reservation_sleep_sharing_children (
                reservation_id,
                service_date,
                display_order,
                name,
                age,
                gender
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
        )
        .bind(child.reservation_id.to_string())
        .bind(child.service_date.to_string())
        .bind(child.display_order)
        .bind(&child.name)
        .bind(child.age)
        .bind(child.gender.as_ref().map(Gender::to_snake))
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn list_by_reservation_id(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: Uuid,
    ) -> AppResult<Vec<ReservationSleepSharingChild>> {
        let rows = sqlx::query(
            r#"
            SELECT
                reservation_id,
                service_date,
                display_order,
                name,
                age,
                gender
            FROM reservation_sleep_sharing_children
            WHERE reservation_id = ?1
            ORDER BY service_date, display_order
            "#,
        )
        .bind(reservation_id.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.iter().map(Self::row_to_child).collect()
    }

    pub async fn delete_by_reservation_id(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: Uuid,
    ) -> AppResult<()> {
        sqlx::query("DELETE FROM reservation_sleep_sharing_children WHERE reservation_id = ?1")
            .bind(reservation_id.to_string())
            .execute(&mut **tx)
            .await
            .map_err(infra)?;

        Ok(())
    }

    fn row_to_child(row: &sqlx::sqlite::SqliteRow) -> AppResult<ReservationSleepSharingChild> {
        let gender = row
            .get::<Option<String>, _>("gender")
            .map(|value| Gender::from_snake(&value).ok_or_else(|| infra("invalid gender")))
            .transpose()?;

        Ok(ReservationSleepSharingChild {
            reservation_id: Uuid::parse_str(row.get::<String, _>("reservation_id").as_str())
                .map_err(infra)?,
            service_date: row
                .get::<String, _>("service_date")
                .parse::<NaiveDate>()
                .map_err(infra)?,
            display_order: row.get("display_order"),
            name: row.get("name"),
            age: row.get("age"),
            gender,
        })
    }
}
