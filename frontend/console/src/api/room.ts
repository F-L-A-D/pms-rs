import { apiGet, apiPost } from "./client";

export type RoomDailyOccupancyStatus =
  | "vacant"
  | "occupied"
  | "out_of_order";

export type RoomDailyHousekeepingStatus =
  | "dirty"
  | "cleaning"
  | "cleaned"
  | "inspected";

export type RoomDailyState = {
  room_id: string;
  service_date: string;
  occupancy_status: RoomDailyOccupancyStatus;
  housekeeping_status: RoomDailyHousekeepingStatus;
  updated_at: string;
};

export type RoomAssignmentVisibility = {
  assignment_status: "unassigned" | "assigned";
  reservation_id: string | null;
  external_id: string | null;
  reservation_status: string | null;
  stay_status: string | null;
  check_in: string | null;
  check_out: string | null;
  warning: string | null;
};

export type RoomListItem = {
  id: string;
  room_no: string;
  room_class: string;
  capacity: number | null;
  area_sqm: string;
  is_physical: boolean;
  is_active: boolean;
  daily_state: RoomDailyState | null;
  assignment: RoomAssignmentVisibility;
};

export type RoomDetail = {
  id: string;
  room_no: string;
  room_class: string;
  capacity: number | null;
  area_sqm: string;
  is_physical: boolean;
  is_active: boolean;
  daily_state: RoomDailyState | null;
  assignment: RoomAssignmentVisibility;
};

export type RoomListResponse = {
  rooms: RoomListItem[];
};

export type ListRoomsRequest = {
  service_date?: string;
};

export type GetRoomRequest = {
  service_date?: string;
};

export type RoomDailyStateCommandRequest = {
  service_date: string;
};

function toRoomServiceDateQuery(
  request: ListRoomsRequest | GetRoomRequest,
): string {
  const params = new URLSearchParams();

  const serviceDate = request.service_date?.trim();

  if (serviceDate) {
    params.set("service_date", serviceDate);
  }

  const query = params.toString();

  return query ? `?${query}` : "";
}

export function listRooms(
  request: ListRoomsRequest,
): Promise<RoomListResponse> {
  return apiGet<RoomListResponse>(
    `/rooms${toRoomServiceDateQuery(request)}`,
  );
}

export function getRoom(
  roomId: string,
  request: GetRoomRequest,
): Promise<RoomDetail> {
  return apiGet<RoomDetail>(
    `/rooms/${roomId}${toRoomServiceDateQuery(request)}`,
  );
}

export function markRoomDirty(
  roomId: string,
  request: RoomDailyStateCommandRequest,
): Promise<RoomDailyState> {
  return apiPost<RoomDailyState, RoomDailyStateCommandRequest>(
    `/housekeeping/${roomId}/dirty`,
    request,
  );
}

export function startRoomCleaning(
  roomId: string,
  request: RoomDailyStateCommandRequest,
): Promise<RoomDailyState> {
  return apiPost<RoomDailyState, RoomDailyStateCommandRequest>(
    `/housekeeping/${roomId}/start-cleaning`,
    request,
  );
}

export function finishRoomCleaning(
  roomId: string,
  request: RoomDailyStateCommandRequest,
): Promise<RoomDailyState> {
  return apiPost<RoomDailyState, RoomDailyStateCommandRequest>(
    `/housekeeping/${roomId}/finish-cleaning`,
    request,
  );
}

export function inspectRoom(
  roomId: string,
  request: RoomDailyStateCommandRequest,
): Promise<RoomDailyState> {
  return apiPost<RoomDailyState, RoomDailyStateCommandRequest>(
    `/housekeeping/${roomId}/inspect`,
    request,
  );
}

export function markRoomOutOfOrder(
  roomId: string,
  request: RoomDailyStateCommandRequest,
): Promise<RoomDailyState> {
  return apiPost<RoomDailyState, RoomDailyStateCommandRequest>(
    `/rooms/${roomId}/out-of-order`,
    request,
  );
}

export function returnRoomToService(
  roomId: string,
  request: RoomDailyStateCommandRequest,
): Promise<RoomDailyState> {
  return apiPost<RoomDailyState, RoomDailyStateCommandRequest>(
    `/rooms/${roomId}/return-to-service`,
    request,
  );
}