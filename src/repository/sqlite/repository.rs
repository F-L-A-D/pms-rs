use sqlx::{Sqlite, Transaction, Row};
use crate::domain::reservation::{Reservation, ReservationStatus};

pub struct SqliteReservationRepository;

impl SqliteReservationRepository {

    pub async fn save_tx(
        tx: &mut Transaction<'_, Sqlite>,
        reservation: &Reservation,
    ) -> Result<(), String> {

        // UPDATE
        let result = sqlx::query(
            r#"
            UPDATE reservations
            SET check_in = ?1,
                check_out = ?2,
                status = ?3
            WHERE id = ?4
            "#
        )
        .bind(reservation.check_in.to_string())
        .bind(reservation.check_out.to_string())
        .bind(format!("{:?}", reservation.status))
        .bind(&reservation.id)
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        // INSERT if not exists
        if result.rows_affected() == 0 {
            sqlx::query(
                r#"
                INSERT INTO reservations
                (id, check_in, check_out, status, created_at, channel)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                "#
            )
            .bind(&reservation.id)
            .bind(reservation.check_in.to_string())
            .bind(reservation.check_out.to_string())
            .bind(format!("{:?}", reservation.status))
            .bind(chrono::Utc::now().to_string())
            .bind("direct")
            .execute(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    pub async fn find_by_id_tx(
        tx: &mut Transaction<'_, Sqlite>,
        id: &str,
    ) -> Result<Option<Reservation>, String> {

        let row = sqlx::query(
            r#"
            SELECT id, check_in, check_out, status
            FROM reservations
            WHERE id = ?1
            "#
        )
        .bind(id)
        .fetch_optional(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            Ok(Some(Reservation {
                id: r.get("id"),
                check_in: r.get::<String, _>("check_in").parse().unwrap(),
                check_out: r.get::<String, _>("check_out").parse().unwrap(),
                status: match r.get::<String, _>("status").as_str() {
                    "Cancelled" => ReservationStatus::Cancelled,
                    _ => ReservationStatus::Active,
                },
            }))
        } else {
            Ok(None)
        }
    }
}