use chrono::Utc;

use sqlx::{Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    error::app_error::AppResult,
    projection::signal::{
        access::fetch_guest_activity_source::fetch_guest_activity_source,
        model::guest_activity_signal::GuestActivitySignal,
    },
};

pub async fn materialize_guest_activity_signal(
    tx: &mut Transaction<'_, Sqlite>,
    guest_id: Uuid,
) -> AppResult<GuestActivitySignal> {
    let source = fetch_guest_activity_source(tx, guest_id).await?;

    Ok(GuestActivitySignal {
        guest_id: guest_id,

        is_active: source.guest_id == guest_id,

        projection_version: 1,

        updated_at: Utc::now(),
    })
}
