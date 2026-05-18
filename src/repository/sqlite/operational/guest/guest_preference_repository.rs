use chrono::{DateTime, Utc};

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::guest_preference::{GuestPreference, GuestPreferenceType},
    error::app_error::{infra, AppResult},
};

pub struct SqliteGuestPreferenceRepository;

impl SqliteGuestPreferenceRepository {
    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        preference: &GuestPreference,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO guest_preferences (
                id,
                guest_id,
                preference_type,
                value,
                notes,
                created_at
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
        )
        .bind(preference.id.to_string())
        .bind(preference.guest_id.to_string())
        .bind(preference.preference_type.to_snake())
        .bind(&preference.value)
        .bind(&preference.notes)
        .bind(preference.created_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn list_by_guest_id(
        tx: &mut Transaction<'_, Sqlite>,
        guest_id: Uuid,
    ) -> AppResult<Vec<GuestPreference>> {
        let rows = sqlx::query(
            r#"
            SELECT
                id,
                guest_id,
                preference_type,
                value,
                notes,
                created_at
            FROM guest_preferences
            WHERE guest_id = ?1
            ORDER BY created_at, id
            "#,
        )
        .bind(guest_id.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        rows.iter().map(Self::row_to_preference).collect()
    }

    fn row_to_preference(row: &sqlx::sqlite::SqliteRow) -> AppResult<GuestPreference> {
        Ok(GuestPreference {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str()).map_err(infra)?,
            guest_id: Uuid::parse_str(row.get::<String, _>("guest_id").as_str()).map_err(infra)?,
            preference_type: GuestPreferenceType::from_snake(
                row.get::<String, _>("preference_type").as_str(),
            )
            .ok_or_else(|| infra("invalid guest preference type"))?,
            value: row.get("value"),
            notes: row.get("notes"),
            created_at: row
                .get::<String, _>("created_at")
                .parse::<DateTime<Utc>>()
                .map_err(infra)?,
        })
    }
}
