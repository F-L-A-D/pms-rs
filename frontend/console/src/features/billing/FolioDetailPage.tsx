import { Link, useParams } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";

import { getFolioDetail } from "../../api/folio";
import { queryKeys } from "../../api/queryKeys";
import { FolioDetailView } from "./FolioDetailView";

export function FolioDetailPage() {
  const { folioId } =
    useParams<{ folioId: string }>();

  const folioQuery =
    useQuery({
      queryKey: queryKeys.folioDetail(folioId ?? ""),
      queryFn: () => getFolioDetail(folioId ?? ""),
      enabled: Boolean(folioId),
    });

  if (!folioId) {
    return (
      <main className="p-6">
        <div className="rounded-lg border border-red-200 bg-red-50 p-4 text-sm text-red-700">
          Folio ID is missing.
        </div>
      </main>
    );
  }

  return (
    <main className="mx-auto max-w-6xl p-6">
      <div className="mb-4">
        <Link
          className="text-sm font-medium text-blue-700 underline"
          to="/reservations"
        >
          Back to reservations
        </Link>
      </div>

      {folioQuery.isLoading && (
        <div className="rounded-lg border border-slate-200 bg-white p-4 text-sm text-slate-500">
          Loading folio detail...
        </div>
      )}

      {folioQuery.isError && (
        <div className="rounded-lg border border-red-200 bg-red-50 p-4 text-sm text-red-700">
          Failed to load folio detail.
        </div>
      )}

      {folioQuery.data && (
        <FolioDetailView detail={folioQuery.data} />
      )}
    </main>
  );
}