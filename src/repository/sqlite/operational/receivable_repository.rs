use sqlx::{Sqlite, Transaction};

use crate::domain::receivable::Receivable;


pub struct SqliteReceivableRepository;

impl SqliteReceivableRepository {

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        receivable: &Receivable,
    ) -> Result<(), String> {

        sqlx::query(
            r#"
            INSERT OR REPLACE INTO receivables (
                id,
                invoice_id,
                outstanding_amount,
                status
            )
            VALUES (?1, ?2, ?3, ?4)
            "#
        )
        .bind(receivable.id.to_string())
        .bind(receivable.invoice_id.to_string())
        .bind(receivable.outstanding_amount)
        .bind(format!("{:?}", receivable.status))
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}