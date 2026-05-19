use sqlx::{Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    domain::{
        entity::reservation::{Reservation, ReservationStatus, StayStatus},
        semantic::reservation_booking::ReservationBookingChannel,
    },
    error::app_error::{conflict, infra, AppResult},
    repository::sqlite::operational::{
        reservation_daily_revenue_allocation_repository::SqliteReservationDailyRevenueAllocationRepository,
        reservation_daily_stay_detail_repository::SqliteReservationDailyStayDetailRepository,
        reservation_guest_relation_repository::SqliteReservationGuestRelationRepository,
        reservation_package_breakdown_repository::SqliteReservationPackageBreakdownRepository,
    },
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
                booking_channel,
                plan_code,
                version,
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
                ?9,
                ?10,
                ?11,
                ?12
            )
            "#,
        )
        .bind(reservation.id.to_string())
        .bind(&reservation.external_id)
        .bind(reservation.check_in.to_string())
        .bind(reservation.check_out.to_string())
        .bind(reservation.reservation_status.to_snake())
        .bind(reservation.stay_status.as_ref().map(StayStatus::to_snake))
        .bind(&reservation.room_class)
        .bind(reservation.room_id.map(|id| id.to_string()))
        .bind(reservation.booking_channel.to_snake())
        .bind(&reservation.plan_code)
        .bind(reservation.version)
        .bind(reservation.created_at.to_rfc3339())
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        Ok(())
    }

    pub async fn modify(
        tx: &mut Transaction<'_, Sqlite>,
        reservation: &mut Reservation,
    ) -> AppResult<()> {
        Self::modify_with_expected_version(tx, reservation, reservation.version).await
    }

    pub async fn modify_with_expected_version(
        tx: &mut Transaction<'_, Sqlite>,
        reservation: &mut Reservation,
        expected_version: i64,
    ) -> AppResult<()> {
        let next_version = expected_version + 1;

        let result = sqlx::query(
            r#"
            UPDATE reservations
            SET
                external_id = ?1,
                check_in = ?2,
                check_out = ?3,
                reservation_status = ?4,
                stay_status = ?5,
                room_class = ?6,
                room_id = ?7,
                booking_channel = ?8,
                plan_code = ?9,
                version = ?10
            WHERE id = ?11
              AND version = ?12
            "#,
        )
        .bind(&reservation.external_id)
        .bind(reservation.check_in.to_string())
        .bind(reservation.check_out.to_string())
        .bind(reservation.reservation_status.to_snake())
        .bind(reservation.stay_status.as_ref().map(StayStatus::to_snake))
        .bind(&reservation.room_class)
        .bind(reservation.room_id.map(|id| id.to_string()))
        .bind(reservation.booking_channel.to_snake())
        .bind(&reservation.plan_code)
        .bind(next_version)
        .bind(reservation.id.to_string())
        .bind(expected_version)
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

        if result.rows_affected() == 0 {
            return Err(conflict(format!(
                "reservation version conflict: expected {expected_version}"
            )));
        }

        reservation.version = next_version;

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
                    booking_channel,
                    plan_code,
                    version,
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
            Some(r) => Ok(Some(Self::row_to_reservation(tx, &r).await?)),

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
                    r.booking_channel,
                    r.plan_code,
                    r.version,
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

        let mut reservations = vec![];

        for row in rows.iter() {
            reservations.push(Self::row_to_reservation(tx, row).await?);
        }

        Ok(reservations)
    }

    pub async fn find_all(tx: &mut Transaction<'_, Sqlite>) -> AppResult<Vec<Reservation>> {
        let rows = sqlx::query(
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
                    booking_channel,
                    plan_code,
                    version,
                    created_at
                FROM reservations
                ORDER BY check_in DESC
                "#,
        )
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        let mut reservations = vec![];

        for row in rows.iter() {
            reservations.push(Self::row_to_reservation(tx, row).await?);
        }

        Ok(reservations)
    }

    async fn row_to_reservation(
        tx: &mut Transaction<'_, Sqlite>,
        row: &sqlx::sqlite::SqliteRow,
    ) -> AppResult<Reservation> {
        let reservation_id: String = row.get("id");

        let reservation_uuid = Uuid::parse_str(&reservation_id).map_err(infra)?;

        let participants =
            SqliteReservationGuestRelationRepository::list_by_reservation_id(tx, &reservation_uuid)
                .await?;

        let package_breakdowns =
            SqliteReservationPackageBreakdownRepository::list_by_reservation_id(
                tx,
                reservation_uuid,
            )
            .await?;

        let daily_stay_details =
            SqliteReservationDailyStayDetailRepository::list_by_reservation_id(
                tx,
                reservation_uuid,
            )
            .await?;

        let daily_revenue_allocations =
            SqliteReservationDailyRevenueAllocationRepository::list_by_reservation_id(
                tx,
                reservation_uuid,
            )
            .await?;

        Ok(Reservation {
            id: reservation_uuid,

            external_id: row.get("external_id"),

            check_in: row.get::<String, _>("check_in").parse().map_err(infra)?,

            check_out: row.get::<String, _>("check_out").parse().map_err(infra)?,

            reservation_status: ReservationStatus::from_snake(
                &row.get::<String, _>("reservation_status"),
            )
            .ok_or(infra("invalid reservation status"))?,

            stay_status: match row.get::<Option<String>, _>("stay_status") {
                Some(v) => Some(StayStatus::from_snake(&v).ok_or(infra("invalid stay status"))?),

                None => None,
            },

            room_class: row.get("room_class"),

            room_id: row
                .get::<Option<String>, _>("room_id")
                .map(|id| Uuid::parse_str(&id))
                .transpose()
                .map_err(infra)?,

            booking_channel: ReservationBookingChannel::from_snake(
                row.get::<String, _>("booking_channel").as_str(),
            )
            .ok_or_else(|| infra("invalid booking channel"))?,

            plan_code: row.get("plan_code"),

            version: row.get("version"),

            package_breakdowns,

            daily_stay_details,

            daily_revenue_allocations,

            participants,

            created_at: row.get::<String, _>("created_at").parse().map_err(infra)?,
        })
    }
}
