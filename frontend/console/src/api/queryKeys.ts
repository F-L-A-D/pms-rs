export const queryKeys = {
  health: ["health"] as const,

  reservationSearch: (params: unknown) =>
    ["reservations", "search", params] as const,

  reservationDetail: (reservationId: string) =>
    ["reservations", "detail", reservationId] as const,

  folioDetail: (folioId: string) =>
    ["folios", "detail", folioId] as const,

  folioAudit: (folioId: string) =>
    ["folios", "audit", folioId] as const,

  roomList: (params: unknown) =>
    ["rooms", "list", params] as const,
};