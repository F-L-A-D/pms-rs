use sqlx::{Sqlite, Transaction, Row};

pub struct SqliteInventoryRepository;

impl SqliteInventoryRepository {

    pub async fn add_tx(
        tx: &mut Transaction<'_, Sqlite>,
        date: &str,
        delta: i32,
        total_rooms: i32,
    ) -> Result<(), String> {

        let row = sqlx::query(
            r#"
            SELECT reserved_rooms
            FROM inventory
            WHERE date = ?1
            "#
        )
        .bind(date)
        .fetch_optional(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(row) = row {
            let reserved: i32 = row.get(0);
            let new_value = reserved + delta;

            if new_value < 0 {
                return Err("inventory cannot be negative".into());
            }

            sqlx::query(
                r#"
                UPDATE inventory
                SET reserved_rooms = ?1
                WHERE date = ?2
                "#
            )
            .bind(new_value)
            .bind(date)
            .execute(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;
        } else {
            if delta < 0 {
                return Err("cannot reduce non-existing inventory".into());
            }

            sqlx::query(
                r#"
                INSERT INTO inventory (date, total_rooms, reserved_rooms)
                VALUES (?1, ?2, ?3)
                "#
            )
            .bind(date)
            .bind(total_rooms)
            .bind(delta)
            .execute(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}