use rust_decimal::Decimal;

use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::semantic_activation::SemanticActivationKey,
    error::app_error::{infra, AppResult},
    projection::signal::model::semantic_activation::SemanticActivation,
};

pub async fn get_semantic_activation(
    tx: &mut Transaction<'_, Sqlite>,
    event_id: Uuid,
) -> AppResult<Option<SemanticActivation>> {
    let row = sqlx::query(
        r#"
        SELECT *
        FROM semantic_activations
        WHERE event_id = ?1
        "#,
    )
    .bind(event_id.to_string())
    .fetch_optional(&mut **tx)
    .await
    .map_err(infra)?;

    row.map(|row| {
        Ok(SemanticActivation {
            event_id,
            activation_key: SemanticActivationKey::from_snake(
                row.get::<String, _>("activation_key").as_str(),
            )
            .ok_or_else(|| infra("invalid semantic activation key"))?,
            activation_score: row
                .get::<String, _>("activation_score")
                .parse::<Decimal>()
                .map_err(infra)?,
            confidence_score: row
                .get::<String, _>("confidence_score")
                .parse::<Decimal>()
                .map_err(infra)?,
            is_active: row.get("is_active"),
            projection_version: row.get("projection_version"),
            updated_at: row.get("updated_at"),
        })
    })
    .transpose()
}
