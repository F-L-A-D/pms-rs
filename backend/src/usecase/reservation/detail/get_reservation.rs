use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::{
        entity::{folio::FolioStatus, guest::Guest, reservation::Reservation, room::Room},
        semantic::{
            operation_change_event::OperationChangeEvent,
            operational_audit_log::OperationalAuditLog,
            reservation_edit_session::ReservationEditSession,
            reservation_guest_relation::ReservationGuestRelationType,
            reservation_note::ReservationNote, reservation_trace::ReservationTrace,
            reservation_transition::ReservationTransition,
        },
    },
    error::app_error::AppResult,
    projection::signal::{
        access::{
            fetch_change_pattern::fetch_change_pattern,
            fetch_confidence_profile::fetch_confidence_profile,
            fetch_semantic_activation::fetch_semantic_activation,
        },
        model::operation_semantic_signal::OperationSemanticSignal,
    },
    repository::sqlite::behavioral::reservation_transition_repository::SqliteReservationTransitionRepository,
    repository::sqlite::operational::{
        billing::folio_repository::SqliteFolioRepository,
        guest::guest_repository::SqliteGuestRepository,
        operation::{
            operation_change_event_repository::SqliteOperationChangeEventRepository,
            operational_audit_log_repository::SqliteOperationalAuditLogRepository,
        },
        reservation::{
            reservation_edit_session_repository::SqliteReservationEditSessionRepository,
            reservation_note_repository::SqliteReservationNoteRepository,
            reservation_repository::SqliteReservationRepository,
            reservation_trace_repository::SqliteReservationTraceRepository,
        },
        room::room_repository::SqliteRoomRepository,
    },
};

#[derive(Debug)]
pub struct ReservationDetail {
    pub reservation: Reservation,
    pub participant_details: Vec<ReservationDetailParticipant>,
    pub room_detail: Option<ReservationDetailRoom>,
    pub notes: Vec<ReservationNote>,
    pub traces: Vec<ReservationTrace>,
    pub audit_logs: Vec<OperationalAuditLog>,
    pub operation_events: Vec<ReservationDetailOperationEvent>,
    pub active_edit_sessions: Vec<ReservationEditSession>,
    pub room_history: Vec<ReservationTransition>,
    pub folio_id: Option<Uuid>,
    pub folio_links: Vec<ReservationDetailFolioLink>,
}

#[derive(Debug)]
pub struct ReservationDetailParticipant {
    pub guest_id: Uuid,
    pub relation_type: ReservationGuestRelationType,
    pub guest: Option<Guest>,
}

#[derive(Debug)]
pub struct ReservationDetailRoom {
    pub room: Room,
}

#[derive(Debug)]
pub struct ReservationDetailOperationEvent {
    pub event: OperationChangeEvent,
    pub semantic_signal: OperationSemanticSignal,
}

#[derive(Debug)]
pub struct ReservationDetailFolioLink {
    pub folio_id: Uuid,
    pub status: FolioStatus,
}

pub async fn get_reservation(db: &Db, reservation_id: Uuid) -> AppResult<Option<Reservation>> {
    let mut tx = db.begin_tx().await;

    let result = SqliteReservationRepository::find_by_id(&mut tx, reservation_id).await;

    let _ = tx.rollback().await;

    result
}

pub async fn get_reservation_detail(
    db: &Db,
    reservation_id: Uuid,
) -> AppResult<Option<ReservationDetail>> {
    let mut tx = db.begin_tx().await;

    let reservation = match SqliteReservationRepository::find_by_id(&mut tx, reservation_id).await?
    {
        Some(reservation) => reservation,

        None => {
            let _ = tx.rollback().await;

            return Ok(None);
        }
    };

    let mut participant_details = vec![];

    for participant in reservation.participants.iter() {
        let guest = SqliteGuestRepository::find_by_id(&mut tx, participant.guest_id).await?;

        participant_details.push(ReservationDetailParticipant {
            guest_id: participant.guest_id,
            relation_type: participant.relation_type.clone(),
            guest,
        });
    }

    let room_detail = match reservation.room_id {
        Some(room_id) => SqliteRoomRepository::find_by_id(&mut tx, room_id)
            .await?
            .map(|room| ReservationDetailRoom { room }),

        None => None,
    };

    let notes =
        SqliteReservationNoteRepository::list_by_reservation_id(&mut tx, reservation_id).await?;

    let traces =
        SqliteReservationTraceRepository::list_by_reservation_id(&mut tx, reservation_id).await?;

    let active_edit_sessions =
        SqliteReservationEditSessionRepository::list_active_by_reservation_id(
            &mut tx,
            reservation_id,
            chrono::Utc::now(),
        )
        .await?;

    let room_history =
        SqliteReservationTransitionRepository::find_by_reservation_id(&mut tx, reservation_id)
            .await?
            .into_iter()
            .filter(|transition| transition.field_name == "room_id")
            .collect();

    let folios = SqliteFolioRepository::list_by_reservation_id(&mut tx, reservation_id).await?;

    let folio_id = folios
        .iter()
        .find(|folio| matches!(folio.status, FolioStatus::Open | FolioStatus::Locked,))
        .map(|folio| folio.id);

    let folio_links = folios
        .iter()
        .map(|folio| ReservationDetailFolioLink {
            folio_id: folio.id,
            status: folio.status,
        })
        .collect::<Vec<_>>();

    let audit_logs = SqliteOperationalAuditLogRepository::list_by_aggregate(
        &mut tx,
        "reservation",
        reservation_id,
    )
    .await?;

    let events = SqliteOperationChangeEventRepository::list_by_aggregate(
        &mut tx,
        "reservation",
        reservation_id,
    )
    .await?;

    let mut operation_events = vec![];

    for event in events {
        operation_events.push(ReservationDetailOperationEvent {
            semantic_signal: OperationSemanticSignal {
                event_id: event.id,
                change_pattern: fetch_change_pattern(&mut tx, event.id).await?,
                confidence_profile: fetch_confidence_profile(&mut tx, event.id).await?,
                semantic_activation: fetch_semantic_activation(&mut tx, event.id).await?,
            },
            event,
        });
    }

    let _ = tx.rollback().await;

    Ok(Some(ReservationDetail {
        reservation,
        participant_details,
        room_detail,
        notes,
        traces,
        audit_logs,
        operation_events,
        active_edit_sessions,
        room_history,
        folio_id,
        folio_links,
    }))
}
