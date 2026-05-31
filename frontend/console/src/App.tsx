import { Link, Route, Routes } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";

import { ReservationDetailPage } from "./features/reservations/ReservationDetailPage";
import { ReservationListPage } from "./features/reservations/ReservationListPage";
import { FolioDetailPage } from "./features/billing/FolioDetailPage";
import { NightAuditPage } from "./features/businessDate/NightAuditPage";
import { RoomDetailPage } from "./features/rooms/RoomDetailPage";
import { RoomListPage } from "./features/rooms/RoomListPage";
import { getCurrentBusinessDate } from "./api/businessDate";
import { queryKeys } from "./api/queryKeys";

function statusClassName(status: string) {
  if (status === "open") {
    return "border-emerald-300 bg-emerald-50 text-emerald-800";
  }

  if (status === "closing") {
    return "border-amber-300 bg-amber-50 text-amber-800";
  }

  return "border-slate-300 bg-slate-50 text-slate-700";
}

function HeaderBusinessDate() {
  const businessDateQuery = useQuery({
    queryKey: queryKeys.businessDateCurrent,
    queryFn: getCurrentBusinessDate,
    refetchInterval: 30_000,
  });

  if (businessDateQuery.isLoading) {
    return (
      <span className="border border-slate-200 bg-slate-50 px-2 py-1 text-xs font-medium text-slate-500">
        Business date loading
      </span>
    );
  }

  if (!businessDateQuery.data) {
    return (
      <span className="border border-red-200 bg-red-50 px-2 py-1 text-xs font-medium text-red-700">
        Business date unavailable
      </span>
    );
  }

  return (
    <Link
      to="/night-audit"
      className={`border px-2 py-1 text-xs font-semibold ${statusClassName(
        businessDateQuery.data.status,
      )}`}
    >
      Business Date {businessDateQuery.data.business_date} ·{" "}
      {businessDateQuery.data.status}
    </Link>
  );
}

function App() {
  return (
    <div className="min-h-screen bg-slate-100 text-slate-900">
      <header className="border-b border-slate-200 bg-white">
        <div className="mx-auto flex max-w-6xl flex-wrap items-center justify-between gap-3 px-6 py-4">
          <Link
            to="/reservations"
            className="text-lg font-semibold"
          >
            PMS Console
          </Link>

          <nav className="flex items-center gap-4 text-sm">
            <Link
              to="/reservations"
              className="font-medium text-slate-600 hover:text-slate-900"
            >
              Reservations
            </Link>

            <Link
              to="/rooms"
              className="font-medium text-slate-600 hover:text-slate-900"
            >
              Rooms
            </Link>

            <Link
              to="/night-audit"
              className="font-medium text-slate-600 hover:text-slate-900"
            >
              Night Audit
            </Link>

            <HeaderBusinessDate />
          </nav>
        </div>
      </header>

      <Routes>
        <Route
          path="/"
          element={<ReservationListPage />}
        />

        <Route
          path="/reservations"
          element={<ReservationListPage />}
        />

        <Route
          path="/reservations/:reservationId"
          element={<ReservationDetailPage />}
        />

        <Route
          path="/folios/:folioId"
          element={<FolioDetailPage />}
        />

        <Route
          path="/rooms"
          element={<RoomListPage />}
        />

        <Route
          path="/rooms/:roomId"
          element={<RoomDetailPage />}
        />

        <Route
          path="/night-audit"
          element={<NightAuditPage />}
        />
      </Routes>
    </div>
  );
}

export default App;
