use chrono::{Duration, Utc};

use uuid::Uuid;

use crate::{
    api::dto::input::reservation::OpenReservationEditSessionInput,
    db::connection::Db,
    domain::semantic::reservation_edit_session::ReservationEditSession,
    error::app_error::{infra, not_found, validation, AppResult},
    repository::sqlite::operational::reservation::{
        reservation_edit_session_repository::SqliteReservationEditSessionRepository,
        reservation_repository::SqliteReservationRepository,
    },
};

pub struct OpenReservationEditSessionResult {
    pub session: ReservationEditSession,
    pub active_other_sessions: Vec<ReservationEditSession>,
}

pub async fn execute(
    db: &Db,
    input: OpenReservationEditSessionInput,
) -> AppResult<OpenReservationEditSessionResult> {
    let mut tx = db.begin_tx().await;

    let result = async {
        if input.actor_id.trim().is_empty() {
            return Err(validation("actor_id is required"));
        }

        SqliteReservationRepository::find_by_id(&mut tx, input.reservation_id)
            .await?
            .ok_or_else(|| not_found("reservation not found"))?;

        let now = Utc::now();
        let active_sessions =
            SqliteReservationEditSessionRepository::list_active_by_reservation_id(
                &mut tx,
                input.reservation_id,
                now,
            )
            .await?;

        let active_other_sessions = active_sessions
            .into_iter()
            .filter(|session| session.actor_id != input.actor_id)
            .collect::<Vec<_>>();

        let lease_minutes = input.lease_minutes.unwrap_or(15);

        if !(1..=120).contains(&lease_minutes) {
            return Err(validation("lease_minutes must be between 1 and 120"));
        }

        let session = ReservationEditSession {
            id: Uuid::new_v4(),
            reservation_id: input.reservation_id,
            actor_id: input.actor_id,
            actor_label: input.actor_label,
            opened_at: now,
            expires_at: now + Duration::minutes(lease_minutes),
            closed_at: None,
        };

        SqliteReservationEditSessionRepository::save(&mut tx, &session).await?;

        Ok(OpenReservationEditSessionResult {
            session,
            active_other_sessions,
        })
    }
    .await;

    match result {
        Ok(result) => {
            tx.commit().await.map_err(infra)?;

            Ok(result)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}
