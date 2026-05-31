export const queryKeys = {
  health: ["health"] as const,

  businessDateCurrent: ["business-date", "current"] as const,

  nightAuditWorklist: ["business-date", "night-audit", "worklist"] as const,

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

  roomDetail: (roomId: string, params: unknown) =>
    ["rooms", "detail", roomId, params] as const,
};
