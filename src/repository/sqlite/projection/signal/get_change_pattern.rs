use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::semantic::{
        operation_change_event::OperationType, semantic_activation::ChangePatternType,
    },
    error::app_error::{infra, AppResult},
    projection::signal::model::change_pattern::ChangePattern,
};

pub async fn get_change_pattern(
    tx: &mut Transaction<'_, Sqlite>,
    event_id: Uuid,
) -> AppResult<Option<ChangePattern>> {
    let row = sqlx::query(
        r#"
        SELECT *
        FROM change_patterns
        WHERE event_id = ?1
        "#,
    )
    .bind(event_id.to_string())
    .fetch_optional(&mut **tx)
    .await
    .map_err(infra)?;

    row.map(|row| {
        Ok(ChangePattern {
            event_id,
            pattern_type: ChangePatternType::from_snake(
                row.get::<String, _>("pattern_type").as_str(),
            )
            .ok_or_else(|| infra("invalid change pattern type"))?,
            operation_type: OperationType::from_snake(
                row.get::<String, _>("operation_type").as_str(),
            )
            .ok_or_else(|| infra("invalid operation type"))?,
            changed_fields_json: row.get("changed_fields_json"),
            projection_version: row.get("projection_version"),
            updated_at: row.get("updated_at"),
        })
    })
    .transpose()
}
