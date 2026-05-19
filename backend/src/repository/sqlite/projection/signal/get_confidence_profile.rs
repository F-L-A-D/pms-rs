use rust_decimal::Decimal;

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    error::app_error::{infra, AppResult},
    projection::signal::model::confidence_profile::ConfidenceProfile,
};

pub async fn get_confidence_profile(
    tx: &mut Transaction<'_, Sqlite>,
    event_id: Uuid,
) -> AppResult<Option<ConfidenceProfile>> {
    let row = sqlx::query(
        r#"
        SELECT *
        FROM confidence_profiles
        WHERE event_id = ?1
        "#,
    )
    .bind(event_id.to_string())
    .fetch_optional(&mut **tx)
    .await
    .map_err(infra)?;

    row.map(|row| {
        Ok(ConfidenceProfile {
            event_id,
            confidence_score: row
                .get::<String, _>("confidence_score")
                .parse::<Decimal>()
                .map_err(infra)?,
            reasons_json: row.get("reasons_json"),
            projection_version: row.get("projection_version"),
            updated_at: row.get("updated_at"),
        })
    })
    .transpose()
}
