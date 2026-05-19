use chrono::NaiveDate;

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::reservation_booking::ReservationDailyStayDetail,
    error::app_error::{infra, AppResult},
};

pub struct SqliteReservationDailyStayDetailRepository;

impl SqliteReservationDailyStayDetailRepository {
    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        detail: &ReservationDailyStayDetail,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO reservation_daily_stay_details (
                reservation_id,
                service_date,
                room_class,
                plan_code,
                adult_count,
                child_count
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
        )
        .bind(detail.reservation_id.to_string())
        .bind(detail.service_date.to_string())
        .bind(&detail.room_class)
        .bind(&detail.plan_code)
        .bind(detail.adult_count)
        .bind(detail.child_count)
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn list_by_reservation_id(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: Uuid,
    ) -> AppResult<Vec<ReservationDailyStayDetail>> {
        let rows = sqlx::query(
            r#"
            SELECT
                reservation_id,
                service_date,
                room_class,
                plan_code,
                adult_count,
                child_count
            FROM reservation_daily_stay_details
            WHERE reservation_id = ?1
            ORDER BY service_date
            "#,
        )
        .bind(reservation_id.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.iter().map(Self::row_to_detail).collect()
    }

    pub async fn delete_by_reservation_id(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: Uuid,
    ) -> AppResult<()> {
        sqlx::query("DELETE FROM reservation_daily_stay_details WHERE reservation_id = ?1")
            .bind(reservation_id.to_string())
            .execute(&mut **tx)
            .await
            .map_err(infra)?;

        Ok(())
    }

    fn row_to_detail(row: &sqlx::sqlite::SqliteRow) -> AppResult<ReservationDailyStayDetail> {
        Ok(ReservationDailyStayDetail {
            reservation_id: Uuid::parse_str(row.get::<String, _>("reservation_id").as_str())
                .map_err(infra)?,
            service_date: row
                .get::<String, _>("service_date")
                .parse::<NaiveDate>()
                .map_err(infra)?,
            room_class: row.get("room_class"),
            plan_code: row.get("plan_code"),
            adult_count: row.get("adult_count"),
            child_count: row.get("child_count"),
        })
    }
}
