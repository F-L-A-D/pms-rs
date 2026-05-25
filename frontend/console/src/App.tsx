import { Link, Route, Routes } from "react-router-dom";

import { ReservationDetailPage } from "./features/reservations/ReservationDetailPage";
import { ReservationListPage } from "./features/reservations/ReservationListPage";
import { FolioDetailPage } from "./features/billing/FolioDetailPage";
import { RoomDetailPage } from "./features/rooms/RoomDetailPage";
import { RoomListPage } from "./features/rooms/RoomListPage";

function App() {
  return (
    <div className="min-h-screen bg-slate-100 text-slate-900">
      <header className="border-b border-slate-200 bg-white">
        <div className="mx-auto flex max-w-6xl items-center justify-between px-6 py-4">
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
      </Routes>
    </div>
  );
}

export default App;