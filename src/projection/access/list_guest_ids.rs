use sqlx::{Sqlite, Transaction};

use uuid::Uuid;

use crate::error::app_error::{infra, AppResult};

pub async fn list_guest_ids(tx: &mut Transaction<'_, Sqlite>) -> AppResult<Vec<Uuid>> {
    let rows = sqlx::query_scalar::<_, String>(
        r#"
            SELECT id
            FROM guests
            "#,
    )
    .fetch_all(&mut **tx)
    .await
    .map_err(infra)?;

    rows.into_iter()
        .map(|id| Uuid::parse_str(&id).map_err(infra))
        .collect()
}
