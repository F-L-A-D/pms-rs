import type { ReservationSearchItem } from "../../api/reservation";

type ReservationSearchTableProps = {
  items: ReservationSearchItem[];
  onSelect: (reservationId: string) => void;
};

function valueOrDash(value: string | null | undefined) {
  return value === null || value === undefined || value === "" ? "-" : value;
}

export function ReservationSearchTable({
  items,
  onSelect,
}: ReservationSearchTableProps) {
  return (
    <div className="overflow-x-auto border border-slate-300 bg-white">
      <table className="w-full border-collapse text-left text-sm">
        <thead className="bg-slate-100 text-xs uppercase text-slate-600">
          <tr>
            <th className="border-b border-slate-300 px-3 py-2">Reservation ID</th>
            <th className="border-b border-slate-300 px-3 py-2">External ID</th>
            <th className="border-b border-slate-300 px-3 py-2">Guest</th>
            <th className="border-b border-slate-300 px-3 py-2">Check-in</th>
            <th className="border-b border-slate-300 px-3 py-2">Check-out</th>
            <th className="border-b border-slate-300 px-3 py-2">Reservation</th>
            <th className="border-b border-slate-300 px-3 py-2">Stay</th>
            <th className="border-b border-slate-300 px-3 py-2">Room class</th>
            <th className="border-b border-slate-300 px-3 py-2">Room ID</th>
            <th className="border-b border-slate-300 px-3 py-2">Booking Channel</th>
            <th className="border-b border-slate-300 px-3 py-2">Source Channel</th>
            <th className="border-b border-slate-300 px-3 py-2">Primary guest ID</th>
            <th className="border-b border-slate-300 px-3 py-2">Assigned room ID</th>
            <th className="border-b border-slate-300 px-3 py-2">Folio ID</th>
          </tr>
        </thead>

        <tbody>
          {items.length === 0 ? (
            <tr>
              <td
                className="px-3 py-4 text-slate-500"
                colSpan={12}
              >
                No reservations found.
              </td>
            </tr>
          ) : (
            items.map((item) => (
              <tr
                className="cursor-pointer hover:bg-slate-50"
                key={item.id}
                onClick={() => onSelect(item.id)}
              >
                <td className="border-b border-slate-200 px-3 py-2 font-mono text-xs">
                  {item.id}
                </td>
                <td className="border-b border-slate-200 px-3 py-2">
                  {valueOrDash(item.external_id)}
                </td>
                <td className="border-b border-slate-200 px-3 py-2">
                  {valueOrDash(item.primary_guest_name)}
                </td>
                <td className="border-b border-slate-200 px-3 py-2">
                  {item.check_in}
                </td>
                <td className="border-b border-slate-200 px-3 py-2">
                  {item.check_out}
                </td>
                <td className="border-b border-slate-200 px-3 py-2">
                  {item.reservation_status}
                </td>
                <td className="border-b border-slate-200 px-3 py-2">
                  {valueOrDash(item.stay_status)}
                </td>
                <td className="border-b border-slate-200 px-3 py-2">
                  {item.room_class}
                </td>
                <td className="border-b border-slate-200 px-3 py-2 font-mono text-xs">
                  {valueOrDash(item.room_id)}
                </td>
                <td className="border-b border-slate-200 px-3 py-2 font-mono text-xs">
                  {valueOrDash(item.booking_channel)}
                </td>
                <td className="border-b border-slate-200 px-3 py-2 font-mono text-xs">
                  {valueOrDash(item.source_channel)}
                </td>
                <td className="border-b border-slate-200 px-3 py-2 font-mono text-xs">
                  {valueOrDash(item.linked_resources.primary_guest_id)}
                </td>
                <td className="border-b border-slate-200 px-3 py-2 font-mono text-xs">
                  {valueOrDash(item.linked_resources.assigned_room_id)}
                </td>
                <td className="border-b border-slate-200 px-3 py-2 font-mono text-xs">
                  {valueOrDash(item.linked_resources.folio_id)}
                </td>
              </tr>
            ))
          )}
        </tbody>
      </table>
    </div>
  );
}