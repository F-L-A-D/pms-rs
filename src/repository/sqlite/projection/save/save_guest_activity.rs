use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::{infra, AppResult},
    projection::signal::model::guest_activity_signal::GuestActivitySignal,
};

const QUERY: &str = include_str!("save_guest_activity.sql");

pub async fn save_guest_activity(
    tx: &mut Transaction<'_, Sqlite>,
    signal: &GuestActivitySignal,
) -> AppResult<()> {
    sqlx::query(QUERY)
        .bind(signal.guest_id.to_string())
        .bind(signal.is_active)
        .bind(signal.projection_version)
        .bind(signal.updated_at)
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

    Ok(())
}
