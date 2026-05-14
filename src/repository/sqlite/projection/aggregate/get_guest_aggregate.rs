use chrono::NaiveDate;

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

    projection::aggregate::model::
        guest_aggregate::GuestAggregate,
};

const QUERY: &str =
    include_str!(
        "get_guest_aggregate.sql"
    );

pub async fn get_guest_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
    guest_id: Uuid,
) -> AppResult<GuestAggregate> {

    let row =
        sqlx::query(QUERY)
            .bind(guest_id.to_string())
            .fetch_one(&mut **tx)
            .await
            .map_err(infra)?;

    Ok(
        GuestAggregate {
            guest_id:
                guest_id,

            total_stays:
                row.get("total_stays"),

            total_nights:
                row.get::<i64, _>("total_nights"),

            total_spending:
                row.get("total_spending"),

            last_stay_at:
                row.get::<Option<NaiveDate>, _>(
                    "last_stay_at"
                ),
            
            projection_version:
                row.get("projection_version"),

            updated_at:
                row.get("updated_at"),
        }
    )
}