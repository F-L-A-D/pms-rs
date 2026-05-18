use chrono::NaiveDate;

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        aggregate::materializer::materialize_daily_hotel_kpi_aggregate::materialize_daily_hotel_kpi_aggregate,
        execution::execution_trace::push_trace, topology::projection_node::ProjectionNode,
    },
    repository::sqlite::projection::save::{
        delete_daily_hotel_kpi_aggregate_by_date::delete_daily_hotel_kpi_aggregate_by_date,
        save_daily_hotel_kpi_aggregate::save_daily_hotel_kpi_aggregate,
    },
};

pub async fn refresh_daily_hotel_kpi_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<()> {
    push_trace(ProjectionNode::DailyHotelKpiAggregate);

    delete_daily_hotel_kpi_aggregate_by_date(tx, service_date).await?;

    let aggregate = materialize_daily_hotel_kpi_aggregate(tx, service_date).await?;

    save_daily_hotel_kpi_aggregate(tx, &aggregate).await
}
