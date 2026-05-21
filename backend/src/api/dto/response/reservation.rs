use serde::{Deserialize, Serialize};

use chrono::{DateTime, NaiveDate, Utc};

use uuid::Uuid;

use crate::{
    api::dto::response::semantic_signal::OperationSemanticSignalResponse,
    domain::{
        entity::{
            guest::{Gender, Guest},
            reservation::{ReservationStatus, StayStatus},
            room::Room,
        },
        semantic::{
            operation_change_event::{ChangedField, OperationChangeEvent, OperationType},
            operation_context::{OperationActor, OperationSource},
            operational_audit_log::OperationalAuditLog,
            reservation_booking::{ReservationBookingChannel, ReservationRevenueCategory},
            reservation_edit_session::ReservationEditSession,
            reservation_guest_relation::ReservationGuestRelationType,
            reservation_linked_resources::ReservationLinkedResources,
            reservation_search_item::ReservationSearchItem,
            reservation_note::{ReservationNote, ReservationNoteKind},
            reservation_trace::{ReservationTrace, ReservationTraceKind},
            reservation_transition::{ReservationTransition, ReservationTransitionType},
        },
    },
    usecase::reservation::detail::get_reservation::ReservationDetail,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationResponse {
    pub id: Uuid,
    pub external_id: Option<String>,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub reservation_status: ReservationStatus,
    pub stay_status: Option<StayStatus>,
    pub room_class: String,
    pub room_id: Option<Uuid>,
    pub booking_channel: ReservationBookingChannel,
    pub source_channel: Option<String>,
    pub plan_code: Option<String>,
    pub version: i64,
    pub created_at: DateTime<Utc>,
    pub operation_metadata: ReservationOperationMetadataResponse,
    pub linked_resources: ReservationLinkedResources,
    pub room_assignment: ReservationRoomAssignmentResponse,
    pub package_breakdowns: Vec<ReservationPackageBreakdownResponse>,
    pub daily_details: Vec<ReservationDailyDetailResponse>,
    pub daily_revenue_allocations: Vec<ReservationDailyRevenueAllocationResponse>,
    pub participants: Vec<ReservationParticipantResponse>,
    pub participant_details: Vec<ReservationParticipantDetailResponse>,
    pub notes: Vec<ReservationNoteResponse>,
    pub traces: Vec<ReservationTraceResponse>,
    pub audit_logs: Vec<ReservationAuditLogResponse>,
    pub operation_events: Vec<ReservationOperationEventResponse>,
    pub active_edit_sessions: Vec<ReservationEditSessionResponse>,
    pub room_history: Vec<ReservationRoomHistoryResponse>,
    pub operational_visibility: ReservationOperationalVisibilityResponse,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationPackageBreakdownResponse {
    pub package_code: String,
    pub revenue_category: ReservationRevenueCategory,
    pub amount: rust_decimal::Decimal,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationDailyDetailResponse {
    pub service_date: NaiveDate,
    pub room_class: String,
    pub plan_code: Option<String>,
    pub adult_count: i64,
    pub child_count: i64,
    pub sleep_sharing_child_count: i64,
    pub sleep_sharing_children: Vec<ReservationSleepSharingChildResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationSleepSharingChildResponse {
    pub name: Option<String>,
    pub age: Option<i64>,
    pub gender: Option<Gender>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationDailyRevenueAllocationResponse {
    pub service_date: NaiveDate,
    pub package_code: String,
    pub revenue_category: ReservationRevenueCategory,
    pub department_code: Option<String>,
    pub account_code: Option<String>,
    pub amount: rust_decimal::Decimal,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationParticipantResponse {
    pub guest_id: Uuid,
    pub relation_type: ReservationGuestRelationType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationParticipantDetailResponse {
    pub guest_id: Uuid,
    pub relation_type: ReservationGuestRelationType,
    pub guest: Option<ReservationGuestSummaryResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationGuestSummaryResponse {
    pub id: Uuid,
    pub last_name: String,
    pub first_name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub nationality: Option<String>,
    pub gender: Option<Gender>,
    pub membership_code: Option<String>,
}

impl From<Guest> for ReservationGuestSummaryResponse {
    fn from(guest: Guest) -> Self {
        Self {
            id: guest.id,
            last_name: guest.profile.last_name,
            first_name: guest.profile.first_name,
            phone: guest.profile.phone,
            email: guest.profile.email,
            nationality: guest.profile.nationality,
            gender: guest.profile.gender,
            membership_code: guest.profile.membership_code,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationRoomAssignmentResponse {
    pub room_id: Option<Uuid>,
    pub room: Option<ReservationRoomSummaryResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationRoomSummaryResponse {
    pub id: Uuid,
    pub room_no: String,
    pub room_class: String,
    pub is_physical: bool,
    pub is_active: bool,
}

impl From<Room> for ReservationRoomSummaryResponse {
    fn from(room: Room) -> Self {
        Self {
            id: room.id,
            room_no: room.room_no,
            room_class: room.room_class,
            is_physical: room.is_physical,
            is_active: room.is_active,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationOperationMetadataResponse {
    pub version: i64,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationOperationalVisibilityResponse {
    pub internal_note: Option<String>,
    pub audit_trail_available: bool,
    pub timeline_available: bool,
    pub semantic_signal_available: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationNoteResponse {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub kind: ReservationNoteKind,
    pub body: String,
    pub actor_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<String>,
}

impl From<ReservationNote> for ReservationNoteResponse {
    fn from(note: ReservationNote) -> Self {
        Self {
            id: note.id,
            reservation_id: note.reservation_id,
            kind: note.kind,
            body: note.body,
            actor_id: note.actor_id,
            created_at: note.created_at,
            deleted_at: note.deleted_at,
            deleted_by: note.deleted_by,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationTraceResponse {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub kind: ReservationTraceKind,
    pub department_code: Option<String>,
    pub body: String,
    pub actor_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<String>,
}

impl From<ReservationTrace> for ReservationTraceResponse {
    fn from(trace: ReservationTrace) -> Self {
        Self {
            id: trace.id,
            reservation_id: trace.reservation_id,
            kind: trace.kind,
            department_code: trace.department_code,
            body: trace.body,
            actor_id: trace.actor_id,
            created_at: trace.created_at,
            resolved_at: trace.resolved_at,
            resolved_by: trace.resolved_by,
            deleted_at: trace.deleted_at,
            deleted_by: trace.deleted_by,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationAuditLogResponse {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub action: String,
    pub actor: OperationActor,
    pub actor_id: Option<String>,
    pub source: OperationSource,
    pub before_json: Option<String>,
    pub after_json: String,
    pub changed_fields_json: String,
    pub reason: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

impl From<OperationalAuditLog> for ReservationAuditLogResponse {
    fn from(log: OperationalAuditLog) -> Self {
        Self {
            id: log.id,
            operation_id: log.operation_id,
            action: log.action,
            actor: log.actor,
            actor_id: log.actor_id,
            source: log.source,
            before_json: log.before_json,
            after_json: log.after_json,
            changed_fields_json: log.changed_fields_json,
            reason: log.reason,
            occurred_at: log.occurred_at,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationOperationEventResponse {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub operation_type: OperationType,
    pub actor: OperationActor,
    pub actor_id: Option<String>,
    pub source: OperationSource,
    pub before_json: Option<String>,
    pub after_json: String,
    pub changed_fields: Vec<ChangedField>,
    pub occurred_at: DateTime<Utc>,
    pub semantic_signal: OperationSemanticSignalResponse,
}

impl ReservationOperationEventResponse {
    fn from_event(
        event: OperationChangeEvent,
        semantic_signal: OperationSemanticSignalResponse,
    ) -> Self {
        let changed_fields =
            serde_json::from_str(&event.changed_fields_json).unwrap_or_else(|_| vec![]);

        Self {
            id: event.id,
            operation_id: event.operation_id,
            operation_type: event.operation_type,
            actor: event.actor,
            actor_id: event.actor_id,
            source: event.source,
            before_json: event.before_json,
            after_json: event.after_json,
            changed_fields,
            occurred_at: event.occurred_at,
            semantic_signal,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationRoomHistoryResponse {
    pub id: Uuid,
    pub transition_type: ReservationTransitionType,
    pub before_room_id: Option<Uuid>,
    pub after_room_id: Option<Uuid>,
    pub occurred_at: DateTime<Utc>,
}

impl From<ReservationTransition> for ReservationRoomHistoryResponse {
    fn from(transition: ReservationTransition) -> Self {
        Self {
            id: transition.id,
            transition_type: transition.transition_type,
            before_room_id: parse_optional_uuid(&transition.before_value),
            after_room_id: parse_optional_uuid(&transition.after_value),
            occurred_at: transition.occurred_at,
        }
    }
}

fn parse_optional_uuid(value: &str) -> Option<Uuid> {
    if value.trim().is_empty() {
        return None;
    }

    Uuid::parse_str(value).ok()
}

impl From<ReservationDetail> for ReservationResponse {
    fn from(detail: ReservationDetail) -> Self {
        let reservation = detail.reservation;
        let room_id = reservation.room_id;
        let version = reservation.version;

        Self {
            id: reservation.id,

            external_id: reservation.external_id,

            check_in: reservation.check_in,

            check_out: reservation.check_out,

            reservation_status: reservation.reservation_status,

            stay_status: reservation.stay_status,

            room_class: reservation.room_class,

            room_id,

            booking_channel: reservation.booking_channel,

            source_channel: reservation.source_channel,

            plan_code: reservation.plan_code,

            version,

            created_at: reservation.created_at,

            operation_metadata: ReservationOperationMetadataResponse {
                version,
                updated_at: None,
            },
            
            linked_resources: ReservationLinkedResources{
                primary_guest_id: detail
                    .participant_details
                    .iter()
                    .find(|participant| {
                        participant.relation_type
                            == crate::domain::semantic::reservation_guest_relation::ReservationGuestRelationType::Primary
                    })
                    .map(|participant| participant.guest_id),
                assigned_room_id: room_id,
                folio_id: None,
            },

            room_assignment: ReservationRoomAssignmentResponse {
                room_id,
                room: detail
                    .room_detail
                    .map(|room_detail| ReservationRoomSummaryResponse::from(room_detail.room)),
            },

            package_breakdowns: reservation
                .package_breakdowns
                .into_iter()
                .map(|breakdown| ReservationPackageBreakdownResponse {
                    package_code: breakdown.package_code,
                    revenue_category: breakdown.revenue_category,
                    amount: breakdown.amount,
                })
                .collect(),

            daily_details: reservation
                .daily_stay_details
                .into_iter()
                .map(|detail| ReservationDailyDetailResponse {
                    service_date: detail.service_date,
                    room_class: detail.room_class,
                    plan_code: detail.plan_code,
                    adult_count: detail.adult_count,
                    child_count: detail.child_count,
                    sleep_sharing_child_count: detail.sleep_sharing_child_count,
                    sleep_sharing_children: detail
                        .sleep_sharing_children
                        .into_iter()
                        .map(|child| ReservationSleepSharingChildResponse {
                            name: child.name,
                            age: child.age,
                            gender: child.gender,
                        })
                        .collect(),
                })
                .collect(),

            daily_revenue_allocations: reservation
                .daily_revenue_allocations
                .into_iter()
                .map(|allocation| ReservationDailyRevenueAllocationResponse {
                    service_date: allocation.service_date,
                    package_code: allocation.package_code,
                    revenue_category: allocation.revenue_category,
                    department_code: allocation.department_code,
                    account_code: allocation.account_code,
                    amount: allocation.amount,
                })
                .collect(),

            participants: reservation
                .participants
                .into_iter()
                .map(|p| ReservationParticipantResponse {
                    guest_id: p.guest_id,
                    relation_type: p.relation_type,
                })
                .collect(),

            participant_details: detail
                .participant_details
                .into_iter()
                .map(|participant| ReservationParticipantDetailResponse {
                    guest_id: participant.guest_id,
                    relation_type: participant.relation_type,
                    guest: participant.guest.map(ReservationGuestSummaryResponse::from),
                })
                .collect(),

            notes: detail
                .notes
                .into_iter()
                .map(ReservationNoteResponse::from)
                .collect(),

            traces: detail
                .traces
                .into_iter()
                .map(ReservationTraceResponse::from)
                .collect(),

            audit_logs: detail
                .audit_logs
                .into_iter()
                .map(ReservationAuditLogResponse::from)
                .collect(),

            operation_events: detail
                .operation_events
                .into_iter()
                .map(|operation_event| {
                    ReservationOperationEventResponse::from_event(
                        operation_event.event,
                        OperationSemanticSignalResponse::from(operation_event.semantic_signal),
                    )
                })
                .collect(),

            active_edit_sessions: detail
                .active_edit_sessions
                .into_iter()
                .map(reservation_edit_session_to_response)
                .collect(),

            room_history: detail
                .room_history
                .into_iter()
                .map(ReservationRoomHistoryResponse::from)
                .collect(),

            operational_visibility: ReservationOperationalVisibilityResponse {
                internal_note: None,
                audit_trail_available: true,
                timeline_available: true,
                semantic_signal_available: false,
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationSearchItemResponse {
    pub id: Uuid,
    pub external_id: Option<String>,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub reservation_status: String,
    pub stay_status: Option<String>,
    pub room_class: Option<String>,
    pub room_id: Option<Uuid>,
    pub booking_channel: Option<String>,
    pub source_channel: Option<String>,
    pub primary_guest_id: Option<Uuid>,
    pub primary_guest_name: Option<String>,
    pub linked_resources: ReservationLinkedResources,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationEditSessionResponse {
    pub id: Uuid,
    pub reservation_id: Uuid,
    pub actor_id: String,
    pub actor_label: Option<String>,
    pub opened_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReservationEditSessionWarningResponse {
    pub active_sessions: Vec<ReservationEditSessionResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenReservationEditSessionResponse {
    pub session: ReservationEditSessionResponse,
    pub warning: Option<ReservationEditSessionWarningResponse>,
}

fn reservation_edit_session_to_response(
    session: ReservationEditSession,
) -> ReservationEditSessionResponse {
    ReservationEditSessionResponse {
        id: session.id,
        reservation_id: session.reservation_id,
        actor_id: session.actor_id,
        actor_label: session.actor_label,
        opened_at: session.opened_at,
        expires_at: session.expires_at,
    }
}

impl From<ReservationSearchItem>
    for ReservationSearchItemResponse
{
    fn from(item: ReservationSearchItem) -> Self {
        Self {
            id: item.id,
            external_id: item.external_id,
            check_in: item.check_in,
            check_out: item.check_out,

            reservation_status: item
                .reservation_status
                .to_snake()
                .to_string(),

            stay_status: item
                .stay_status
                .map(|status| status.to_snake().to_string()),

            room_class: item.room_class,
            room_id: item.room_id,

            booking_channel: item.booking_channel,
            source_channel: item.source_channel,

            primary_guest_id: item.linked_resources.primary_guest_id,
            primary_guest_name: item.primary_guest_name,

            linked_resources: item.linked_resources,

            created_at: item.created_at,
        }
    }
}