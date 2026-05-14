use sqlx::{
    Row,
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    error::app_error::{
        AppResult,
        infra,
    },

    projection::signal::model::
        guest_activity_signal::
            GuestActivitySignal,
};

const QUERY: &str =
    include_str!(
        "get_guest_activity.sql"
    );

pub async fn get_guest_activity(
    tx: &mut Transaction<'_, Sqlite>,
    guest_id: Uuid,
) -> AppResult<GuestActivitySignal>
{
    let row =
        sqlx::query(QUERY)
            .bind(guest_id.to_string())
            .fetch_one(&mut **tx)
            .await
            .map_err(infra)?;

    Ok(
        GuestActivitySignal {
            guest_id:
                guest_id,
            
            is_active:
                row.get("is_active"),

            projection_version:
                row.get("projection_version"),

            updated_at:
                row.get("updated_at"),
        }
    )
}