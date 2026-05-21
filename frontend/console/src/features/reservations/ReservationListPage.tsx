import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";

import { ApiClientError } from "../../api/client";
import { queryKeys } from "../../api/queryKeys";
import {
  searchReservations,
  type SearchReservationsRequest,
} from "../../api/reservation";
import { ReservationSearchForm } from "./ReservationSearchForm";
import { ReservationSearchTable } from "./ReservationSearchTable";

function errorMessage(error: unknown) {
  if (error instanceof ApiClientError) {
    return `HTTP ${error.status}: ${error.body}`;
  }

  if (error instanceof Error) {
    return error.message;
  }

  return "Unknown error";
}

export function ReservationListPage() {
  const navigate = useNavigate();

  const [searchRequest, setSearchRequest] =
    useState<SearchReservationsRequest>({});

  const reservationSearchQuery = useQuery({
    queryKey: queryKeys.reservationSearch(searchRequest),
    queryFn: () => searchReservations(searchRequest),
  });

  return (
    <div className="space-y-4">
      <section className="border border-slate-300 bg-white p-3">
        <h2 className="text-base font-semibold text-slate-950">
          Reservation Search
        </h2>

        <p className="mt-1 text-sm text-slate-600">
          Console surface for exposing missing reservation search/detail logic.
        </p>
      </section>

      <ReservationSearchForm
        initialValue={searchRequest}
        onSearch={setSearchRequest}
      />

      {reservationSearchQuery.isLoading && (
        <div className="border border-slate-300 bg-white p-4 text-sm">
          Loading reservations...
        </div>
      )}

      {reservationSearchQuery.isError && (
        <pre className="overflow-auto border border-red-300 bg-red-50 p-3 text-xs text-red-800">
          {errorMessage(reservationSearchQuery.error)}
        </pre>
      )}

      {reservationSearchQuery.data && (
        <ReservationSearchTable
          items={reservationSearchQuery.data}
          onSelect={(reservationId) =>
            navigate(`/reservations/${reservationId}`)
          }
        />
      )}
    </div>
  );
}