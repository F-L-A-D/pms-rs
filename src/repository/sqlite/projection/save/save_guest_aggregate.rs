use sqlx::{
    Sqlite,
    Transaction,
};

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
        "save_guest_aggregate.sql"
    );

pub async fn save_guest_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
    aggregate: &GuestAggregate,
) -> AppResult<()> {

    sqlx::query(QUERY)
        .bind(aggregate.guest_id.to_string())
        .bind(aggregate.total_stays)
        .bind(aggregate.total_nights)
        .bind(aggregate.total_spending)
        .bind(aggregate.last_stay_at)
        .bind(aggregate.projection_version)
        .bind(aggregate.updated_at)
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

    Ok(())
}