use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::{infra, AppResult},
    projection::signal::model::semantic_activation::SemanticActivation,
};

pub async fn save_semantic_activation(
    tx: &mut Transaction<'_, Sqlite>,
    activation: &SemanticActivation,
) -> AppResult<()> {
    sqlx::query(
        r#"
        INSERT INTO semantic_activations (
            event_id,
            activation_key,
            activation_score,
            confidence_score,
            is_active,
            projection_version,
            updated_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        ON CONFLICT(event_id)
        DO UPDATE SET
            activation_key = excluded.activation_key,
            activation_score = excluded.activation_score,
            confidence_score = excluded.confidence_score,
            is_active = excluded.is_active,
            projection_version = excluded.projection_version,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(activation.event_id.to_string())
    .bind(activation.activation_key.to_snake())
    .bind(activation.activation_score.to_string())
    .bind(activation.confidence_score.to_string())
    .bind(activation.is_active)
    .bind(activation.projection_version)
    .bind(activation.updated_at)
    .execute(&mut **tx)
    .await
    .map_err(infra)?;

    Ok(())
}
