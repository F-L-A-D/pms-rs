use sqlx::{Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    error::app_error::AppResult, projection::signal::model::confidence_profile::ConfidenceProfile,
    repository::sqlite::projection::signal::get_confidence_profile::get_confidence_profile,
};

pub async fn fetch_confidence_profile(
    tx: &mut Transaction<'_, Sqlite>,
    event_id: Uuid,
) -> AppResult<Option<ConfidenceProfile>> {
    get_confidence_profile(tx, event_id).await
}
