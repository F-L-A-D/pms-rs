use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::aggregate::model::monthly_hotel_kpi_aggregate::MonthlyHotelKpiAggregate,
    repository::sqlite::projection::aggregate::get_monthly_hotel_kpi_aggregate_by_month::get_monthly_hotel_kpi_aggregate_by_month,
};

pub async fn fetch_monthly_hotel_kpi_aggregate_by_month(
    tx: &mut Transaction<'_, Sqlite>,
    year_month: &str,
) -> AppResult<Option<MonthlyHotelKpiAggregate>> {
    get_monthly_hotel_kpi_aggregate_by_month(tx, year_month).await
}
