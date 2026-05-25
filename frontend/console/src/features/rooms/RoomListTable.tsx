import { Link } from "react-router-dom";

import type { RoomListItem } from "../../api/room";

type RoomListTableProps = {
  rooms: RoomListItem[];
  onSelect: (roomId: string) => void;
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

function formatDateTime(value: string | null | undefined) {
  if (!value) {
    return "-";
  }

  return value;
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

export function RoomListTable({
  rooms,
  onSelect,
}: RoomListTableProps) {
  return (
    <div className="overflow-auto border border-slate-300 bg-white">
      <table className="min-w-full border-collapse text-left text-sm">
        <thead className="bg-slate-100 text-xs uppercase text-slate-600">
          <tr>
            <th className="border-b border-slate-300 px-3 py-2">
              Room No
            </th>
            <th className="border-b border-slate-300 px-3 py-2">
              Class
            </th>
            <th className="border-b border-slate-300 px-3 py-2">
              Assignment
            </th>
            <th className="border-b border-slate-300 px-3 py-2">
              Reservation
            </th>
            <th className="border-b border-slate-300 px-3 py-2">
              Stay
            </th>
            <th className="border-b border-slate-300 px-3 py-2">
              Warning
            </th>
            <th className="border-b border-slate-300 px-3 py-2">
              Occupancy
            </th>
            <th className="border-b border-slate-300 px-3 py-2">
              Housekeeping
            </th>
            <th className="border-b border-slate-300 px-3 py-2">
              Updated At
            </th>
            <th className="border-b border-slate-300 px-3 py-2">
              Active
            </th>
          </tr>
        </thead>

        <tbody>
          {rooms.map((room) => (
            <tr
              key={room.id}
              className="cursor-pointer hover:bg-slate-50"
              onClick={() => onSelect(room.id)}
            >
              <td className="border-b border-slate-200 px-3 py-2 font-medium underline">
                {room.room_no}
              </td>

              <td className="border-b border-slate-200 px-3 py-2">
                {room.room_class}
              </td>

              <td className="border-b border-slate-200 px-3 py-2">
                {room.assignment.assignment_status}
              </td>

              <td className="border-b border-slate-200 px-3 py-2">
                {room.assignment.reservation_id ? (
                  <Link
                    to={`/reservations/${room.assignment.reservation_id}`}
                    className="font-medium text-slate-700 underline"
                    onClick={(event) => event.stopPropagation()}
                  >
                    {room.assignment.external_id ??
                      room.assignment.reservation_id}
                  </Link>
                ) : (
                  "-"
                )}
              </td>

              <td className="border-b border-slate-200 px-3 py-2">
                {formatValue(room.assignment.stay_status)}
              </td>

              <td className="border-b border-slate-200 px-3 py-2">
                {room.assignment.warning ? (
                  <span className="font-medium text-amber-700">
                    {warningLabel(room.assignment.warning)}
                  </span>
                ) : (
                  "-"
                )}
              </td>

              <td className="border-b border-slate-200 px-3 py-2">
                {formatValue(room.daily_state?.occupancy_status)}
              </td>

              <td className="border-b border-slate-200 px-3 py-2">
                {formatValue(room.daily_state?.housekeeping_status)}
              </td>

              <td className="border-b border-slate-200 px-3 py-2">
                {formatDateTime(room.daily_state?.updated_at)}
              </td>

              <td className="border-b border-slate-200 px-3 py-2">
                {formatValue(room.is_active)}
              </td>
            </tr>
          ))}

          {rooms.length === 0 && (
            <tr>
              <td
                colSpan={10}
                className="px-3 py-6 text-center text-slate-500"
              >
                No rooms found.
              </td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
}