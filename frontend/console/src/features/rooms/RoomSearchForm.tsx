import { useState } from "react";
import type { FormEvent } from "react";

import type { ListRoomsRequest } from "../../api/room";

type RoomSearchFormProps = {
  initialValue: ListRoomsRequest;
  onSearch: (value: ListRoomsRequest) => void;
};

function todayString() {
  return new Date().toISOString().slice(0, 10);
}

export function RoomSearchForm({
  initialValue,
  onSearch,
}: RoomSearchFormProps) {
  const [serviceDate, setServiceDate] = useState(
    initialValue.service_date ?? todayString(),
  );

  function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    onSearch({
      service_date: serviceDate,
    });
  }

  return (
    <form
      onSubmit={handleSubmit}
      className="border border-slate-300 bg-white p-3"
    >
      <div className="grid gap-3 md:grid-cols-3">
        <label className="flex flex-col gap-1 text-sm">
          <span className="font-medium text-slate-700">
            Service Date
          </span>

          <input
            type="date"
            value={serviceDate}
            onChange={(event) => setServiceDate(event.target.value)}
            className="border border-slate-300 px-2 py-1"
          />
        </label>

        <div className="flex items-end">
          <button
            type="submit"
            className="border border-slate-900 bg-slate-900 px-3 py-1 text-sm font-medium text-white hover:bg-slate-700"
          >
            Search
          </button>
        </div>
      </div>
    </form>
  );
}