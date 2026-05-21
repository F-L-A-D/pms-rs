import { useQuery } from "@tanstack/react-query";
import {
  BrowserRouter,
  Link,
  Navigate,
  Route,
  Routes,
} from "react-router-dom";

import { getHealth } from "./api/health";
import { queryKeys } from "./api/queryKeys";
import { ReservationDetailPage } from "./features/reservations/ReservationDetailPage";
import { ReservationListPage } from "./features/reservations/ReservationListPage";

export default function App() {
  const healthQuery = useQuery({
    queryKey: queryKeys.health,
    queryFn: getHealth,
  });

  return (
    <BrowserRouter>
      <main className="min-h-screen bg-slate-50 text-slate-900">
        <div className="mx-auto max-w-7xl px-4 py-4">
          <header className="flex flex-wrap items-end justify-between gap-4 border-b border-slate-300 pb-3">
            <div>
              <h1 className="text-xl font-semibold text-slate-950">
                PMS Console
              </h1>

              <p className="mt-1 text-sm text-slate-600">
                Operational validation surface
              </p>

              <nav className="mt-2 flex gap-3 text-sm">
                <Link
                  className="font-semibold text-slate-700 underline"
                  to="/reservations"
                >
                  Reservations
                </Link>
              </nav>
            </div>

            <div className="text-right text-xs text-slate-600">
              Backend:{" "}
              <span className="font-semibold text-slate-900">
                {healthQuery.data?.status ??
                  (healthQuery.isLoading ? "checking" : "unavailable")}
              </span>
            </div>
          </header>

          <section className="mt-4">
            <Routes>
              <Route
                path="/"
                element={<Navigate replace to="/reservations" />}
              />
              <Route
                path="/reservations"
                element={<ReservationListPage />}
              />
              <Route
                path="/reservations/:reservationId"
                element={<ReservationDetailPage />}
              />
            </Routes>
          </section>
        </div>
      </main>
    </BrowserRouter>
  );
}