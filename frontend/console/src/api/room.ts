import { apiGet } from "./client";

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

export type RoomListItem = {
  id: string;
  room_no: string;
  room_class: string;
  capacity: number | null;
  area_sqm: string;
  is_physical: boolean;
  is_active: boolean;
  daily_state: RoomDailyState | null;
};

export type RoomListResponse = {
  rooms: RoomListItem[];
};

export type ListRoomsRequest = {
  service_date?: string;
};

function toRoomListQuery(request: ListRoomsRequest): string {
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
    `/rooms${toRoomListQuery(request)}`,
  );
}