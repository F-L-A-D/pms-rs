use chrono::NaiveDate;

use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::aggregate::model::daily_room_class_kpi_aggregate::DailyRoomClassKpiAggregate,
    repository::sqlite::projection::aggregate::get_daily_room_class_kpi_aggregates_by_date::get_daily_room_class_kpi_aggregates_by_date,
};

pub async fn fetch_daily_room_class_kpi_aggregates_by_date(
    tx: &mut Transaction<'_, Sqlite>,
    service_date: NaiveDate,
) -> AppResult<Vec<DailyRoomClassKpiAggregate>> {
    get_daily_room_class_kpi_aggregates_by_date(tx, service_date).await
}
