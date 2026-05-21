import { useState } from "react";
import type { FormEvent } from "react";

import type { SearchReservationsRequest } from "../../api/reservation";

type ReservationSearchFormProps = {
  initialValue: SearchReservationsRequest;
  onSearch: (request: SearchReservationsRequest) => void;
};

export function ReservationSearchForm({
  initialValue,
  onSearch,
}: ReservationSearchFormProps) {
  const [form, setForm] =
    useState<SearchReservationsRequest>(initialValue);

  function updateField(
    key: keyof SearchReservationsRequest,
    value: string,
  ) {
    setForm((current) => ({
      ...current,
      [key]: value,
    }));
  }

  function submit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    onSearch(form);
  }

  function clear() {
    const emptyRequest: SearchReservationsRequest = {};

    setForm(emptyRequest);
    onSearch(emptyRequest);
  }

  return (
    <form
      className="border border-slate-300 bg-white p-3"
      onSubmit={submit}
    >
      <div className="grid gap-3 md:grid-cols-2">
        <label className="flex flex-col gap-1 text-sm text-slate-600">
          Guest name
          <input
            className="border border-slate-400 px-2 py-2 text-sm text-slate-950"
            value={form.guest_name ?? ""}
            onChange={(event) =>
              updateField("guest_name", event.target.value)
            }
          />
        </label>

        <label className="flex flex-col gap-1 text-sm text-slate-600">
          External ID
          <input
            className="border border-slate-400 px-2 py-2 text-sm text-slate-950"
            value={form.external_id ?? ""}
            onChange={(event) =>
              updateField("external_id", event.target.value)
            }
          />
        </label>
      </div>

      <div className="grid gap-3 md:grid-cols-3">
        <label className="flex flex-col gap-1 text-sm text-slate-600">
          Check-in from
          <input
            className="border border-slate-400 px-2 py-2 text-sm text-slate-950"
            type="date"
            value={form.check_in_from ?? ""}
            onChange={(event) =>
              updateField("check_in_from", event.target.value)
            }
          />
        </label>

        <label className="flex flex-col gap-1 text-sm text-slate-600">
          Check-in to
          <input
            className="border border-slate-400 px-2 py-2 text-sm text-slate-950"
            type="date"
            value={form.check_in_to ?? ""}
            onChange={(event) =>
              updateField("check_in_to", event.target.value)
            }
          />
        </label>

        <label className="flex flex-col gap-1 text-sm text-slate-600">
          Stay in
          <input
            className="border border-slate-400 px-2 py-2 text-sm text-slate-950"
            type="date"
            value={form.stay_date ?? ""}
            onChange={(event) =>
              updateField("stay_date", event.target.value)
            }
          />
        </label>
      </div>

      <div className="grid gap-3 md:grid-cols-4">
        <label className="flex flex-col gap-1 text-sm text-slate-600">
          Reservation status
          <select
            className="border border-slate-400 px-2 py-2 text-sm text-slate-950"
            value={form.reservation_status ?? ""}
            onChange={(event) =>
              updateField("reservation_status", event.target.value)
            }
          >
            <option value="">any</option>
            <option value="pending">pending</option>
            <option value="confirmed">confirmed</option>
            <option value="cancelled">cancelled</option>
            <option value="no_show">no_show</option>
            <option value="completed">completed</option>
          </select>
        </label>

        <label className="flex flex-col gap-1 text-sm text-slate-600">
          Stay status
          <select
            className="border border-slate-400 px-2 py-2 text-sm text-slate-950"
            value={form.stay_status ?? ""}
            onChange={(event) =>
              updateField("stay_status", event.target.value)
            }
          >
            <option value="">any</option>
            <option value="confirmed">confirmed</option>
            <option value="checked_in">checked_in</option>
            <option value="checked_out">checked_out</option>
            <option value="no_show">no_show</option>
          </select>
        </label>
        
        <label className="flex flex-col gap-1 text-sm text-slate-600">
          Booking Channel
          <input
            className="border border-slate-400 px-2 py-2 text-sm text-slate-950"
            value={form.booking_channel ?? ""}
            onChange={(event) =>
              updateField("booking_channel", event.target.value)
            }
          />
        </label>
        
        <label className="flex flex-col gap-1 text-sm text-slate-600">
          Source Channel
          <input
            className="border border-slate-400 px-2 py-2 text-sm text-slate-950"
            value={form.source_channel ?? ""}
            onChange={(event) =>
              updateField("source_channel", event.target.value)
            }
          />
        </label>
      </div>

      <div className="grid gap-3 md:grid-cols-3">
        <label className="flex flex-col gap-1 text-sm text-slate-600">
          Reservation status
          <select
            className="border border-slate-400 px-2 py-2 text-sm text-slate-950"
            value={form.reservation_status ?? ""}
            onChange={(event) =>
              updateField("reservation_status", event.target.value)
            }
          >
            <option value="">any</option>
            <option value="pending">pending</option>
            <option value="confirmed">confirmed</option>
            <option value="cancelled">cancelled</option>
            <option value="no_show">no_show</option>
            <option value="completed">completed</option>
          </select>
        </label>
        
        <label className="flex flex-col gap-1 text-sm text-slate-600">
          Booking Channel
          <input
            className="border border-slate-400 px-2 py-2 text-sm text-slate-950"
            value={form.booking_channel ?? ""}
            onChange={(event) =>
              updateField("booking_channel", event.target.value)
            }
          />
        </label>
        
        <label className="flex flex-col gap-1 text-sm text-slate-600">
          Source Channel
          <input
            className="border border-slate-400 px-2 py-2 text-sm text-slate-950"
            value={form.source_channel ?? ""}
            onChange={(event) =>
              updateField("source_channel", event.target.value)
            }
          />
        </label>
      </div>

      <div className="mt-3 flex gap-2">
        <button
          className="border border-slate-900 bg-slate-900 px-4 py-2 text-sm font-semibold text-white"
          type="submit"
        >
          Search
        </button>

        <button
          className="border border-slate-400 bg-white px-4 py-2 text-sm font-semibold text-slate-700"
          type="button"
          onClick={clear}
        >
          Clear
        </button>
      </div>
    </form>
  );
}