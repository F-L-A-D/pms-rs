use sqlx::{Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    error::app_error::AppResult,
    projection::signal::model::semantic_activation::SemanticActivation,
    repository::sqlite::projection::signal::get_semantic_activation::get_semantic_activation,
};

pub async fn fetch_semantic_activation(
    tx: &mut Transaction<'_, Sqlite>,
    event_id: Uuid,
) -> AppResult<Option<SemanticActivation>> {
    get_semantic_activation(tx, event_id).await
}
