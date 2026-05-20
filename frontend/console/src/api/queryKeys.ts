export const queryKeys = {
  health: ["health"] as const,
  reservationDetail: (reservationId: string) =>
    ["reservation", "detail", reservationId] as const,
};
