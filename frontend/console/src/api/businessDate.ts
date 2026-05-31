import { apiGet, apiPost } from "./client";

export type BusinessDateStatus =
  | "open"
  | "closing"
  | "closed";

export type BusinessDate = {
  id: string;
  business_date: string;
  status: BusinessDateStatus;
  opened_at: string;
  closing_started_at: string | null;
  closed_at: string | null;
  created_at: string;
  updated_at: string;
};

export type NightAuditReservationItem = {
  reservation_id: string;
  external_id: string | null;
  check_in: string;
  check_out: string;
  room_id: string | null;
};

export type NightAuditRoomChargeCandidate = {
  reservation_id: string;
  folio_id: string;
  service_date: string;
  amount: string;
};

export type NightAuditRoomChargeBlocker = {
  reservation_id: string;
  service_date: string;
  amount: string;
  reason: string;
};

export type NightAuditWorklist = {
  business_date: BusinessDate;
  unresolved_arrivals: NightAuditReservationItem[];
  unresolved_departures: NightAuditReservationItem[];
  room_charge_candidates: NightAuditRoomChargeCandidate[];
  room_charge_blockers: NightAuditRoomChargeBlocker[];
};

export type StartNightAuditResponse = {
  business_date: BusinessDate;
};

export type FinalizeNightAuditResponse = {
  closed_business_date: BusinessDate;
  current_business_date: BusinessDate;
};

export type PostNightAuditRoomChargesResponse = {
  posted_room_charges: NightAuditRoomChargeCandidate[];
};

export function getCurrentBusinessDate(): Promise<BusinessDate> {
  return apiGet<BusinessDate>("/business-date/current");
}

export function getNightAuditWorklist(): Promise<NightAuditWorklist> {
  return apiGet<NightAuditWorklist>("/business-date/night-audit/worklist");
}

export function startNightAudit(): Promise<StartNightAuditResponse> {
  return apiPost<StartNightAuditResponse, { reason: string }>(
    "/business-date/night-audit/start",
    { reason: "console night audit start" },
  );
}

export function postNightAuditRoomCharges(): Promise<PostNightAuditRoomChargesResponse> {
  return apiPost<PostNightAuditRoomChargesResponse, undefined>(
    "/business-date/night-audit/post-room-charges",
    undefined,
  );
}

export function finalizeNightAudit(): Promise<FinalizeNightAuditResponse> {
  return apiPost<FinalizeNightAuditResponse, { reason: string }>(
    "/business-date/night-audit/finalize",
    { reason: "console night audit finalize" },
  );
}

export function markNightAuditArrivalNoShow(
  reservationId: string,
): Promise<unknown> {
  return apiPost<unknown, undefined>(
    `/business-date/night-audit/arrivals/${reservationId}/no-show`,
    undefined,
  );
}

export function extendNightAuditDeparture(
  reservationId: string,
): Promise<unknown> {
  return apiPost<unknown, undefined>(
    `/business-date/night-audit/departures/${reservationId}/extend-stay`,
    undefined,
  );
}
