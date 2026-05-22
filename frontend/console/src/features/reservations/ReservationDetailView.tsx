import { useState } from "react";
import type { FormEvent } from "react";
import { Link } from "react-router-dom";
import type {
  CreateReservationNoteRequest,
  CreateReservationTraceRequest,
  ReservationDetail,
} from "../../api/reservation";

type ReservationDetailViewProps = {
  reservation: ReservationDetail;

  noteError: string | null;
  noteSaving: boolean;
  onCreateNote: (request: CreateReservationNoteRequest) => void;

  traceError: string | null;
  traceSaving: boolean;
  onCreateTrace: (request: CreateReservationTraceRequest) => void;
};

function valueOrDash(value: string | number | null | undefined) {
  return value === null || value === undefined || value === "" ? "-" : value;
}

function formatGuestName(
  guest: ReservationDetail["participant_details"][number]["guest"],
) {
  if (!guest) {
    return "-";
  }

  return `${guest.last_name} ${guest.first_name}`;
}

function StatusLabel({ value }: { value: string | null }) {
  return (
    <span className="inline-flex min-w-24 justify-center border border-slate-400 bg-slate-100 px-2 py-1 text-xs font-semibold uppercase text-slate-900">
      {valueOrDash(value)}
    </span>
  );
}

function semanticSummary(
  event: ReservationDetail["operation_events"][number],
) {
  const signal = event.semantic_signal;

  if (signal.semantic_activation) {
    return `${signal.semantic_activation.activation_key} / ${signal.semantic_activation.activation_score}`;
  }

  if (signal.change_pattern) {
    return signal.change_pattern.pattern_type;
  }

  return "no signal";
}

function Field({
  label,
  value,
}: {
  label: string;
  value: string | number | null | undefined;
}) {
  return (
    <div className="grid grid-cols-[9rem_1fr] border-b border-slate-200 py-1.5 text-sm">
      <dt className="text-slate-500">{label}</dt>
      <dd className="break-words font-medium text-slate-950">{valueOrDash(value)}</dd>
    </div>
  );
}

export function ReservationDetailView({
  noteError,
  noteSaving,
  onCreateNote,
  traceError,
  traceSaving,
  onCreateTrace,
  reservation,
}: ReservationDetailViewProps) {
  const room = reservation.room_assignment.room;
  const visibility = reservation.operational_visibility;

  const activeFolios =
    reservation.folios.filter((folio) =>
      folio.status === "open" || folio.status === "locked"
    );

  const otherFolios =
    reservation.folios.filter((folio) =>
      folio.status !== "open" && folio.status !== "locked"
    );
  const [noteBody, setNoteBody] = useState("");
  const [traceDepartmentCode, setTraceDepartmentCode] = useState("");
  const [traceBody, setTraceBody] = useState("");

  function submitNote(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    onCreateNote({
      kind: "global_memo",
      body: noteBody,
      actor_id: "console",
    });

    setNoteBody("");
  }

  function submitTrace(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    onCreateTrace({
      department_code: traceDepartmentCode,
      body: traceBody,
      actor_id: "console",
    });

    setTraceBody("");
  }

  return (
    <div className="space-y-5">
      <section className="border border-slate-300 bg-white">
        <div className="flex flex-wrap items-center justify-between gap-3 border-b border-slate-300 px-4 py-3">
          <div>
            <h2 className="text-base font-semibold text-slate-950">
              Reservation {reservation.id}
            </h2>
            <p className="mt-1 text-xs text-slate-500">
              External ID: {valueOrDash(reservation.external_id)}
            </p>
          </div>

          <div className="flex flex-wrap gap-2">
            <StatusLabel value={reservation.reservation_status} />
            <StatusLabel value={reservation.stay_status} />
          </div>
        </div>

        <dl className="grid gap-x-8 px-4 py-3 md:grid-cols-2">
          <Field label="Check-in" value={reservation.check_in} />
          <Field label="Check-out" value={reservation.check_out} />
          <Field label="Room class" value={reservation.room_class} />
          <Field
            label="Assigned room"
            value={
              room
                ? `${room.room_no} / ${room.room_class}`
                : reservation.room_assignment.room_id
            }
          />
          <Field label="Booking channel" value={reservation.booking_channel} />
          <Field label="Source channel" value={reservation.source_channel} />
          <Field label="Plan code" value={reservation.plan_code} />
          <Field
            label="Primary guest ID"
            value={reservation.linked_resources.primary_guest_id}
          />
          <Field
            label="Assigned room ID"
            value={reservation.linked_resources.assigned_room_id}
          />
          <div className="grid grid-cols-[10rem_1fr] py-2 text-sm">
            <dt className="text-slate-500">
              Folio ID
            </dt>

            <dd>
              {reservation.linked_resources.folio_id ? (
                <Link
                  className="font-medium text-blue-700 underline"
                  to={`/folios/${reservation.linked_resources.folio_id}`}
                >
                  {reservation.linked_resources.folio_id}
                </Link>
              ) : (
                "-"
              )}
            </dd>
          </div>
          <Field label="Version" value={reservation.operation_metadata.version} />
          <Field
            label="Updated at"
            value={reservation.operation_metadata.updated_at}
          />
          <Field label="Created at" value={reservation.created_at} />
          <Field label="Internal note" value={visibility.internal_note} />
        </dl>
      </section>

      <section className="rounded-lg border border-slate-200 bg-white p-4 shadow-sm">
        <h2 className="text-base font-semibold text-slate-900">
          Folios
        </h2>

        <div className="mt-4 grid gap-4 md:grid-cols-2">
          <div>
            <h3 className="text-sm font-medium text-slate-700">
              Active Folios
            </h3>

            {activeFolios.length === 0 ? (
              <div className="mt-2 text-sm text-slate-500">
                -
              </div>
            ) : (
              <div className="mt-2 space-y-2">
                {activeFolios.map((folio) => (
                  <div
                    key={folio.folio_id}
                    className="flex items-center gap-2 text-sm"
                  >
                    <Link
                      className="font-medium text-blue-700 underline"
                      to={`/folios/${folio.folio_id}`}
                    >
                      {folio.folio_id}
                    </Link>

                    <span className="rounded-full bg-slate-100 px-2 py-0.5 text-xs font-medium text-slate-600">
                      {folio.status}
                    </span>
                  </div>
                ))}
              </div>
            )}
          </div>

          <div>
            <h3 className="text-sm font-medium text-slate-700">
              Other Folios
            </h3>

            {otherFolios.length === 0 ? (
              <div className="mt-2 text-sm text-slate-500">
                -
              </div>
            ) : (
              <div className="mt-2 space-y-2">
                {otherFolios.map((folio) => (
                  <div
                    key={folio.folio_id}
                    className="flex items-center gap-2 text-sm"
                  >
                    <Link
                      className="font-medium text-blue-700 underline"
                      to={`/folios/${folio.folio_id}`}
                    >
                      {folio.folio_id}
                    </Link>

                    <span className="rounded-full bg-slate-100 px-2 py-0.5 text-xs font-medium text-slate-600">
                      {folio.status}
                    </span>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      </section>

      <section className="border border-slate-300 bg-white">
        <h3 className="border-b border-slate-300 px-4 py-2 text-sm font-semibold text-slate-950">
          Guests / Participants
        </h3>

        <div className="overflow-x-auto">
          <table className="w-full border-collapse text-left text-sm">
            <thead className="bg-slate-100 text-xs uppercase text-slate-600">
              <tr>
                <th className="border-b border-slate-300 px-4 py-2 font-semibold">Role</th>
                <th className="border-b border-slate-300 px-4 py-2 font-semibold">Guest</th>
                <th className="border-b border-slate-300 px-4 py-2 font-semibold">Guest ID</th>
                <th className="border-b border-slate-300 px-4 py-2 font-semibold">Contact</th>
              </tr>
            </thead>
            <tbody>
              {reservation.participant_details.map((participant) => (
                <tr key={participant.guest_id}>
                  <td className="border-b border-slate-200 px-4 py-2">
                    {participant.relation_type}
                  </td>
                  <td className="border-b border-slate-200 px-4 py-2 font-medium text-slate-950">
                    {formatGuestName(participant.guest)}
                  </td>
                  <td className="border-b border-slate-200 px-4 py-2 font-mono text-xs">
                    {participant.guest_id}
                  </td>
                  <td className="border-b border-slate-200 px-4 py-2">
                    {valueOrDash(
                      participant.guest?.email ?? participant.guest?.phone,
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </section>

      <div className="grid gap-5 lg:grid-cols-2">
        <section className="border border-slate-300 bg-white">
          <h3 className="border-b border-slate-300 px-4 py-2 text-sm font-semibold text-slate-950">
            Package / Rate
          </h3>

          <div className="overflow-x-auto">
            <table className="w-full border-collapse text-left text-sm">
              <thead className="bg-slate-100 text-xs uppercase text-slate-600">
                <tr>
                  <th className="border-b border-slate-300 px-4 py-2">Package</th>
                  <th className="border-b border-slate-300 px-4 py-2">Category</th>
                  <th className="border-b border-slate-300 px-4 py-2 text-right">Amount</th>
                </tr>
              </thead>
              <tbody>
                {reservation.package_breakdowns.length === 0 ? (
                  <tr>
                    <td className="px-4 py-3 text-slate-500" colSpan={3}>
                      No package breakdowns
                    </td>
                  </tr>
                ) : (
                  reservation.package_breakdowns.map((breakdown) => (
                    <tr key={`${breakdown.package_code}-${breakdown.revenue_category}`}>
                      <td className="border-b border-slate-200 px-4 py-2">
                        {breakdown.package_code}
                      </td>
                      <td className="border-b border-slate-200 px-4 py-2">
                        {breakdown.revenue_category}
                      </td>
                      <td className="border-b border-slate-200 px-4 py-2 text-right tabular-nums">
                        {breakdown.amount}
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </section>

        <section className="border border-slate-300 bg-white">
          <h3 className="border-b border-slate-300 px-4 py-2 text-sm font-semibold text-slate-950">
            Daily Details
          </h3>

          <div className="overflow-x-auto">
            <table className="w-full border-collapse text-left text-sm">
              <thead className="bg-slate-100 text-xs uppercase text-slate-600">
                <tr>
                  <th className="border-b border-slate-300 px-4 py-2">Date</th>
                  <th className="border-b border-slate-300 px-4 py-2">Class</th>
                  <th className="border-b border-slate-300 px-4 py-2">Plan</th>
                  <th className="border-b border-slate-300 px-4 py-2 text-right">Pax</th>
                  <th className="border-b border-slate-300 px-4 py-2">Sleep sharing</th>
                </tr>
              </thead>
              <tbody>
                {reservation.daily_details.length === 0 ? (
                  <tr>
                    <td className="px-4 py-3 text-slate-500" colSpan={5}>
                      No daily details
                    </td>
                  </tr>
                ) : (
                  reservation.daily_details.map((detail) => (
                    <tr key={detail.service_date}>
                      <td className="border-b border-slate-200 px-4 py-2">
                        {detail.service_date}
                      </td>
                      <td className="border-b border-slate-200 px-4 py-2">
                        {detail.room_class}
                      </td>
                      <td className="border-b border-slate-200 px-4 py-2">
                        {valueOrDash(detail.plan_code)}
                      </td>
                      <td className="border-b border-slate-200 px-4 py-2 text-right tabular-nums">
                        {detail.adult_count} / {detail.child_count}
                      </td>
                      <td className="border-b border-slate-200 px-4 py-2">
                        <div className="font-medium text-slate-950">
                          {detail.sleep_sharing_child_count}
                        </div>
                        {detail.sleep_sharing_children.length > 0 && (
                          <div className="mt-1 space-y-1 text-xs text-slate-600">
                            {detail.sleep_sharing_children.map((child, index) => (
                              <div key={`${detail.service_date}-${index}`}>
                                {valueOrDash(child.name)} / {valueOrDash(child.age)} /{" "}
                                {valueOrDash(child.gender)}
                              </div>
                            ))}
                          </div>
                        )}
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </section>
      </div>

      <section className="border border-slate-300 bg-white">
        <h3 className="border-b border-slate-300 px-4 py-2 text-sm font-semibold text-slate-950">
          Conflict / Room History
        </h3>

        <div className="grid gap-5 p-4 lg:grid-cols-2">
          <div className="overflow-x-auto">
            <h4 className="mb-2 text-xs font-semibold uppercase text-slate-600">
              Active edit sessions
            </h4>
            <table className="w-full border-collapse text-left text-sm">
              <thead className="bg-slate-100 text-xs uppercase text-slate-600">
                <tr>
                  <th className="border-b border-slate-300 px-3 py-2">Actor</th>
                  <th className="border-b border-slate-300 px-3 py-2">Opened</th>
                  <th className="border-b border-slate-300 px-3 py-2">Expires</th>
                </tr>
              </thead>
              <tbody>
                {reservation.active_edit_sessions.length === 0 ? (
                  <tr>
                    <td className="px-3 py-3 text-slate-500" colSpan={3}>
                      No active edit sessions
                    </td>
                  </tr>
                ) : (
                  reservation.active_edit_sessions.map((session) => (
                    <tr key={session.id}>
                      <td className="border-b border-slate-200 px-3 py-2">
                        {session.actor_label ?? session.actor_id}
                      </td>
                      <td className="border-b border-slate-200 px-3 py-2 text-xs">
                        {session.opened_at}
                      </td>
                      <td className="border-b border-slate-200 px-3 py-2 text-xs">
                        {session.expires_at}
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>

          <div className="overflow-x-auto">
            <h4 className="mb-2 text-xs font-semibold uppercase text-slate-600">
              Room history
            </h4>
            <table className="w-full border-collapse text-left text-sm">
              <thead className="bg-slate-100 text-xs uppercase text-slate-600">
                <tr>
                  <th className="border-b border-slate-300 px-3 py-2">Type</th>
                  <th className="border-b border-slate-300 px-3 py-2">From</th>
                  <th className="border-b border-slate-300 px-3 py-2">To</th>
                  <th className="border-b border-slate-300 px-3 py-2">At</th>
                </tr>
              </thead>
              <tbody>
                {reservation.room_history.length === 0 ? (
                  <tr>
                    <td className="px-3 py-3 text-slate-500" colSpan={4}>
                      No room history
                    </td>
                  </tr>
                ) : (
                  reservation.room_history.map((history) => (
                    <tr key={history.id}>
                      <td className="border-b border-slate-200 px-3 py-2">
                        {history.transition_type}
                      </td>
                      <td className="border-b border-slate-200 px-3 py-2 font-mono text-xs">
                        {valueOrDash(history.before_room_id)}
                      </td>
                      <td className="border-b border-slate-200 px-3 py-2 font-mono text-xs">
                        {valueOrDash(history.after_room_id)}
                      </td>
                      <td className="border-b border-slate-200 px-3 py-2 text-xs">
                        {history.occurred_at}
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </div>
      </section>

      <section className="border border-slate-300 bg-white">
        <h3 className="border-b border-slate-300 px-4 py-2 text-sm font-semibold text-slate-950">
          Memo
        </h3>

        <div className="grid gap-4 p-4 lg:grid-cols-[22rem_1fr]">
          <form className="space-y-3" onSubmit={submitNote}>
            <label className="flex flex-col gap-1 text-sm text-slate-600">
              Body
              <textarea
                className="min-h-24 border border-slate-400 bg-white px-2 py-2 text-sm text-slate-950"
                value={noteBody}
                onChange={(event) => setNoteBody(event.target.value)}
              />
            </label>

            <button
              className="border border-slate-900 bg-slate-900 px-4 py-2 text-sm font-semibold text-white disabled:border-slate-300 disabled:bg-slate-200 disabled:text-slate-500"
              disabled={noteSaving || noteBody.trim().length === 0}
              type="submit"
            >
              Add memo
            </button>

            {noteError && <div className="text-xs text-red-700">{noteError}</div>}
          </form>

          <div className="overflow-x-auto">
            <table className="w-full border-collapse text-left text-sm">
              <thead className="bg-slate-100 text-xs uppercase text-slate-600">
                <tr>
                  <th className="border-b border-slate-300 px-4 py-2">Body</th>
                  <th className="border-b border-slate-300 px-4 py-2">Actor</th>
                  <th className="border-b border-slate-300 px-4 py-2">At</th>
                </tr>
              </thead>
              <tbody>
                {reservation.notes.length === 0 ? (
                  <tr>
                    <td className="px-4 py-3 text-slate-500" colSpan={3}>
                      No memo
                    </td>
                  </tr>
                ) : (
                  reservation.notes.map((note) => (
                    <tr key={note.id}>
                      <td className="border-b border-slate-200 px-4 py-2 text-slate-950">
                        {note.body}
                      </td>
                      <td className="border-b border-slate-200 px-4 py-2">
                        {valueOrDash(note.actor_id)}
                      </td>
                      <td className="border-b border-slate-200 px-4 py-2 text-xs">
                        {note.created_at}
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </div>
      </section>

      <section className="border border-slate-300 bg-white">
        <h3 className="border-b border-slate-300 px-4 py-2 text-sm font-semibold text-slate-950">
          Trace
        </h3>

        <div className="grid gap-4 p-4 lg:grid-cols-[22rem_1fr]">
          <form className="space-y-3" onSubmit={submitTrace}>
            <label className="flex flex-col gap-1 text-sm text-slate-600">
              Department
              <input
                className="border border-slate-400 bg-white px-2 py-2 text-sm text-slate-950"
                value={traceDepartmentCode}
                onChange={(event) => setTraceDepartmentCode(event.target.value)}
              />
            </label>

            <label className="flex flex-col gap-1 text-sm text-slate-600">
              Body
              <textarea
                className="min-h-24 border border-slate-400 bg-white px-2 py-2 text-sm text-slate-950"
                value={traceBody}
                onChange={(event) => setTraceBody(event.target.value)}
              />
            </label>

            <button
              className="border border-slate-900 bg-slate-900 px-4 py-2 text-sm font-semibold text-white disabled:border-slate-300 disabled:bg-slate-200 disabled:text-slate-500"
              disabled={
                traceSaving ||
                traceBody.trim().length === 0 ||
                traceDepartmentCode.trim().length === 0
              }
              type="submit"
            >
              Add trace
            </button>

            {traceError && <div className="text-xs text-red-700">{traceError}</div>}
          </form>

          <div className="overflow-x-auto">
            <table className="w-full border-collapse text-left text-sm">
              <thead className="bg-slate-100 text-xs uppercase text-slate-600">
                <tr>
                  <th className="border-b border-slate-300 px-4 py-2">Department</th>
                  <th className="border-b border-slate-300 px-4 py-2">Body</th>
                  <th className="border-b border-slate-300 px-4 py-2">Status</th>
                  <th className="border-b border-slate-300 px-4 py-2">At</th>
                </tr>
              </thead>
              <tbody>
                {reservation.traces.length === 0 ? (
                  <tr>
                    <td className="px-4 py-3 text-slate-500" colSpan={4}>
                      No trace
                    </td>
                  </tr>
                ) : (
                  reservation.traces.map((trace) => (
                    <tr key={trace.id}>
                      <td className="border-b border-slate-200 px-4 py-2">
                        {valueOrDash(trace.department_code)}
                      </td>
                      <td className="border-b border-slate-200 px-4 py-2 text-slate-950">
                        {trace.body}
                      </td>
                      <td className="border-b border-slate-200 px-4 py-2">
                        {trace.resolved_at ? "resolved" : "open"}
                      </td>
                      <td className="border-b border-slate-200 px-4 py-2 text-xs">
                        {trace.created_at}
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </div>
      </section>

      <section className="border border-slate-300 bg-white">
        <h3 className="border-b border-slate-300 px-4 py-2 text-sm font-semibold text-slate-950">
          Audit / Timeline / Semantic
        </h3>
        <div className="grid gap-x-8 px-4 py-3 md:grid-cols-3">
          <Field
            label="Audit"
            value={visibility.audit_trail_available ? "available" : "placeholder"}
          />
          <Field
            label="Timeline"
            value={visibility.timeline_available ? "available" : "placeholder"}
          />
          <Field
            label="Semantic signal"
            value={visibility.semantic_signal_available ? "available" : "placeholder"}
          />
        </div>

        <div className="grid gap-5 border-t border-slate-300 p-4 lg:grid-cols-2">
          <div className="overflow-x-auto">
            <h4 className="mb-2 text-xs font-semibold uppercase text-slate-600">
              Operation events
            </h4>
            <table className="w-full border-collapse text-left text-sm">
              <thead className="bg-slate-100 text-xs uppercase text-slate-600">
                <tr>
                  <th className="border-b border-slate-300 px-3 py-2">Type</th>
                  <th className="border-b border-slate-300 px-3 py-2">Fields</th>
                  <th className="border-b border-slate-300 px-3 py-2">Semantic</th>
                  <th className="border-b border-slate-300 px-3 py-2">At</th>
                </tr>
              </thead>
              <tbody>
                {reservation.operation_events.length === 0 ? (
                  <tr>
                    <td className="px-3 py-3 text-slate-500" colSpan={4}>
                      No operation events
                    </td>
                  </tr>
                ) : (
                  reservation.operation_events.map((event) => (
                    <tr key={event.id}>
                      <td className="border-b border-slate-200 px-3 py-2">
                        {event.operation_type}
                      </td>
                      <td className="border-b border-slate-200 px-3 py-2">
                        {event.changed_fields.length === 0
                          ? "-"
                          : event.changed_fields
                              .map((field) => field.field_name)
                              .join(", ")}
                      </td>
                      <td className="border-b border-slate-200 px-3 py-2">
                        {semanticSummary(event)}
                      </td>
                      <td className="border-b border-slate-200 px-3 py-2 text-xs">
                        {event.occurred_at}
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>

          <div className="overflow-x-auto">
            <h4 className="mb-2 text-xs font-semibold uppercase text-slate-600">
              Audit log
            </h4>
            <table className="w-full border-collapse text-left text-sm">
              <thead className="bg-slate-100 text-xs uppercase text-slate-600">
                <tr>
                  <th className="border-b border-slate-300 px-3 py-2">Action</th>
                  <th className="border-b border-slate-300 px-3 py-2">Actor</th>
                  <th className="border-b border-slate-300 px-3 py-2">Source</th>
                  <th className="border-b border-slate-300 px-3 py-2">At</th>
                </tr>
              </thead>
              <tbody>
                {reservation.audit_logs.length === 0 ? (
                  <tr>
                    <td className="px-3 py-3 text-slate-500" colSpan={4}>
                      No audit logs
                    </td>
                  </tr>
                ) : (
                  reservation.audit_logs.map((log) => (
                    <tr key={log.id}>
                      <td className="border-b border-slate-200 px-3 py-2">
                        {log.action}
                      </td>
                      <td className="border-b border-slate-200 px-3 py-2">
                        {log.actor_id ?? log.actor}
                      </td>
                      <td className="border-b border-slate-200 px-3 py-2">
                        {log.source}
                      </td>
                      <td className="border-b border-slate-200 px-3 py-2 text-xs">
                        {log.occurred_at}
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </div>
      </section>
    </div>
  );
}
