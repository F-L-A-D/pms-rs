use sqlx::{Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    error::app_error::AppResult, projection::signal::model::change_pattern::ChangePattern,
    repository::sqlite::projection::signal::get_change_pattern::get_change_pattern,
};

pub async fn fetch_change_pattern(
    tx: &mut Transaction<'_, Sqlite>,
    event_id: Uuid,
) -> AppResult<Option<ChangePattern>> {
    get_change_pattern(tx, event_id).await
}
