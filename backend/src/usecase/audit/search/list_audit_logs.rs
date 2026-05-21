use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::semantic::operational_audit_log::OperationalAuditLog,
    error::app_error::{infra, AppResult},
    repository::sqlite::operational::operation::operational_audit_log_repository::SqliteOperationalAuditLogRepository,
};

pub async fn execute(
    db: &Db,
    aggregate_type: String,
    aggregate_id: Uuid,
) -> AppResult<Vec<OperationalAuditLog>> {
    let mut tx = db.begin_tx().await;

    let result = SqliteOperationalAuditLogRepository::list_by_aggregate(
        &mut tx,
        &aggregate_type,
        aggregate_id,
    )
    .await;

    match result {
        Ok(logs) => {
            tx.commit().await.map_err(infra)?;

            Ok(logs)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}
