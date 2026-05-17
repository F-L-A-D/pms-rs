use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        aggregate::materializer::materialize_monthly_hotel_kpi_aggregate::materialize_monthly_hotel_kpi_aggregate,
        execution::execution_trace::push_trace, topology::projection_node::ProjectionNode,
    },
    repository::sqlite::projection::save::{
        delete_monthly_hotel_kpi_aggregate_by_month::delete_monthly_hotel_kpi_aggregate_by_month,
        save_monthly_hotel_kpi_aggregate::save_monthly_hotel_kpi_aggregate,
    },
};

pub async fn refresh_monthly_hotel_kpi_aggregate(
    tx: &mut Transaction<'_, Sqlite>,
    year_month: &str,
) -> AppResult<()> {
    push_trace(ProjectionNode::MonthlyHotelKpiAggregate);

    delete_monthly_hotel_kpi_aggregate_by_month(tx, year_month).await?;

    let aggregate = materialize_monthly_hotel_kpi_aggregate(tx, year_month).await?;

    save_monthly_hotel_kpi_aggregate(tx, &aggregate).await
}
