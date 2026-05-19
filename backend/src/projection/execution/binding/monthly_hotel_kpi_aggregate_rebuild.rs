use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        aggregate::rebuild::rebuild_monthly_hotel_kpi_aggregate::rebuild_monthly_hotel_kpi_aggregate,
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::projection::clear::clear_projection_table::clear_projection_table,
};

pub async fn execute_monthly_hotel_kpi_aggregate_rebuild(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()> {
    clear_projection_table(tx, ProjectionNode::MonthlyHotelKpiAggregate).await?;

    rebuild_monthly_hotel_kpi_aggregate(tx).await
}
