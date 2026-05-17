use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        aggregate::rebuild::rebuild_daily_hotel_kpi_aggregate::rebuild_daily_hotel_kpi_aggregate,
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::projection::clear::clear_projection_table::clear_projection_table,
};

pub async fn execute_daily_hotel_kpi_aggregate_rebuild(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()> {
    clear_projection_table(tx, ProjectionNode::DailyHotelKpiAggregate).await?;

    rebuild_daily_hotel_kpi_aggregate(tx).await
}
