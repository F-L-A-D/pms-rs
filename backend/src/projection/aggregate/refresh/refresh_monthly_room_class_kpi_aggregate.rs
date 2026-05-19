use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        aggregate::materializer::materialize_monthly_room_class_kpi_aggregates::materialize_monthly_room_class_kpi_aggregates,
        execution::execution_trace::push_trace, topology::projection_node::ProjectionNode,
    },
    repository::sqlite::projection::save::{
        delete_monthly_room_class_kpi_aggregates_by_month::delete_monthly_room_class_kpi_aggregates_by_month,
        save_monthly_room_class_kpi_aggregate::save_monthly_room_class_kpi_aggregate,
    },
};

pub async fn refresh_monthly_room_class_kpi_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
    year_month: &str,
) -> AppResult<()> {
    push_trace(ProjectionNode::MonthlyRoomClassKpiAggregate);

    delete_monthly_room_class_kpi_aggregates_by_month(tx, year_month).await?;

    let aggregates = materialize_monthly_room_class_kpi_aggregates(tx, year_month).await?;

    for aggregate in aggregates {
        save_monthly_room_class_kpi_aggregate(tx, &aggregate).await?;
    }

    Ok(())
}
