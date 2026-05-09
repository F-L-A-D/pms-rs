use sqlx::{
    Row,
    Sqlite,
    Transaction,
};

use crate::projection::materializer::
    inventory_materializer::
        InventoryProjectionRow;

pub struct SqliteInventoryProjectionRepository;

impl SqliteInventoryProjectionRepository {

    pub async fn add(
        tx: &mut Transaction<'_, Sqlite>,
        date: &str,
        room_class: &str,
        delta: i32,
    ) -> Result<(), String> {

        let row =
            sqlx::query(
                r#"
                SELECT reserved_rooms
                FROM inventory_projection
                WHERE date = ?1
                  AND room_class = ?2
                "#
            )
            .bind(date)
            .bind(room_class)
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| {
                e.to_string()
            })?;

        if let Some(row) = row {

            let reserved: i32 =
                row.get(0);

            let new_value =
                reserved + delta;

            if new_value < 0 {

                return Err(
                    "inventory projection underflow"
                        .into()
                );
            }

            sqlx::query(
                r#"
                UPDATE inventory_projection
                SET reserved_rooms = ?1
                WHERE date = ?2
                  AND room_class = ?3
                "#
            )
            .bind(new_value)
            .bind(date)
            .bind(room_class)
            .execute(&mut **tx)
            .await
            .map_err(|e| {
                e.to_string()
            })?;

        } else {

            if delta < 0 {

                return Err(
                    "inventory projection missing during decrement"
                        .into()
                );
            }

            sqlx::query(
                r#"
                INSERT INTO inventory_projection (
                    date,
                    room_class,
                    reserved_rooms
                )
                VALUES (
                    ?1,
                    ?2,
                    ?3
                )
                "#
            )
            .bind(date)
            .bind(room_class)
            .bind(delta)
            .execute(&mut **tx)
            .await
            .map_err(|e| {
                e.to_string()
            })?;
        }

        Ok(())
    }

    pub async fn delete_all(
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), String> {

        sqlx::query(
            r#"
            DELETE FROM inventory_projection
            "#
        )
        .execute(&mut **tx)
        .await
        .map_err(|e| {
            e.to_string()
        })?;

        Ok(())
    }

    pub async fn list_all(
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<
        Vec<InventoryProjectionRow>,
        String,
    > {

        let rows =
            sqlx::query(
                r#"
                SELECT
                    date,
                    room_class,
                    reserved_rooms
                FROM inventory_projection
                ORDER BY
                    date,
                    room_class
                "#
            )
            .fetch_all(&mut **tx)
            .await
            .map_err(|e| {
                e.to_string()
            })?;

        Ok(
            rows
                .into_iter()
                .map(|row| {
                    InventoryProjectionRow {
                        date:
                            row.get("date"),

                        room_class:
                            row.get("room_class"),

                        reserved_rooms:
                            row.get("reserved_rooms"),
                    }
                })
                .collect()
        )
    }

    pub async fn find_by_date_and_room_class(
        tx: &mut Transaction<'_, Sqlite>,
        date: &str,
        room_class: &str,
    ) -> Result<
        Option<InventoryProjectionRow>,
        String,
    > {

        let row =
            sqlx::query(
                r#"
                SELECT
                    date,
                    room_class,
                    reserved_rooms
                FROM inventory_projection
                WHERE
                    date = ?1
                    AND room_class = ?2
                "#
            )
            .bind(date)
            .bind(room_class)
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| {
                e.to_string()
            })?;

        match row {

            Some(row) => {

                Ok(
                    Some(
                        InventoryProjectionRow {
                            date:
                                row.get("date"),

                            room_class:
                                row.get("room_class"),

                            reserved_rooms:
                                row.get("reserved_rooms"),
                        }
                    )
                )
            }

            None => Ok(None),
        }
    }
}