import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Link } from "react-router-dom";

import { ApiClientError } from "../../api/client";
import {
  extendNightAuditDeparture,
  finalizeNightAudit,
  getCurrentBusinessDate,
  getNightAuditWorklist,
  markNightAuditArrivalNoShow,
  postNightAuditRoomCharges,
  startNightAudit,
  type NightAuditReservationItem,
  type NightAuditRoomChargeBlocker,
  type NightAuditRoomChargeCandidate,
} from "../../api/businessDate";
import { queryKeys } from "../../api/queryKeys";

function errorMessage(error: unknown) {
  if (error instanceof ApiClientError) {
    return `HTTP ${error.status}: ${error.body}`;
  }

  if (error instanceof Error) {
    return error.message;
  }

  return "Unknown error";
}

function valueOrDash(value: string | null | undefined) {
  return value === null || value === undefined || value === "" ? "-" : value;
}

function countLabel(count: number, noun: string) {
  return `${count} ${noun}${count === 1 ? "" : "s"}`;
}

type ReservationItemsTableProps = {
  title: string;
  items: NightAuditReservationItem[];
  emptyLabel: string;
  actionLabel: string;
  actionPending: boolean;
  onAction: (reservationId: string) => void;
};

function ReservationItemsTable({
  title,
  items,
  emptyLabel,
  actionLabel,
  actionPending,
  onAction,
}: ReservationItemsTableProps) {
  return (
    <section className="border border-slate-300 bg-white">
      <div className="flex items-center justify-between border-b border-slate-200 px-3 py-2">
        <h2 className="text-sm font-semibold text-slate-950">{title}</h2>
        <span className="text-xs font-medium text-slate-500">
          {countLabel(items.length, "item")}
        </span>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full border-collapse text-left text-sm">
          <thead className="bg-slate-100 text-xs uppercase text-slate-600">
            <tr>
              <th className="border-b border-slate-300 px-3 py-2">Reservation</th>
              <th className="border-b border-slate-300 px-3 py-2">External</th>
              <th className="border-b border-slate-300 px-3 py-2">Check-in</th>
              <th className="border-b border-slate-300 px-3 py-2">Check-out</th>
              <th className="border-b border-slate-300 px-3 py-2">Room</th>
              <th className="border-b border-slate-300 px-3 py-2">Action</th>
            </tr>
          </thead>

          <tbody>
            {items.length === 0 ? (
              <tr>
                <td
                  className="px-3 py-4 text-sm text-slate-500"
                  colSpan={6}
                >
                  {emptyLabel}
                </td>
              </tr>
            ) : (
              items.map((item) => (
                <tr key={item.reservation_id}>
                  <td className="border-b border-slate-200 px-3 py-2 font-mono text-xs">
                    <Link
                      to={`/reservations/${item.reservation_id}`}
                      className="font-semibold text-slate-700 underline"
                    >
                      {item.reservation_id}
                    </Link>
                  </td>
                  <td className="border-b border-slate-200 px-3 py-2">
                    {valueOrDash(item.external_id)}
                  </td>
                  <td className="border-b border-slate-200 px-3 py-2">
                    {item.check_in}
                  </td>
                  <td className="border-b border-slate-200 px-3 py-2">
                    {item.check_out}
                  </td>
                  <td className="border-b border-slate-200 px-3 py-2 font-mono text-xs">
                    {valueOrDash(item.room_id)}
                  </td>
                  <td className="border-b border-slate-200 px-3 py-2">
                    <button
                      className="border border-slate-300 bg-white px-2 py-1 text-xs font-semibold text-slate-700 hover:bg-slate-100 disabled:cursor-not-allowed disabled:opacity-50"
                      disabled={actionPending}
                      onClick={() => onAction(item.reservation_id)}
                      type="button"
                    >
                      {actionLabel}
                    </button>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </section>
  );
}

type RoomChargeCandidatesTableProps = {
  candidates: NightAuditRoomChargeCandidate[];
};

function RoomChargeCandidatesTable({
  candidates,
}: RoomChargeCandidatesTableProps) {
  return (
    <section className="border border-slate-300 bg-white">
      <div className="flex items-center justify-between border-b border-slate-200 px-3 py-2">
        <h2 className="text-sm font-semibold text-slate-950">
          Room Charge Candidates
        </h2>
        <span className="text-xs font-medium text-slate-500">
          {countLabel(candidates.length, "item")}
        </span>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full border-collapse text-left text-sm">
          <thead className="bg-slate-100 text-xs uppercase text-slate-600">
            <tr>
              <th className="border-b border-slate-300 px-3 py-2">Reservation</th>
              <th className="border-b border-slate-300 px-3 py-2">Folio</th>
              <th className="border-b border-slate-300 px-3 py-2">Service date</th>
              <th className="border-b border-slate-300 px-3 py-2">Amount</th>
            </tr>
          </thead>

          <tbody>
            {candidates.length === 0 ? (
              <tr>
                <td
                  className="px-3 py-4 text-sm text-slate-500"
                  colSpan={4}
                >
                  No room charges pending.
                </td>
              </tr>
            ) : (
              candidates.map((candidate) => (
                <tr
                  key={`${candidate.reservation_id}-${candidate.service_date}`}
                >
                  <td className="border-b border-slate-200 px-3 py-2 font-mono text-xs">
                    <Link
                      to={`/reservations/${candidate.reservation_id}`}
                      className="font-semibold text-slate-700 underline"
                    >
                      {candidate.reservation_id}
                    </Link>
                  </td>
                  <td className="border-b border-slate-200 px-3 py-2 font-mono text-xs">
                    <Link
                      to={`/folios/${candidate.folio_id}`}
                      className="font-semibold text-slate-700 underline"
                    >
                      {candidate.folio_id}
                    </Link>
                  </td>
                  <td className="border-b border-slate-200 px-3 py-2">
                    {candidate.service_date}
                  </td>
                  <td className="border-b border-slate-200 px-3 py-2">
                    {candidate.amount}
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </section>
  );
}

type RoomChargeBlockersTableProps = {
  blockers: NightAuditRoomChargeBlocker[];
};

function RoomChargeBlockersTable({
  blockers,
}: RoomChargeBlockersTableProps) {
  return (
    <section className="border border-amber-300 bg-white">
      <div className="flex items-center justify-between border-b border-amber-200 px-3 py-2">
        <h2 className="text-sm font-semibold text-amber-900">
          Room Charge Blockers
        </h2>
        <span className="text-xs font-medium text-amber-700">
          {countLabel(blockers.length, "item")}
        </span>
      </div>

      <div className="overflow-x-auto">
        <table className="w-full border-collapse text-left text-sm">
          <thead className="bg-amber-50 text-xs uppercase text-amber-800">
            <tr>
              <th className="border-b border-amber-200 px-3 py-2">Reservation</th>
              <th className="border-b border-amber-200 px-3 py-2">Service date</th>
              <th className="border-b border-amber-200 px-3 py-2">Amount</th>
              <th className="border-b border-amber-200 px-3 py-2">Reason</th>
            </tr>
          </thead>

          <tbody>
            {blockers.length === 0 ? (
              <tr>
                <td
                  className="px-3 py-4 text-sm text-slate-500"
                  colSpan={4}
                >
                  No room charge blockers.
                </td>
              </tr>
            ) : (
              blockers.map((blocker) => (
                <tr
                  key={`${blocker.reservation_id}-${blocker.service_date}-${blocker.reason}`}
                >
                  <td className="border-b border-amber-100 px-3 py-2 font-mono text-xs">
                    <Link
                      to={`/reservations/${blocker.reservation_id}`}
                      className="font-semibold text-amber-800 underline"
                    >
                      {blocker.reservation_id}
                    </Link>
                  </td>
                  <td className="border-b border-amber-100 px-3 py-2">
                    {blocker.service_date}
                  </td>
                  <td className="border-b border-amber-100 px-3 py-2">
                    {blocker.amount}
                  </td>
                  <td className="border-b border-amber-100 px-3 py-2">
                    {blocker.reason}
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </section>
  );
}

export function NightAuditPage() {
  const queryClient = useQueryClient();

  const currentBusinessDateQuery = useQuery({
    queryKey: queryKeys.businessDateCurrent,
    queryFn: getCurrentBusinessDate,
  });

  const worklistQuery = useQuery({
    queryKey: queryKeys.nightAuditWorklist,
    queryFn: getNightAuditWorklist,
  });

  function refreshNightAudit() {
    queryClient.invalidateQueries({
      queryKey: queryKeys.businessDateCurrent,
    });
    queryClient.invalidateQueries({
      queryKey: queryKeys.nightAuditWorklist,
    });
    queryClient.invalidateQueries({
      queryKey: ["reservations"],
    });
    queryClient.invalidateQueries({
      queryKey: ["rooms"],
    });
    queryClient.invalidateQueries({
      queryKey: ["folios"],
    });
  }

  const startMutation = useMutation({
    mutationFn: startNightAudit,
    onSuccess: refreshNightAudit,
  });

  const postRoomChargesMutation = useMutation({
    mutationFn: postNightAuditRoomCharges,
    onSuccess: refreshNightAudit,
  });

  const finalizeMutation = useMutation({
    mutationFn: finalizeNightAudit,
    onSuccess: refreshNightAudit,
  });

  const arrivalNoShowMutation = useMutation({
    mutationFn: markNightAuditArrivalNoShow,
    onSuccess: refreshNightAudit,
  });

  const extendDepartureMutation = useMutation({
    mutationFn: extendNightAuditDeparture,
    onSuccess: refreshNightAudit,
  });

  const actionError =
    startMutation.error ??
    postRoomChargesMutation.error ??
    finalizeMutation.error ??
    arrivalNoShowMutation.error ??
    extendDepartureMutation.error ??
    null;

  const actionPending =
    startMutation.isPending ||
    postRoomChargesMutation.isPending ||
    finalizeMutation.isPending ||
    arrivalNoShowMutation.isPending ||
    extendDepartureMutation.isPending;

  const currentBusinessDate = currentBusinessDateQuery.data;
  const worklist = worklistQuery.data;
  const status = currentBusinessDate?.status;
  const canStart = status === "open";
  const canRunClosingActions = status === "closing";

  return (
    <main className="mx-auto max-w-6xl space-y-4 p-6">
      <section className="border border-slate-300 bg-white p-3">
        <div className="flex flex-wrap items-center justify-between gap-3">
          <div>
            <h1 className="text-base font-semibold text-slate-950">
              Night Audit
            </h1>
            <div className="mt-1 flex flex-wrap items-center gap-2 text-sm text-slate-600">
              <span>
                {currentBusinessDate
                  ? `${currentBusinessDate.business_date} · ${currentBusinessDate.status}`
                  : "Business date unavailable"}
              </span>
              {worklist && (
                <span>
                  {countLabel(worklist.unresolved_arrivals.length, "arrival")} ·{" "}
                  {countLabel(worklist.unresolved_departures.length, "departure")} ·{" "}
                  {countLabel(worklist.room_charge_candidates.length, "charge")} ·{" "}
                  {countLabel(worklist.room_charge_blockers.length, "blocker")}
                </span>
              )}
            </div>
          </div>

          <div className="flex flex-wrap items-center gap-2">
            <button
              className="border border-slate-300 bg-white px-3 py-2 text-sm font-semibold text-slate-700 hover:bg-slate-100 disabled:cursor-not-allowed disabled:opacity-50"
              disabled={actionPending}
              onClick={() => refreshNightAudit()}
              type="button"
            >
              Refresh
            </button>

            <button
              className="border border-slate-900 bg-slate-900 px-3 py-2 text-sm font-semibold text-white hover:bg-slate-700 disabled:cursor-not-allowed disabled:opacity-50"
              disabled={!canStart || actionPending}
              onClick={() => startMutation.mutate()}
              type="button"
            >
              Start
            </button>

            <button
              className="border border-slate-300 bg-white px-3 py-2 text-sm font-semibold text-slate-700 hover:bg-slate-100 disabled:cursor-not-allowed disabled:opacity-50"
              disabled={!canRunClosingActions || actionPending}
              onClick={() => postRoomChargesMutation.mutate()}
              type="button"
            >
              Post Charges
            </button>

            <button
              className="border border-emerald-700 bg-emerald-700 px-3 py-2 text-sm font-semibold text-white hover:bg-emerald-600 disabled:cursor-not-allowed disabled:opacity-50"
              disabled={!canRunClosingActions || actionPending}
              onClick={() => finalizeMutation.mutate()}
              type="button"
            >
              Finalize
            </button>
          </div>
        </div>
      </section>

      {(currentBusinessDateQuery.isError || worklistQuery.isError || actionError) && (
        <pre className="overflow-auto border border-red-300 bg-red-50 p-3 text-xs text-red-800">
          {errorMessage(
            actionError ??
              currentBusinessDateQuery.error ??
              worklistQuery.error,
          )}
        </pre>
      )}

      {(currentBusinessDateQuery.isLoading || worklistQuery.isLoading) && (
        <div className="border border-slate-300 bg-white p-4 text-sm">
          Loading night audit...
        </div>
      )}

      {worklist && (
        <>
          <ReservationItemsTable
            title="Unresolved Arrivals"
            items={worklist.unresolved_arrivals}
            emptyLabel="No unresolved arrivals."
            actionLabel="No-show"
            actionPending={actionPending || !canRunClosingActions}
            onAction={(reservationId) =>
              arrivalNoShowMutation.mutate(reservationId)
            }
          />

          <ReservationItemsTable
            title="Unresolved Departures"
            items={worklist.unresolved_departures}
            emptyLabel="No unresolved departures."
            actionLabel="Extend"
            actionPending={actionPending || !canRunClosingActions}
            onAction={(reservationId) =>
              extendDepartureMutation.mutate(reservationId)
            }
          />

          <RoomChargeBlockersTable
            blockers={worklist.room_charge_blockers}
          />

          <RoomChargeCandidatesTable
            candidates={worklist.room_charge_candidates}
          />
        </>
      )}
    </main>
  );
}
