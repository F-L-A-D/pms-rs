use chrono::Utc;

use uuid::Uuid;

use sqlx::{
    Sqlite, 
    Transaction,
};

use crate::error::app_error::AppResult;

use crate::projection::crm::guest_summary::
    GuestSummaryProjection;

use crate::usecase::timeline::
    get_guest_summary::get_guest_summary;

pub async fn materialize_guest_summary(
    tx: &mut Transaction<'_, Sqlite>,
    guest_id: Uuid,
) -> AppResult<GuestSummaryProjection> {

    let summary =
        get_guest_summary(
            tx,
            guest_id,
        )
        .await?;

    Ok(
        GuestSummaryProjection {
            guest_id,

            total_stays:
                summary.total_stays as i64,

            total_nights:
                summary.total_nights as i64,

            total_spending:
                summary.total_spending,

            last_stay_at:
                summary.last_stay_at,

            projection_version: 1,

            updated_at:
                Utc::now(),
        }
    )
}