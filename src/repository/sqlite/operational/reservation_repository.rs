use sqlx::{
    Row, 
    Sqlite, 
    Transaction,
};

use uuid::Uuid;

use crate::{
    error::app_error::{
        AppResult,
        infra,
    },

    domain::reservation::{
        Reservation, 
        ReservationStatus, 
        StayStatus,
    },

    repository::sqlite::operational::
        reservation_guest_relation_repository::
            SqliteReservationGuestRelationRepository,
};

pub struct SqliteReservationRepository;

impl SqliteReservationRepository {
    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        reservation: &Reservation,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO reservations (
                id,
                external_id,
                check_in,
                check_out,
                reservation_status,
                stay_status,
                room_class,
                room_id,
                created_at
            )
            VALUES (
                ?1,
                ?2,
                ?3,
                ?4,
                ?5,
                ?6,
                ?7,
                ?8,
                ?9
            )
            "#,
        )
        .bind(reservation.id.to_string())
        .bind(&reservation.external_id)
        .bind(reservation.check_in.to_string())
        .bind(reservation.check_out.to_string())
        .bind(match reservation.reservation_status {
            ReservationStatus::Active => "ACTIVE",
            ReservationStatus::Cancelled => "CANCELLED",
        })
        .bind(reservation.stay_status.as_ref().map(|s| match s {
            StayStatus::Confirmed => "CONFIRMED",
            StayStatus::CheckedIn => "CHECKED_IN",
            StayStatus::CheckedOut => "CHECKED_OUT",
        }))
        .bind(&reservation.room_class)
        .bind(&reservation.room_id)
        .bind(reservation.created_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn modify(
        tx: &mut Transaction<'_, Sqlite>,
        reservation: &Reservation,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE reservations
            SET
                external_id = ?1,
                check_in = ?2,
                check_out = ?3,
                reservation_status = ?4,
                stay_status = ?5,
                room_class = ?6,
                room_id = ?7
            WHERE id = ?8
            "#,
        )
        .bind(&reservation.external_id)
        .bind(reservation.check_in.to_string())
        .bind(reservation.check_out.to_string())
        .bind(match reservation.reservation_status {
            ReservationStatus::Active => "ACTIVE",

            ReservationStatus::Cancelled => "CANCELLED",
        })
        .bind(reservation.stay_status.as_ref().map(|s| match s {
            StayStatus::Confirmed => "CONFIRMED",

            StayStatus::CheckedIn => "CHECKED_IN",

            StayStatus::CheckedOut => "CHECKED_OUT",
        }))
        .bind(&reservation.room_class)
        .bind(&reservation.room_id)
        .bind(reservation.id.to_string())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: Uuid,
    ) -> AppResult<Option<Reservation>> {
        let row = sqlx::query(
            r#"
                SELECT
                    id,
                    external_id,
                    check_in,
                    check_out,
                    reservation_status,
                    stay_status,
                    room_class,
                    room_id,
                    created_at
                FROM reservations
                WHERE id = ?1
                "#,
        )
        .bind(id.to_string())
        .fetch_optional(&mut **tx)
        .await
        .map_err(infra)?;

        match row {
            Some(r) => {
                Ok(
                    Some(
                        Self::row_to_reservation(
                            tx,
                            &r,
                        )
                        .await?
                    )
                )
            }

            None => Ok(None),
        }
    }

    pub async fn find_by_guest_id(
        tx: &mut Transaction<'_, Sqlite>,
        guest_id: Uuid,
    ) -> AppResult<Vec<Reservation>> {
        let rows = sqlx::query(
            r#"
                SELECT DISTINCT
                    r.id,
                    r.external_id,
                    r.check_in,
                    r.check_out,
                    r.reservation_status,
                    r.stay_status,
                    r.room_class,
                    r.room_id,
                    r.created_at
                FROM reservations r
                INNER JOIN reservation_guest_relations rel
                    ON r.id = rel.reservation_id
                WHERE rel.guest_id = ?1
                "#,
        )
        .bind(guest_id.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        let mut reservations 
            = vec![];

        for row in rows.iter() {

            reservations.push(
                Self::row_to_reservation(
                    tx,
                    row,
                )
                .await?
            );
        }

        Ok(reservations)
    }

    pub async fn find_all(
        tx: &mut Transaction<'_, Sqlite>,
    ) -> AppResult<Vec<Reservation>> {

        let rows =
            sqlx::query(
                r#"
                SELECT id
                FROM reservations
                "#
            )
            .fetch_all(&mut **tx)
            .await
            .map_err(infra)?;

        let mut reservations 
            = vec![];

        for row in rows.iter() {

            reservations.push(
                Self::row_to_reservation(
                    tx,
                    row,
                )
                .await?
            );
        }

        Ok(reservations)
    }

    async fn row_to_reservation(
        tx: &mut Transaction<'_, Sqlite>,
        row: &sqlx::sqlite::SqliteRow,
    ) -> AppResult<Reservation> {

        let reservation_id: String =
            row.get("id");

        let reservation_uuid =
            Uuid::parse_str(&reservation_id)
                .map_err(infra)?;

        let participants =
            SqliteReservationGuestRelationRepository
                ::list_by_reservation_id(
                    tx,
                    &reservation_uuid,
                )
                .await?;

        Ok(
            Reservation {

                id: reservation_uuid,

                external_id:
                    row.get("external_id"),

                check_in:
                    row.get::<String, _>("check_in")
                        .parse()
                        .map_err(infra)?,

                check_out:
                    row.get::<String, _>("check_out")
                        .parse()
                        .map_err(infra)?,

                reservation_status:
                    match row
                        .get::<String, _>("reservation_status")
                        .as_str()
                    {
                        "ACTIVE" =>
                            ReservationStatus::Active,

                        "CANCELLED" =>
                            ReservationStatus::Cancelled,

                        _ =>
                            return Err(
                                infra(
                                    "invalid reservation status"
                                )
                            ),
                    },

                stay_status:
                    match row
                        .get::<Option<String>, _>("stay_status")
                    {
                        Some(v) =>
                            Some(
                                match v.as_str() {

                                    "CONFIRMED" =>
                                        StayStatus::Confirmed,

                                    "CHECKED_IN" =>
                                        StayStatus::CheckedIn,

                                    "CHECKED_OUT" =>
                                        StayStatus::CheckedOut,

                                    _ =>
                                        return Err(
                                            infra(
                                                "invalid stay status"
                                            )
                                        ),
                                }
                            ),

                        None => None,
                    },

                room_class:
                    row.get("room_class"),

                room_id:
                    row.get("room_id"),

                participants,

                created_at:
                    row.get::<String, _>("created_at")
                        .parse()
                        .map_err(infra)?,
            }
        )
    }
}
