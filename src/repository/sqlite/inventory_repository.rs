use sqlx::{Sqlite, Transaction};

pub struct SqliteInventoryRepository;

impl SqliteInventoryRepository {

    pub async fn add_tx(
        tx: &mut Transaction<'_, Sqlite>,
        date: &str,
        delta: i32,
        total_rooms: i32,
    ) -> Result<(), String> {

        let result = sqlx::query(
            r#"
            UPDATE inventory
            SET reserved_rooms = reserved_rooms + ?1
            WHERE date = ?2
            "#
        )
        .bind(delta)
        .bind(date)
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        if result.rows_affected() == 0 {
            sqlx::query(
                r#"
                INSERT INTO inventory (date, total_rooms, reserved_rooms)
                VALUES (?1, ?2, ?3)
                "#
            )
            .bind(date)
            .bind(total_rooms)
            .bind(delta.max(0))
            .execute(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}