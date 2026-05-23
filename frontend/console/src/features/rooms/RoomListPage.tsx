import { useState } from "react";
import { useQuery } from "@tanstack/react-query";

import { ApiClientError } from "../../api/client";
import { queryKeys } from "../../api/queryKeys";
import {
  listRooms,
  type ListRoomsRequest,
} from "../../api/room";
import { RoomListTable } from "./RoomListTable";
import { RoomSearchForm } from "./RoomSearchForm";

function todayString() {
  return new Date().toISOString().slice(0, 10);
}

function errorMessage(error: unknown) {
  if (error instanceof ApiClientError) {
    return `HTTP ${error.status}: ${error.body}`;
  }

  if (error instanceof Error) {
    return error.message;
  }

  return "Unknown error";
}

export function RoomListPage() {
  const [listRequest, setListRequest] =
    useState<ListRoomsRequest>({
      service_date: todayString(),
    });

  const roomListQuery = useQuery({
    queryKey: queryKeys.roomList(listRequest),
    queryFn: () => listRooms(listRequest),
  });

  return (
    <div className="space-y-4">
      <section className="border border-slate-300 bg-white p-3">
        <h2 className="text-base font-semibold text-slate-950">
          Room Daily State
        </h2>

        <p className="mt-1 text-sm text-slate-600">
          Console surface for validating room occupancy and housekeeping
          state by service date.
        </p>
      </section>

      <RoomSearchForm
        initialValue={listRequest}
        onSearch={setListRequest}
      />

      {roomListQuery.isLoading && (
        <div className="border border-slate-300 bg-white p-4 text-sm">
          Loading rooms...
        </div>
      )}

      {roomListQuery.isError && (
        <pre className="overflow-auto border border-red-300 bg-red-50 p-3 text-xs text-red-800">
          {errorMessage(roomListQuery.error)}
        </pre>
      )}

      {roomListQuery.data && (
        <RoomListTable rooms={roomListQuery.data.rooms} />
      )}
    </div>
  );
}