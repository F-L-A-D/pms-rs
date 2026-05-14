use sqlx::{
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    error::app_error::AppResult,

    projection::signal::model::
        guest_activity_source::GuestActivitySource,

    repository::sqlite::projection::aggregate::
        get_guest_aggregate::get_guest_aggregate,
};

pub async fn fetch_guest_activity_source(
    tx: &mut Transaction<'_, Sqlite>,
    guest_id: Uuid,
) -> AppResult<GuestActivitySource> {

    let guest = 
        get_guest_aggregate(
            tx,
            guest_id,
        )
        .await?;
        
    let source =
        GuestActivitySource {
            guest_id:
                guest.guest_id,
        };

    Ok(source)
}