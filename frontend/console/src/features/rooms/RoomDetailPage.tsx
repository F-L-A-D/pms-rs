import { useMemo } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Link, useParams, useSearchParams } from "react-router-dom";

import { ApiClientError } from "../../api/client";
import { queryKeys } from "../../api/queryKeys";
import {
  finishRoomCleaning,
  getRoom,
  inspectRoom,
  markRoomDirty,
  markRoomOutOfOrder,
  returnRoomToService,
  startRoomCleaning,
  type RoomDailyStateCommandRequest,
} from "../../api/room";
import { RoomDetailView } from "./RoomDetailView";

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

export function RoomDetailPage() {
  const { roomId = "" } = useParams();
  const [searchParams] = useSearchParams();

  const queryClient = useQueryClient();

  const serviceDate =
    searchParams.get("service_date") ?? todayString();

  const roomDetailRequest = useMemo(
    () => ({
      service_date: serviceDate,
    }),
    [serviceDate],
  );

  const roomQuery = useQuery({
    queryKey: queryKeys.roomDetail(roomId, roomDetailRequest),
    queryFn: () => getRoom(roomId, roomDetailRequest),
    enabled: roomId.length > 0,
  });

  function invalidateRoomQueries() {
    queryClient.invalidateQueries({
      queryKey: queryKeys.roomDetail(roomId, roomDetailRequest),
    });

    queryClient.invalidateQueries({
      queryKey: queryKeys.roomList({ service_date: serviceDate }),
    });
  }

  function commandRequest(): RoomDailyStateCommandRequest {
    return {
      service_date: serviceDate,
    };
  }

  const markDirtyMutation = useMutation({
    mutationFn: () => markRoomDirty(roomId, commandRequest()),
    onSuccess: invalidateRoomQueries,
  });

  const startCleaningMutation = useMutation({
    mutationFn: () => startRoomCleaning(roomId, commandRequest()),
    onSuccess: invalidateRoomQueries,
  });

  const finishCleaningMutation = useMutation({
    mutationFn: () => finishRoomCleaning(roomId, commandRequest()),
    onSuccess: invalidateRoomQueries,
  });

  const inspectMutation = useMutation({
    mutationFn: () => inspectRoom(roomId, commandRequest()),
    onSuccess: invalidateRoomQueries,
  });

  const markOutOfOrderMutation = useMutation({
    mutationFn: () => markRoomOutOfOrder(roomId, commandRequest()),
    onSuccess: invalidateRoomQueries,
  });

  const returnToServiceMutation = useMutation({
    mutationFn: () => returnRoomToService(roomId, commandRequest()),
    onSuccess: invalidateRoomQueries,
  });

  const actionError =
    markDirtyMutation.error ??
    startCleaningMutation.error ??
    finishCleaningMutation.error ??
    inspectMutation.error ??
    markOutOfOrderMutation.error ??
    returnToServiceMutation.error ??
    null;

  const actionSaving =
    markDirtyMutation.isPending ||
    startCleaningMutation.isPending ||
    finishCleaningMutation.isPending ||
    inspectMutation.isPending ||
    markOutOfOrderMutation.isPending ||
    returnToServiceMutation.isPending;

  return (
    <div className="space-y-4">
      <div className="border border-slate-300 bg-white p-3">
        <Link
          className="text-sm font-semibold text-slate-700 underline"
          to={`/rooms?service_date=${serviceDate}`}
        >
          Back to room list
        </Link>
      </div>

      {!roomId && (
        <div className="border border-slate-300 bg-white p-4 text-sm text-slate-600">
          Room ID is missing.
        </div>
      )}

      {roomQuery.isLoading && (
        <div className="border border-slate-300 bg-white p-4 text-sm">
          Loading room...
        </div>
      )}

      {roomQuery.isError && (
        <pre className="overflow-auto border border-red-300 bg-red-50 p-3 text-xs text-red-800">
          {errorMessage(roomQuery.error)}
        </pre>
      )}

      {roomQuery.data && (
        <RoomDetailView
          room={roomQuery.data}
          serviceDate={serviceDate}
          actionError={actionError ? errorMessage(actionError) : null}
          actionSaving={actionSaving}
          onMarkDirty={() => markDirtyMutation.mutate()}
          onStartCleaning={() => startCleaningMutation.mutate()}
          onFinishCleaning={() => finishCleaningMutation.mutate()}
          onInspect={() => inspectMutation.mutate()}
          onMarkOutOfOrder={() => markOutOfOrderMutation.mutate()}
          onReturnToService={() => returnToServiceMutation.mutate()}
        />
      )}
    </div>
  );
}