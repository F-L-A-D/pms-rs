use chrono::NaiveDate;

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::aggregate::model::daily_room_class_kpi_aggregate_row::DailyRoomClassKpiAggregateRow,
    repository::sqlite::projection::aggregate::aggregate_daily_room_class_kpi::aggregate_daily_room_class_kpi,
};

pub async fn fetch_daily_room_class_kpi_aggregate_rows(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<Vec<DailyRoomClassKpiAggregateRow>> {
    aggregate_daily_room_class_kpi(tx, service_date).await
}
