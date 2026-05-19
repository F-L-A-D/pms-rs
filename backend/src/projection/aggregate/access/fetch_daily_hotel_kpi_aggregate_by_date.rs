use chrono::NaiveDate;

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::aggregate::model::daily_hotel_kpi_aggregate::DailyHotelKpiAggregate,
    repository::sqlite::projection::aggregate::get_daily_hotel_kpi_aggregate_by_date::get_daily_hotel_kpi_aggregate_by_date,
};

pub async fn fetch_daily_hotel_kpi_aggregate_by_date(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<Option<DailyHotelKpiAggregate>> {
    get_daily_hotel_kpi_aggregate_by_date(tx, service_date).await
}
