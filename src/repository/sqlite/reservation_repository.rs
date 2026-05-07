use sqlx::{
    Row,
    Sqlite,
    Transaction,
};

use crate::domain::reservation::{
    Reservation,
    ReservationStatus,
    StayStatus,
};

pub struct SqliteReservationRepository;

impl SqliteReservationRepository {

    pub async fn save(
        tx: &mut Transaction<'_, Sqlite>,
        reservation: &Reservation,
    ) -> Result<(), String> {

        sqlx::query(
            r#"
            INSERT INTO reservations (
                id,
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
                ?8
            )
            "#
        )
        .bind(&reservation.id)
        .bind(reservation.check_in.to_string())
        .bind(reservation.check_out.to_string())
        .bind(
            match reservation.reservation_status {
                ReservationStatus::Active =>
                    "ACTIVE",
                ReservationStatus::Cancelled =>
                    "CANCELLED",
            }
        )
        .bind(
            reservation
                .stay_status
                .as_ref()
                .map(|s| {
                    match s {
                        StayStatus::Confirmed =>
                            "CONFIRMED",
                        StayStatus::CheckedIn =>
                            "CHECKED_IN",
                        StayStatus::CheckedOut =>
                            "CHECKED_OUT",
                    }
                })
        )
        .bind(&reservation.room_class)
        .bind(&reservation.room_id)
        .bind(
            reservation
                .created_at
                .to_rfc3339()
        )
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn update(
        tx: &mut Transaction<'_, Sqlite>,
        reservation: &Reservation,
    ) -> Result<(), String> {

        sqlx::query(
            r#"
            UPDATE reservations
            SET
                check_in = ?1,
                check_out = ?2,
                reservation_status = ?3,
                stay_status = ?4,
                room_class = ?5,
                room_id = ?6
            WHERE id = ?7
            "#
        )
        .bind(
            reservation
                .check_in
                .to_string()
        )
        .bind(
            reservation
                .check_out
                .to_string()
        )
        .bind(
            match reservation.reservation_status {

                ReservationStatus::Active =>
                    "ACTIVE",

                ReservationStatus::Cancelled =>
                    "CANCELLED",
            }
        )
        .bind(
            reservation
                .stay_status
                .as_ref()
                .map(|s| {

                    match s {

                        StayStatus::Confirmed =>
                            "CONFIRMED",

                        StayStatus::CheckedIn =>
                            "CHECKED_IN",

                        StayStatus::CheckedOut =>
                            "CHECKED_OUT",
                    }
                })
        )
        .bind(
            &reservation.room_class
        )
        .bind(
            &reservation.room_id
        )
        .bind(
            &reservation.id
        )
        .execute(&mut **tx)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn find_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        id: &str,
    ) -> Result<Option<Reservation>, String> {

        let row =
            sqlx::query(
                r#"
                SELECT
                    id,
                    check_in,
                    check_out,
                    reservation_status,
                    stay_status,
                    room_class,
                    room_id,
                    created_at
                FROM reservations
                WHERE id = ?1
                "#
            )
            .bind(id)
            .fetch_optional(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

        match row {

            Some(r) => {

                let participants =
                    crate::repository::sqlite::
                        reservation_guest_relation_repository::
                        SqliteReservationGuestRelationRepository
                            ::list_by_reservation_id(
                                tx,
                                id,
                            )
                            .await?;

                Ok(
                    Some(
                        Reservation {
                            id: r.get("id"),
                            check_in: r
                                .get::<String, _>("check_in")
                                .parse()
                                .unwrap(),
                            check_out: r
                                .get::<String, _>("check_out")
                                .parse()
                                .unwrap(),
                            reservation_status:
                                match r.get::<String, _>(
                                    "reservation_status"
                                ).as_str() {

                                    "ACTIVE" =>
                                        ReservationStatus::Active,

                                    "CANCELLED" =>
                                        ReservationStatus::Cancelled,

                                    _ =>
                                        return Err(
                                            "invalid reservation_status"
                                                .into()
                                        ),
                                },
                            stay_status:
                                match r
                                    .get::<Option<String>, _>(
                                        "stay_status"
                                    ) {

                                    Some(v) => {
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
                                                        "invalid stay_status"
                                                            .into()
                                                    ),
                                            }
                                        )
                                    }

                                    None => None,
                                },
                            room_class:
                                r.get("room_class"),
                            room_id:
                                r.get(  "room_id"),
                            participants,

                            created_at:
                                r.get::<String, _>(
                                    "created_at"
                                )
                                .parse()
                                .unwrap(),
                        }
                    )
                )
            }

            None => Ok(None),
        }
    }

    pub async fn find_by_guest_id(
        tx: &mut Transaction<'_, Sqlite>,
        guest_id: &str,
    ) -> Result<Vec<Reservation>, String> {

        let rows =
            sqlx::query(
                r#"
                SELECT DISTINCT
                    r.id,
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
                "#
            )
            .bind(guest_id)
            .fetch_all(&mut **tx)
            .await
            .map_err(|e| e.to_string())?;

        let mut reservations = vec![];

        for row in rows {

            let reservation_id: String =
                row.get("id");

            let participants =
                crate::repository::sqlite::
                    reservation_guest_relation_repository::
                    SqliteReservationGuestRelationRepository
                        ::list_by_reservation_id(
                            tx,
                            &reservation_id,
                        )
                        .await?;

            reservations.push(
                Reservation {
                    id: reservation_id,
                    check_in:
                        row.get::<String, _>(
                            "check_in"
                        )
                        .parse()
                        .unwrap(),

                    check_out:
                        row.get::<String, _>(
                            "check_out"
                        )
                        .parse()
                        .unwrap(),

                    reservation_status:
                        match row
                            .get::<String, _>(
                                "reservation_status"
                            )
                            .as_str() {

                            "ACTIVE" =>
                                ReservationStatus::Active,

                            "CANCELLED" =>
                                ReservationStatus::Cancelled,

                            _ =>
                                return Err(
                                    "invalid reservation_status"
                                        .into()
                                ),
                        },

                    stay_status:
                        match row
                            .get::<Option<String>, _>(
                                "stay_status"
                            ) {

                            Some(v) => {
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
                                                "invalid stay_status"
                                                    .into()
                                            ),
                                    }
                                )
                            }

                            None => None,
                        },

                    room_class:
                        row.get("room_class"),

                    room_id:
                        row.get("room_id"),

                    participants,

                    created_at:
                        row.get::<String, _>(
                            "created_at"
                        )
                        .parse()
                        .unwrap(),
                }
            );
        }

        Ok(reservations)
    }
}