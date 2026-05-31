use chrono::NaiveDate;

use sqlx::{Sqlite, Transaction};

use crate::{
    domain::entity::business_date::BusinessDate,
    error::app_error::{conflict, AppResult},
    repository::sqlite::operational::business_date::business_date_repository::SqliteBusinessDateRepository,
};

pub async fn ensure_active_business_date_open(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<BusinessDate> {
    SqliteBusinessDateRepository::find_current_open(tx)
        .await?
        .ok_or_else(|| conflict("active business date is not open"))
}

pub async fn ensure_active_business_date(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<BusinessDate> {
    SqliteBusinessDateRepository::find_current_active(tx)
        .await?
        .ok_or_else(|| conflict("active business date not found"))
}

pub async fn ensure_active_business_date_closing(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<BusinessDate> {
    SqliteBusinessDateRepository::find_current_closing(tx)
        .await?
        .ok_or_else(|| conflict("active business date is not closing"))
}

pub fn ensure_operation_date_matches_business_date(
    operation_date: NaiveDate,
    business_date: &BusinessDate,
) -> AppResult<()> {
    if operation_date != business_date.business_date {
        return Err(conflict("operation date must match current business date"));
    }

    Ok(())
}
