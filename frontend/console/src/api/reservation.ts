import { apiGet, apiPost } from "./client";

export type ReservationStatus =
  | "pending"
  | "confirmed"
  | "cancelled"
  | "no_show"
  | "completed";

export type StayStatus =
  | "confirmed"
  | "checked_in"
  | "checked_out"
  | "no_show";

export type ReservationParticipant = {
  guest_id: string;
  relation_type: string;
};

export type ReservationGuestSummary = {
  id: string;
  last_name: string;
  first_name: string;
  phone: string | null;
  email: string | null;
  nationality: string | null;
  gender: string | null;
  membership_code: string | null;
};

export type ReservationParticipantDetail = {
  guest_id: string;
  relation_type: string;
  guest: ReservationGuestSummary | null;
};

export type ReservationRoomSummary = {
  id: string;
  room_no: string;
  room_class: string;
  is_physical: boolean;
  is_active: boolean;
};

export type ReservationRoomAssignment = {
  room_id: string | null;
  room: ReservationRoomSummary | null;
};

export type ReservationPackageBreakdown = {
  package_code: string;
  revenue_category: string;
  amount: string;
};

export type ReservationDailyDetail = {
  service_date: string;
  room_class: string;
  plan_code: string | null;
  adult_count: number;
  child_count: number;
  sleep_sharing_child_count: number;
  sleep_sharing_children: ReservationSleepSharingChild[];
};

export type ReservationSleepSharingChild = {
  name: string | null;
  age: number | null;
  gender: string | null;
};

export type ReservationDailyRevenueAllocation = {
  service_date: string;
  package_code: string;
  revenue_category: string;
  department_code: string | null;
  account_code: string | null;
  amount: string;
};

export type ReservationOperationMetadata = {
  version: number;
  updated_at: string | null;
};

export type ReservationOperationalVisibility = {
  internal_note: string | null;
  audit_trail_available: boolean;
  timeline_available: boolean;
  semantic_signal_available: boolean;
};

export type ReservationNoteKind =
  | "global_memo"
  | "department_trace";

export type ReservationNote = {
  id: string;
  reservation_id: string;
  kind: ReservationNoteKind;
  department_code: string | null;
  body: string;
  actor_id: string | null;
  created_at: string;
};

export type ReservationAuditLog = {
  id: string;
  operation_id: string;
  action: string;
  actor: string;
  actor_id: string | null;
  source: string;
  before_json: string | null;
  after_json: string;
  changed_fields_json: string;
  reason: string | null;
  occurred_at: string;
};

export type ReservationChangedField = {
  field_name: string;
  before_value: string | null;
  after_value: string | null;
};

export type ReservationOperationEvent = {
  id: string;
  operation_id: string;
  operation_type: string;
  actor: string;
  actor_id: string | null;
  source: string;
  before_json: string | null;
  after_json: string;
  changed_fields: ReservationChangedField[];
  occurred_at: string;
  semantic_signal: OperationSemanticSignal;
};

export type ReservationEditSession = {
  id: string;
  reservation_id: string;
  actor_id: string;
  actor_label: string | null;
  opened_at: string;
  expires_at: string;
};

export type ReservationRoomHistory = {
  id: string;
  transition_type: string;
  before_room_id: string | null;
  after_room_id: string | null;
  occurred_at: string;
};

export type OperationSemanticSignal = {
  event_id: string;
  change_pattern: ChangePattern | null;
  confidence_profile: ConfidenceProfile | null;
  semantic_activation: SemanticActivation | null;
};

export type ChangePattern = {
  pattern_type: string;
  operation_type: string;
  changed_fields: unknown;
  projection_version: number;
  updated_at: string;
};

export type ConfidenceProfile = {
  confidence_score: string;
  reasons: unknown;
  projection_version: number;
  updated_at: string;
};

export type SemanticActivation = {
  activation_key: string;
  activation_score: string;
  confidence_score: string;
  is_active: boolean;
  projection_version: number;
  updated_at: string;
};

export type ReservationDetail = {
  id: string;
  external_id: string | null;
  check_in: string;
  check_out: string;
  reservation_status: ReservationStatus;
  stay_status: StayStatus | null;
  room_class: string;
  room_id: string | null;
  booking_channel: string;
  plan_code: string | null;
  version: number;
  created_at: string;
  operation_metadata: ReservationOperationMetadata;
  room_assignment: ReservationRoomAssignment;
  package_breakdowns: ReservationPackageBreakdown[];
  daily_details: ReservationDailyDetail[];
  daily_revenue_allocations: ReservationDailyRevenueAllocation[];
  participants: ReservationParticipant[];
  participant_details: ReservationParticipantDetail[];
  notes: ReservationNote[];
  audit_logs: ReservationAuditLog[];
  operation_events: ReservationOperationEvent[];
  active_edit_sessions: ReservationEditSession[];
  room_history: ReservationRoomHistory[];
  operational_visibility: ReservationOperationalVisibility;
};

export type CreateReservationNoteRequest = {
  kind: ReservationNoteKind;
  department_code: string | null;
  body: string;
  actor_id: string | null;
};

export function getReservationDetail(
  reservationId: string,
): Promise<ReservationDetail> {
  return apiGet<ReservationDetail>(
    `/reservations/${reservationId}`,
  );
}

export function createReservationNote(
  reservationId: string,
  request: CreateReservationNoteRequest,
): Promise<ReservationNote> {
  return apiPost<ReservationNote, CreateReservationNoteRequest>(
    `/reservations/${reservationId}/notes`,
    request,
  );
}
