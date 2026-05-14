use sqlx::{
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    error::app_error::AppResult,

    projection::signal::model::
        guest_activity_signal::
            GuestActivitySignal,

    repository::sqlite::projection::signal::
        get_guest_activity::get_guest_activity,
};

pub async fn fetch_guest_activity_signal(
    tx: &mut Transaction<'_, Sqlite>,
    guest_id: Uuid,
) -> AppResult<GuestActivitySignal> {

    get_guest_activity(
        tx,
        guest_id,
    )
    .await
}