import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Link, useParams } from "react-router-dom";

import { ApiClientError } from "../../api/client";
import { queryKeys } from "../../api/queryKeys";
import {
  createReservationNote,
  createReservationTrace,
  getReservationDetail,
  type CreateReservationNoteRequest,
  type CreateReservationTraceRequest,
} from "../../api/reservation";
import { ReservationDetailView } from "./ReservationDetailView";

function errorMessage(error: unknown) {
  if (error instanceof ApiClientError) {
    return `HTTP ${error.status}: ${error.body}`;
  }

  if (error instanceof Error) {
    return error.message;
  }

  return "Unknown error";
}

export function ReservationDetailPage() {
  const { reservationId = "" } = useParams();

  const queryClient = useQueryClient();

  const reservationQuery = useQuery({
    queryKey: queryKeys.reservationDetail(reservationId),
    queryFn: () => getReservationDetail(reservationId),
    enabled: reservationId.length > 0,
  });

  const createNoteMutation = useMutation({
    mutationFn: (request: CreateReservationNoteRequest) =>
      createReservationNote(reservationId, request),
    onSuccess: () =>
      queryClient.invalidateQueries({
        queryKey: queryKeys.reservationDetail(reservationId),
      }),
  });

  const createTraceMutation = useMutation({
    mutationFn: (request: CreateReservationTraceRequest) =>
      createReservationTrace(reservationId, request),
    onSuccess: () =>
      queryClient.invalidateQueries({
        queryKey: queryKeys.reservationDetail(reservationId),
      }),
  });

  return (
    <div className="space-y-4">
      <div className="border border-slate-300 bg-white p-3">
        <Link
          className="text-sm font-semibold text-slate-700 underline"
          to="/reservations"
        >
          Back to reservation search
        </Link>
      </div>

      {!reservationId && (
        <div className="border border-slate-300 bg-white p-4 text-sm text-slate-600">
          Reservation ID is missing.
        </div>
      )}

      {reservationQuery.isLoading && (
        <div className="border border-slate-300 bg-white p-4 text-sm">
          Loading reservation...
        </div>
      )}

      {reservationQuery.isError && (
        <pre className="overflow-auto border border-red-300 bg-red-50 p-3 text-xs text-red-800">
          {errorMessage(reservationQuery.error)}
        </pre>
      )}

      {reservationQuery.data && (
        <ReservationDetailView
          reservation={reservationQuery.data}
          noteError={createNoteMutation.error ? errorMessage(createNoteMutation.error) : null}
          noteSaving={createNoteMutation.isPending}
          onCreateNote={(request) => createNoteMutation.mutate(request)}
          traceError={createTraceMutation.error ? errorMessage(createTraceMutation.error) : null}
          traceSaving={createTraceMutation.isPending}
          onCreateTrace={(request) => createTraceMutation.mutate(request)}
        />
      )}
    </div>
  );
}