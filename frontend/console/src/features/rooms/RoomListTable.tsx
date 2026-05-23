import type { RoomListItem } from "../../api/room";

type RoomListTableProps = {
  rooms: RoomListItem[];
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

export function RoomListTable({ rooms }: RoomListTableProps) {
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
              Occupancy
            </th>
            <th className="border-b border-slate-300 px-3 py-2">
              Housekeeping
            </th>
            <th className="border-b border-slate-300 px-3 py-2">
              Updated At
            </th>
            <th className="border-b border-slate-300 px-3 py-2">
              Capacity
            </th>
            <th className="border-b border-slate-300 px-3 py-2">
              Area
            </th>
            <th className="border-b border-slate-300 px-3 py-2">
              Physical
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
              className="hover:bg-slate-50"
            >
              <td className="border-b border-slate-200 px-3 py-2 font-medium">
                {room.room_no}
              </td>
              <td className="border-b border-slate-200 px-3 py-2">
                {room.room_class}
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
                {formatValue(room.capacity)}
              </td>
              <td className="border-b border-slate-200 px-3 py-2">
                {room.area_sqm}
              </td>
              <td className="border-b border-slate-200 px-3 py-2">
                {formatValue(room.is_physical)}
              </td>
              <td className="border-b border-slate-200 px-3 py-2">
                {formatValue(room.is_active)}
              </td>
            </tr>
          ))}

          {rooms.length === 0 && (
            <tr>
              <td
                colSpan={9}
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