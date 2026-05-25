import { Link } from "react-router-dom";

import type { RoomDetail } from "../../api/room";

type RoomDetailViewProps = {
  room: RoomDetail;
  serviceDate: string;
  actionError: string | null;
  actionSaving: boolean;
  onMarkDirty: () => void;
  onStartCleaning: () => void;
  onFinishCleaning: () => void;
  onInspect: () => void;
  onMarkOutOfOrder: () => void;
  onReturnToService: () => void;
};

function formatValue(value: string | number | boolean | null | undefined) {
  if (value === null || value === undefined || value === "") {
    return "-";
  }

  if (typeof value === "boolean") {
    return value ? "yes" : "no";
  }

  return String(value);
}

function warningLabel(value: string | null) {
  if (!value) {
    return "-";
  }

  if (value === "no_show_reservation_still_linked_to_room") {
    return "No-show linked room";
  }

  return value;
}

function ActionButton({
  children,
  disabled,
  onClick,
}: {
  children: string;
  disabled: boolean;
  onClick: () => void;
}) {
  return (
    <button
      type="button"
      disabled={disabled}
      onClick={onClick}
      className="border border-slate-900 bg-slate-900 px-3 py-1 text-sm font-medium text-white hover:bg-slate-700 disabled:cursor-not-allowed disabled:border-slate-300 disabled:bg-slate-300"
    >
      {children}
    </button>
  );
}

export function RoomDetailView({
  room,
  serviceDate,
  actionError,
  actionSaving,
  onMarkDirty,
  onStartCleaning,
  onFinishCleaning,
  onInspect,
  onMarkOutOfOrder,
  onReturnToService,
}: RoomDetailViewProps) {
  return (
    <div className="space-y-4">
      <section className="border border-slate-300 bg-white p-4">
        <div className="flex items-start justify-between gap-4">
          <div>
            <h2 className="text-lg font-semibold text-slate-950">
              Room {room.room_no}
            </h2>

            <p className="mt-1 text-sm text-slate-600">
              Room daily state validation for {serviceDate}.
            </p>
          </div>

          <div className="text-right text-sm text-slate-600">
            <div>Class: {room.room_class}</div>
            <div>Active: {formatValue(room.is_active)}</div>
          </div>
        </div>
      </section>

      <section className="grid gap-4 md:grid-cols-3">
        <div className="border border-slate-300 bg-white p-4">
          <h3 className="text-sm font-semibold uppercase text-slate-500">
            Room
          </h3>

          <dl className="mt-3 grid grid-cols-2 gap-2 text-sm">
            <dt className="text-slate-500">Room ID</dt>
            <dd className="break-all">{room.id}</dd>

            <dt className="text-slate-500">Room No</dt>
            <dd>{room.room_no}</dd>

            <dt className="text-slate-500">Class</dt>
            <dd>{room.room_class}</dd>

            <dt className="text-slate-500">Capacity</dt>
            <dd>{formatValue(room.capacity)}</dd>

            <dt className="text-slate-500">Area</dt>
            <dd>{room.area_sqm}</dd>

            <dt className="text-slate-500">Physical</dt>
            <dd>{formatValue(room.is_physical)}</dd>

            <dt className="text-slate-500">Active</dt>
            <dd>{formatValue(room.is_active)}</dd>
          </dl>
        </div>

        <div className="border border-slate-300 bg-white p-4">
          <h3 className="text-sm font-semibold uppercase text-slate-500">
            Assignment
          </h3>

          <dl className="mt-3 grid grid-cols-2 gap-2 text-sm">
            <dt className="text-slate-500">Status</dt>
            <dd>{room.assignment.assignment_status}</dd>

            <dt className="text-slate-500">Reservation</dt>
            <dd>
              {room.assignment.reservation_id ? (
                <Link
                  to={`/reservations/${room.assignment.reservation_id}`}
                  className="font-medium text-slate-700 underline"
                >
                  {room.assignment.external_id ??
                    room.assignment.reservation_id}
                </Link>
              ) : (
                "-"
              )}
            </dd>

            <dt className="text-slate-500">Reservation Status</dt>
            <dd>{formatValue(room.assignment.reservation_status)}</dd>

            <dt className="text-slate-500">Stay Status</dt>
            <dd>{formatValue(room.assignment.stay_status)}</dd>

            <dt className="text-slate-500">Check In</dt>
            <dd>{formatValue(room.assignment.check_in)}</dd>

            <dt className="text-slate-500">Check Out</dt>
            <dd>{formatValue(room.assignment.check_out)}</dd>

            <dt className="text-slate-500">Warning</dt>
            <dd>
              {room.assignment.warning ? (
                <span className="font-medium text-amber-700">
                  {warningLabel(room.assignment.warning)}
                </span>
              ) : (
                "-"
              )}
            </dd>
          </dl>
        </div>

        <div className="border border-slate-300 bg-white p-4">
          <h3 className="text-sm font-semibold uppercase text-slate-500">
            Daily State
          </h3>

          {room.daily_state ? (
            <dl className="mt-3 grid grid-cols-2 gap-2 text-sm">
              <dt className="text-slate-500">Service Date</dt>
              <dd>{room.daily_state.service_date}</dd>

              <dt className="text-slate-500">Occupancy</dt>
              <dd>{room.daily_state.occupancy_status}</dd>

              <dt className="text-slate-500">Housekeeping</dt>
              <dd>{room.daily_state.housekeeping_status}</dd>

              <dt className="text-slate-500">Updated At</dt>
              <dd>{room.daily_state.updated_at}</dd>
            </dl>
          ) : (
            <p className="mt-3 text-sm text-slate-600">
              No room daily state exists for this service date.
            </p>
          )}
        </div>
      </section>

      <section className="border border-slate-300 bg-white p-4">
        <h3 className="text-sm font-semibold uppercase text-slate-500">
          Housekeeping Actions
        </h3>

        <div className="mt-3 flex flex-wrap gap-2">
          <ActionButton
            disabled={actionSaving}
            onClick={onMarkDirty}
          >
            Mark Dirty
          </ActionButton>

          <ActionButton
            disabled={actionSaving}
            onClick={onStartCleaning}
          >
            Start Cleaning
          </ActionButton>

          <ActionButton
            disabled={actionSaving}
            onClick={onFinishCleaning}
          >
            Finish Cleaning
          </ActionButton>

          <ActionButton
            disabled={actionSaving}
            onClick={onInspect}
          >
            Inspect
          </ActionButton>
        </div>
      </section>

      <section className="border border-slate-300 bg-white p-4">
        <h3 className="text-sm font-semibold uppercase text-slate-500">
          Maintenance Actions
        </h3>

        <div className="mt-3 flex flex-wrap gap-2">
          <ActionButton
            disabled={actionSaving}
            onClick={onMarkOutOfOrder}
          >
            Mark Out Of Order
          </ActionButton>

          <ActionButton
            disabled={actionSaving}
            onClick={onReturnToService}
          >
            Return To Service
          </ActionButton>
        </div>
      </section>

      {actionError && (
        <pre className="overflow-auto border border-red-300 bg-red-50 p-3 text-xs text-red-800">
          {actionError}
        </pre>
      )}
    </div>
  );
}