use chrono::NaiveDate;

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        aggregate::materializer::materialize_housekeeping_daily_workload_aggregates::materialize_housekeeping_daily_workload_aggregates,
        execution::execution_trace::push_trace, topology::projection_node::ProjectionNode,
    },
    repository::sqlite::projection::save::{
        delete_housekeeping_daily_workload_aggregates_by_date::delete_housekeeping_daily_workload_aggregates_by_date,
        save_housekeeping_daily_workload_aggregate::save_housekeeping_daily_workload_aggregate,
    },
};

pub async fn refresh_housekeeping_daily_workload_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<()> {
    push_trace(ProjectionNode::HousekeepingDailyWorkloadAggregate);

    delete_housekeeping_daily_workload_aggregates_by_date(tx, service_date).await?;

    let aggregates = materialize_housekeeping_daily_workload_aggregates(tx, service_date).await?;

    for aggregate in aggregates {
        save_housekeeping_daily_workload_aggregate(tx, &aggregate).await?;
    }

    Ok(())
}
