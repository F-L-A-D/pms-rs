use chrono::NaiveDate;

use rust_decimal::Decimal;

use crate::{
    api::dto::billing::response::receivable_aging_response::ReceivableAgingResponse,
    db::connection::Db,
    domain::entity::receivable::ReceivableStatus,
    error::app_error::{infra, AppResult},
    repository::sqlite::operational::receivable_repository::SqliteReceivableRepository,
};

pub async fn execute(db: &Db, as_of_date: NaiveDate) -> AppResult<ReceivableAgingResponse> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let receivables = SqliteReceivableRepository::list_all(&mut tx).await?;

        let mut response = ReceivableAgingResponse {
            current_amount: Decimal::ZERO,
            overdue_1_30_amount: Decimal::ZERO,
            overdue_31_60_amount: Decimal::ZERO,
            overdue_61_90_amount: Decimal::ZERO,
            overdue_90_plus_amount: Decimal::ZERO,
            total_open_amount: Decimal::ZERO,
        };

        for receivable in receivables
            .into_iter()
            .filter(|receivable| receivable.status == ReceivableStatus::Open)
        {
            response.total_open_amount += receivable.outstanding_amount;

            let overdue_days = (as_of_date - receivable.due_date).num_days();

            match overdue_days {
                days if days <= 0 => response.current_amount += receivable.outstanding_amount,
                1..=30 => response.overdue_1_30_amount += receivable.outstanding_amount,
                31..=60 => response.overdue_31_60_amount += receivable.outstanding_amount,
                61..=90 => response.overdue_61_90_amount += receivable.outstanding_amount,
                _ => response.overdue_90_plus_amount += receivable.outstanding_amount,
            }
        }

        Ok(response)
    }
    .await;

    let _ = tx.rollback().await.map_err(infra);

    result
}
