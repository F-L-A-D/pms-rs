use sqlx::{
    Row,
    Sqlite,
    Transaction,
};

use crate::projection::model::
    hotel_inventory_projection::
        HotelInventoryProjection;

pub struct SqliteHotelInventoryProjectionRepository;

impl SqliteHotelInventoryProjectionRepository {

    pub async fn upsert(
        tx: &mut Transaction<'_, Sqlite>,
        date: &str,
        reserved_rooms: i32,
    ) -> Result<(), String> {

        sqlx::query(
            r#"
            INSERT INTO hotel_inventory_projection (
                date,
                reserved_rooms
            )
            VALUES (?1, ?2)

            ON CONFLICT(date)
            DO UPDATE SET
                reserved_rooms = excluded.reserved_rooms
            "#
        )
        .bind(date)
        .bind(reserved_rooms)
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn find_by_date(
        tx: &mut Transaction<'_, Sqlite>,
        date: &str,
    ) -> Result<
        Option<HotelInventoryProjection>,
        String,
    > {

        let row =
            sqlx::query(
                r#"
                SELECT
                    date,
                    reserved_rooms
                FROM hotel_inventory_projection
                WHERE date = ?1
                "#
            )
            .bind(date)
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

        match row {

            Some(row) => Ok(Some(
                HotelInventoryProjection {
                    date:
                        row.get("date"),

                    reserved_rooms:
                        row.get("reserved_rooms"),
                }
            )),

            None => Ok(None),
        }
    }

    pub async fn delete_all(
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), String> {

        sqlx::query(
            r#"
            DELETE FROM hotel_inventory_projection
            "#
        )
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}