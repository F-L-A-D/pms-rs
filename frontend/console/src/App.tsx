import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import type { FormEvent } from "react";

import { ApiClientError } from "./api/client";
import { getHealth } from "./api/health";
import { queryKeys } from "./api/queryKeys";
import {
  createReservationNote,
  createReservationTrace,
  getReservationDetail,
  type CreateReservationNoteRequest,
  type CreateReservationTraceRequest,
} from "./api/reservation";
import { ReservationDetailView } from "./features/reservations/ReservationDetailView";

export default function App() {
  const params = new URLSearchParams(window.location.search);

  const initialReservationId = params.get("reservationId") ?? params.get("id") ?? "";

  const [reservationId, setReservationId] = useState(initialReservationId);

  const [draftReservationId, setDraftReservationId] =
    useState(initialReservationId);

  const healthQuery = useQuery({
    queryKey: queryKeys.health,
    queryFn: getHealth,
  });

  const reservationQuery = useQuery({
    queryKey: queryKeys.reservationDetail(reservationId),
    queryFn: () => getReservationDetail(reservationId),
    enabled: reservationId.length > 0,
  });

  const queryClient = useQueryClient();

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

  function loadReservation(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const nextReservationId = draftReservationId.trim();

    setReservationId(nextReservationId);

    const url = new URL(window.location.href);

    if (nextReservationId) {
      url.searchParams.set("reservationId", nextReservationId);
    } else {
      url.searchParams.delete("reservationId");
    }

    window.history.replaceState({}, "", url);
  }

  const reservationError =
    reservationQuery.error instanceof ApiClientError
      ? `HTTP ${reservationQuery.error.status}: ${reservationQuery.error.body}`
      : reservationQuery.error instanceof Error
        ? reservationQuery.error.message
        : "Unknown error";

  const noteError =
    createNoteMutation.error instanceof ApiClientError
      ? `HTTP ${createNoteMutation.error.status}: ${createNoteMutation.error.body}`
      : createNoteMutation.error instanceof Error
        ? createNoteMutation.error.message
        : null;

  const traceError =
    createTraceMutation.error instanceof ApiClientError
      ? `HTTP ${createTraceMutation.error.status}: ${createTraceMutation.error.body}`
      : createTraceMutation.error instanceof Error
        ? createTraceMutation.error.message
        : null;

  return (
    <main className="min-h-screen bg-slate-50 text-slate-900">
      <div className="mx-auto max-w-7xl px-4 py-4">
        <header className="flex flex-wrap items-end justify-between gap-4 border-b border-slate-300 pb-3">
          <div>
            <h1 className="text-xl font-semibold text-slate-950">
              PMS Console
            </h1>

            <p className="mt-1 text-sm text-slate-600">
              Reservation operational visibility
            </p>
          </div>

          <div className="text-right text-xs text-slate-600">
            Backend:{" "}
            <span className="font-semibold text-slate-900">
              {healthQuery.data?.status ??
                (healthQuery.isLoading ? "checking" : "unavailable")}
            </span>
          </div>
        </header>

        <section className="mt-4 border border-slate-300 bg-white p-3">
          <form
            className="flex flex-wrap items-end gap-3"
            onSubmit={loadReservation}
          >
            <label className="flex min-w-80 flex-1 flex-col gap-1 text-sm text-slate-600">
              Reservation ID
              <input
                className="border border-slate-400 bg-white px-2 py-2 font-mono text-sm text-slate-950 outline-none focus:border-slate-900"
                value={draftReservationId}
                onChange={(event) => setDraftReservationId(event.target.value)}
              />
            </label>

            <button
              className="border border-slate-900 bg-slate-900 px-4 py-2 text-sm font-semibold text-white disabled:border-slate-300 disabled:bg-slate-200 disabled:text-slate-500"
              disabled={draftReservationId.trim().length === 0}
              type="submit"
            >
              Load
            </button>
          </form>
        </section>

        <section className="mt-4">
          {!reservationId && (
            <div className="border border-slate-300 bg-white p-4 text-sm text-slate-600">
              Enter a reservation ID to load the first operational detail slice.
            </div>
          )}

          {reservationQuery.isLoading && (
            <div className="border border-slate-300 bg-white p-4 text-sm">
              Loading reservation...
            </div>
          )}

          {reservationQuery.isError && (
            <pre className="overflow-auto border border-red-300 bg-red-50 p-3 text-xs text-red-800">
              {reservationError}
            </pre>
          )}

          {reservationQuery.data && (
            <ReservationDetailView
              reservation={reservationQuery.data}
              noteError={noteError}
              noteSaving={createNoteMutation.isPending}
              onCreateNote={(request) => createNoteMutation.mutate(request)}
              traceError={traceError}
              traceSaving={createTraceMutation.isPending}
              onCreateTrace={(request) => createTraceMutation.mutate(request)}
            />
          )}
        </section>
      </div>
    </main>
  );
}