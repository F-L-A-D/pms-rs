import type { SearchReservationsRequest } from "./reservation";

export const queryKeys = {
  health: ["health"] as const,

  reservationDetail: (reservationId: string) =>
    ["reservation", "detail", reservationId] as const,

  reservationSearch: (request: SearchReservationsRequest) =>
    ["reservation", "search", request] as const,
};