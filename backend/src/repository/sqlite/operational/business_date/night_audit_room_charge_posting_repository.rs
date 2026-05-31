use chrono::{DateTime, NaiveDate, Utc};

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::entity::night_audit_room_charge_posting::NightAuditRoomChargePosting,
    error::app_error::{infra, AppResult},
};

pub struct SqliteNightAuditRoomChargePostingRepository;

impl SqliteNightAuditRoomChargePostingRepository {
    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        posting: &NightAuditRoomChargePosting,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO night_audit_room_charge_postings (
                id,
                business_date_id,
                business_date,
                reservation_id,
                folio_id,
                service_date,
                amount,
                folio_entry_id,
                posted_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            "#,
        )
        .bind(posting.id.to_string())
        .bind(posting.business_date_id.to_string())
        .bind(posting.business_date.to_string())
        .bind(posting.reservation_id.to_string())
        .bind(posting.folio_id.to_string())
        .bind(posting.service_date.to_string())
        .bind(posting.amount.to_string())
        .bind(posting.folio_entry_id.to_string())
        .bind(posting.posted_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_reservation_and_service_date(
        tx: &mut Transaction<'_, Sqlite>,
        reservation_id: Uuid,
        service_date: NaiveDate,
    ) -> AppResult<Option<NightAuditRoomChargePosting>> {
        let row = sqlx::query(
            r#"
            SELECT
                id,
                business_date_id,
                business_date,
                reservation_id,
                folio_id,
                service_date,
                amount,
                folio_entry_id,
                posted_at
            FROM night_audit_room_charge_postings
            WHERE reservation_id = ?1
              AND service_date = ?2
            "#,
        )
        .bind(reservation_id.to_string())
        .bind(service_date.to_string())
        .fetch_optional(&mut **tx)
        .await
        .map_err(infra)?;

        match row {
            Some(row) => Ok(Some(Self::row_to_posting(&row)?)),
            None => Ok(None),
        }
    }

    fn row_to_posting(row: &sqlx::sqlite::SqliteRow) -> AppResult<NightAuditRoomChargePosting> {
        let posted_at = DateTime::parse_from_rfc3339(row.get::<String, _>("posted_at").as_str())
            .map_err(infra)?
            .with_timezone(&Utc);

        Ok(NightAuditRoomChargePosting {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str()).map_err(infra)?,
            business_date_id: Uuid::parse_str(row.get::<String, _>("business_date_id").as_str())
                .map_err(infra)?,
            business_date: row
                .get::<String, _>("business_date")
                .parse::<NaiveDate>()
                .map_err(infra)?,
            reservation_id: Uuid::parse_str(row.get::<String, _>("reservation_id").as_str())
                .map_err(infra)?,
            folio_id: Uuid::parse_str(row.get::<String, _>("folio_id").as_str()).map_err(infra)?,
            service_date: row
                .get::<String, _>("service_date")
                .parse::<NaiveDate>()
                .map_err(infra)?,
            amount: row.get::<String, _>("amount").parse().map_err(infra)?,
            folio_entry_id: Uuid::parse_str(row.get::<String, _>("folio_entry_id").as_str())
                .map_err(infra)?,
            posted_at,
        })
    }
}
