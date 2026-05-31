use sqlx::{QueryBuilder, Row, Sqlite, Transaction};

use uuid::Uuid;

use crate::{
    api::dto::input::reservation::SearchReservationsInput,
    domain::{
        entity::reservation::{Reservation, ReservationStatus, StayStatus},
        semantic::{
            reservation_booking::ReservationBookingChannel,
            reservation_linked_resources::ReservationLinkedResources,
            reservation_search_item::ReservationSearchItem,
        },
    },
    error::app_error::{conflict, infra, AppResult},
    repository::sqlite::operational::reservation::{
        reservation_daily_revenue_allocation_repository::SqliteReservationDailyRevenueAllocationRepository,
        reservation_daily_stay_detail_repository::SqliteReservationDailyStayDetailRepository,
        reservation_guest_relation_repository::SqliteReservationGuestRelationRepository,
        reservation_package_breakdown_repository::SqliteReservationPackageBreakdownRepository,
    },
};

pub struct ReservationSearchRow {
    pub id: Uuid,
    pub external_id: Option<String>,
    pub check_in: chrono::NaiveDate,
    pub check_out: chrono::NaiveDate,
    pub reservation_status: ReservationStatus,
    pub stay_status: Option<StayStatus>,
    pub room_class: Option<String>,
    pub room_id: Option<Uuid>,
    pub primary_guest_id: Option<Uuid>,
    pub primary_guest_name: Option<String>,
    pub folio_id: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

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
                source_channel,
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
                ?12,
                ?13
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
        .bind(&reservation.source_channel)
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
                source_channel = ?9,
                plan_code = ?10,
                version = ?11
            WHERE id = ?12
              AND version = ?13
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
        .bind(&reservation.source_channel)
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
                    source_channel,
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
                    r.source_channel,
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
                    source_channel,
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

    pub async fn list_unresolved_arrivals_by_date(
        tx: &mut Transaction<'_, Sqlite>,
        business_date: chrono::NaiveDate,
    ) -> AppResult<Vec<Reservation>> {
        Self::list_by_date_and_status(
            tx,
            "check_in",
            business_date,
            ReservationStatus::Confirmed,
            Some(StayStatus::Confirmed),
        )
        .await
    }

    pub async fn list_unresolved_departures_by_date(
        tx: &mut Transaction<'_, Sqlite>,
        business_date: chrono::NaiveDate,
    ) -> AppResult<Vec<Reservation>> {
        Self::list_by_date_and_status(
            tx,
            "check_out",
            business_date,
            ReservationStatus::Confirmed,
            Some(StayStatus::CheckedIn),
        )
        .await
    }

    pub async fn list_checked_in_by_stay_date(
        tx: &mut Transaction<'_, Sqlite>,
        business_date: chrono::NaiveDate,
    ) -> AppResult<Vec<Reservation>> {
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
                    source_channel,
                    plan_code,
                    version,
                    created_at
                FROM reservations
                WHERE reservation_status = 'confirmed'
                  AND stay_status = 'checked_in'
                  AND check_in <= ?1
                  AND check_out > ?1
                ORDER BY check_in, id
                "#,
        )
        .bind(business_date.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        let mut reservations = vec![];

        for row in rows.iter() {
            reservations.push(Self::row_to_reservation(tx, row).await?);
        }

        Ok(reservations)
    }

    async fn list_by_date_and_status(
        tx: &mut Transaction<'_, Sqlite>,
        date_column: &str,
        business_date: chrono::NaiveDate,
        reservation_status: ReservationStatus,
        stay_status: Option<StayStatus>,
    ) -> AppResult<Vec<Reservation>> {
        let sql = format!(
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
                    source_channel,
                    plan_code,
                    version,
                    created_at
                FROM reservations
                WHERE reservation_status = ?1
                  AND stay_status = ?2
                  AND {date_column} = ?3
                ORDER BY check_in, id
                "#
        );

        let rows = sqlx::query(&sql)
            .bind(reservation_status.to_snake())
            .bind(stay_status.as_ref().map(StayStatus::to_snake))
            .bind(business_date.to_string())
            .fetch_all(&mut **tx)
            .await
            .map_err(infra)?;

        let mut reservations = vec![];

        for row in rows.iter() {
            reservations.push(Self::row_to_reservation(tx, row).await?);
        }

        Ok(reservations)
    }

    pub async fn find_by_search_input(
        tx: &mut Transaction<'_, Sqlite>,
        input: &SearchReservationsInput,
    ) -> AppResult<Vec<ReservationSearchItem>> {
        let mut builder = QueryBuilder::<Sqlite>::new(
            r#"
            SELECT
                r.id,
                r.external_id,
                r.check_in,
                r.check_out,
                r.reservation_status,
                r.stay_status,
                r.room_class,
                r.room_id,
                r.created_at,
                r.booking_channel,
                r.source_channel,

                rgr.guest_id AS primary_guest_id,
                CASE
                    WHEN g.id IS NULL THEN NULL
                    ELSE TRIM(g.last_name || ' ' || g.first_name)
                END AS primary_guest_name,

                f.folio_id AS folio_id
            FROM reservations r
            LEFT JOIN reservation_guest_relations rgr
                ON rgr.reservation_id = r.id
            AND rgr.relation_type = 'primary'
            LEFT JOIN guests g
                ON g.id = rgr.guest_id
            LEFT JOIN (
                SELECT
                    reservation_id,
                    MIN(id) AS folio_id
                FROM folios
                GROUP BY reservation_id
            ) f
                ON f.reservation_id = r.id
            WHERE 1 = 1
            "#,
        );

        if let Some(external_id) = &input.external_id {
            builder.push(" AND r.external_id = ");
            builder.push_bind(external_id);
        }

        if let Some(check_in_from) = input.check_in_from {
            builder.push(" AND r.check_in >= ");
            builder.push_bind(check_in_from.to_string());
        }

        if let Some(check_in_to) = input.check_in_to {
            builder.push(" AND r.check_in <= ");
            builder.push_bind(check_in_to.to_string());
        }

        if let Some(stay_date) = input.stay_date {
            builder.push(" AND r.check_in <= ");
            builder.push_bind(stay_date.to_string());

            builder.push(" AND r.check_out > ");
            builder.push_bind(stay_date.to_string());
        }

        if let Some(guest_name) = &input.guest_name {
            let keyword = format!("%{}%", guest_name.trim().to_lowercase());

            builder.push(
                r#"
                AND (
                    LOWER(g.last_name || ' ' || g.first_name) LIKE
                "#,
            );
            builder.push_bind(keyword.clone());

            builder.push(
                r#"
                    OR LOWER(g.first_name || ' ' || g.last_name) LIKE
                "#,
            );
            builder.push_bind(keyword);

            builder.push(" ) ");
        }

        if let Some(reservation_status) = &input.reservation_status {
            builder.push(" AND r.reservation_status = ");
            builder.push_bind(reservation_status.to_snake());
        }

        if let Some(stay_status) = &input.stay_status {
            builder.push(" AND r.stay_status = ");
            builder.push_bind(stay_status.to_snake());
        }

        if let Some(room_class) = &input.room_class {
            builder.push(" AND r.room_class = ");
            builder.push_bind(room_class);
        }

        if let Some(room_id) = input.room_id {
            builder.push(" AND r.room_id = ");
            builder.push_bind(room_id.to_string());
        }

        if let Some(booking_channel) = &input.booking_channel {
            builder.push(" AND r.booking_channel = ");
            builder.push_bind(booking_channel.to_string());
        }

        if let Some(source_channel) = &input.source_channel {
            builder.push(" AND r.source_channel = ");
            builder.push_bind(source_channel.to_string());
        }

        builder.push(" ORDER BY r.check_in ASC, r.created_at DESC ");

        let rows = builder.build().fetch_all(&mut **tx).await.map_err(infra)?;

        rows.iter()
            .map(Self::row_to_reservation_search_item)
            .collect()
    }

    pub async fn list_room_assignments_by_service_date(
        tx: &mut Transaction<'_, Sqlite>,
        service_date: chrono::NaiveDate,
    ) -> AppResult<Vec<Reservation>> {
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
                    source_channel,
                    plan_code,
                    version,
                    created_at
                FROM reservations
                WHERE room_id IS NOT NULL
                    AND check_in <= ?1
                    AND check_out > ?1
                    AND reservation_status != 'cancelled'
                    AND (
                        stay_status IS NULL
                        OR stay_status != 'checked_out'
                    )
                ORDER BY check_in ASC, created_at ASC
                "#,
        )
        .bind(service_date.to_string())
        .fetch_all(&mut **tx)
        .await
        .map_err(infra)?;

        let mut reservations = vec![];

        for row in rows.iter() {
            reservations.push(Self::row_to_reservation(tx, row).await?);
        }

        Ok(reservations)
    }

    pub async fn list_room_assignments_by_room_and_service_date(
        tx: &mut Transaction<'_, Sqlite>,
        room_id: Uuid,
        service_date: chrono::NaiveDate,
    ) -> AppResult<Vec<Reservation>> {
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
                    source_channel,
                    plan_code,
                    version,
                    created_at
                FROM reservations
                WHERE room_id = ?1
                    AND check_in <= ?2
                    AND check_out > ?2
                    AND reservation_status != 'cancelled'
                    AND (
                        stay_status IS NULL
                        OR stay_status != 'checked_out'
                    )
                ORDER BY check_in ASC, created_at ASC
                "#,
        )
        .bind(room_id.to_string())
        .bind(service_date.to_string())
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

            source_channel: row.get("source_channel"),

            plan_code: row.get("plan_code"),

            version: row.get("version"),

            package_breakdowns,

            daily_stay_details,

            daily_revenue_allocations,

            participants,

            created_at: row.get::<String, _>("created_at").parse().map_err(infra)?,
        })
    }

    fn row_to_reservation_search_item(
        row: &sqlx::sqlite::SqliteRow,
    ) -> AppResult<ReservationSearchItem> {
        let id = Uuid::parse_str(&row.get::<String, _>("id")).map_err(infra)?;

        let room_id = row
            .get::<Option<String>, _>("room_id")
            .map(|id| Uuid::parse_str(&id))
            .transpose()
            .map_err(infra)?;

        let primary_guest_id = row
            .get::<Option<String>, _>("primary_guest_id")
            .map(|id| Uuid::parse_str(&id))
            .transpose()
            .map_err(infra)?;

        let folio_id = row
            .get::<Option<String>, _>("folio_id")
            .map(|id| Uuid::parse_str(&id))
            .transpose()
            .map_err(infra)?;

        let reservation_status =
            ReservationStatus::from_snake(&row.get::<String, _>("reservation_status"))
                .ok_or_else(|| infra("invalid reservation status"))?;

        let stay_status = match row.get::<Option<String>, _>("stay_status") {
            Some(value) => {
                Some(StayStatus::from_snake(&value).ok_or_else(|| infra("invalid stay status"))?)
            }

            None => None,
        };

        Ok(ReservationSearchItem {
            id,
            external_id: row.get("external_id"),

            check_in: row.get::<String, _>("check_in").parse().map_err(infra)?,

            check_out: row.get::<String, _>("check_out").parse().map_err(infra)?,

            reservation_status,
            stay_status,

            room_class: Some(row.get::<String, _>("room_class")),

            room_id,

            booking_channel: row.get("booking_channel"),

            source_channel: row.get("source_channel"),

            primary_guest_name: row.get("primary_guest_name"),

            linked_resources: ReservationLinkedResources {
                primary_guest_id,
                assigned_room_id: room_id,
                folio_id,
            },

            created_at: row.get::<String, _>("created_at").parse().map_err(infra)?,
        })
    }
}
