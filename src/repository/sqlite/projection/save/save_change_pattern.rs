use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::{infra, AppResult},
    projection::signal::model::change_pattern::ChangePattern,
};

pub async fn save_change_pattern(
    tx: &mut Transaction<'_, Sqlite>,
    pattern: &ChangePattern,
) -> AppResult<()> {
    sqlx::query(
        r#"
        INSERT INTO change_patterns (
            event_id,
            pattern_type,
            operation_type,
            changed_fields_json,
            projection_version,
            updated_at
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        ON CONFLICT(event_id)
        DO UPDATE SET
            pattern_type = excluded.pattern_type,
            operation_type = excluded.operation_type,
            changed_fields_json = excluded.changed_fields_json,
            projection_version = excluded.projection_version,
            updated_at = excluded.updated_at
        "#,
    )
    .bind(pattern.event_id.to_string())
    .bind(pattern.pattern_type.to_snake())
    .bind(pattern.operation_type.to_snake())
    .bind(&pattern.changed_fields_json)
    .bind(pattern.projection_version)
    .bind(pattern.updated_at)
    .execute(&mut **tx)
    .await
    .map_err(infra)?;

    Ok(())
}
