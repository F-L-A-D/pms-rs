use chrono::NaiveDate;

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        aggregate::materializer::materialize_daily_room_class_kpi_aggregates::materialize_daily_room_class_kpi_aggregates,
        execution::execution_trace::push_trace, topology::projection_node::ProjectionNode,
    },
    repository::sqlite::projection::save::{
        delete_daily_room_class_kpi_aggregates_by_date::delete_daily_room_class_kpi_aggregates_by_date,
        save_daily_room_class_kpi_aggregate::save_daily_room_class_kpi_aggregate,
    },
};

pub async fn refresh_daily_room_class_kpi_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<()> {
    push_trace(ProjectionNode::DailyRoomClassKpiAggregate);

    delete_daily_room_class_kpi_aggregates_by_date(tx, service_date).await?;

    let aggregates = materialize_daily_room_class_kpi_aggregates(tx, service_date).await?;

    for aggregate in aggregates {
        save_daily_room_class_kpi_aggregate(tx, &aggregate).await?;
    }

    Ok(())
}
