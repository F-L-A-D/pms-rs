use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::aggregate::model::monthly_room_class_kpi_aggregate::MonthlyRoomClassKpiAggregate,
    repository::sqlite::projection::aggregate::get_monthly_room_class_kpi_aggregates_by_month::get_monthly_room_class_kpi_aggregates_by_month,
};

pub async fn fetch_monthly_room_class_kpi_aggregates_by_month(
    tx: &mut Transaction<'_, Sqlite>,
    year_month: &str,
) -> AppResult<Vec<MonthlyRoomClassKpiAggregate>> {
    get_monthly_room_class_kpi_aggregates_by_month(tx, year_month).await
}
