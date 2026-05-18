use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::{infra, AppResult},
    projection::signal::model::confidence_profile::ConfidenceProfile,
};

pub async fn save_confidence_profile(
    tx: &mut Transaction<'_, Sqlite>,
    profile: &ConfidenceProfile,
) -> AppResult<()> {
    sqlx::query(
        r#"
        INSERT INTO confidence_profiles (
            event_id,
            confidence_score,
            reasons_json,
            projection_version,
            updated_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5)
        ON CONFLICT(event_id)
        DO UPDATE SET
            confidence_score = excluded.confidence_score,
            reasons_json = excluded.reasons_json,
            projection_version = excluded.projection_version,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(profile.event_id.to_string())
    .bind(profile.confidence_score.to_string())
    .bind(&profile.reasons_json)
    .bind(profile.projection_version)
    .bind(profile.updated_at)
    .execute(&mut **tx)
    .await
    .map_err(infra)?;

    Ok(())
}
