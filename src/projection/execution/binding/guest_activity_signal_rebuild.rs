use sqlx::{
    Sqlite,
    Transaction,
};

use crate::{
    error::app_error::AppResult,

    projection::signal::rebuild::
        rebuild_guest_activity_signal::
            rebuild_guest_activity_signal,
};

pub async fn execute_guest_activity_signal_rebuild(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()>
{
    rebuild_guest_activity_signal(
        tx,
    )
    .await
}