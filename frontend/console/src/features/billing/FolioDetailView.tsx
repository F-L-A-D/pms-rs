import { Link } from "react-router-dom";

import type {
  BillingAudit,
  FolioDetail,
} from "../../api/folio";

type Props = {
  detail: FolioDetail;
  audits: BillingAudit[];
  isAuditLoading: boolean;
  isAuditError: boolean;
};

function formatDateTime(value: string): string {
  return new Date(value).toLocaleString();
}

function formatAmount(value: string): string {
  return Number(value).toLocaleString();
}

export function FolioDetailView({
  detail,
  audits,
  isAuditLoading,
  isAuditError,
}: Props) {
  const { folio, entries } = detail;

  return (
    <div className="space-y-6">
      <section className="rounded-lg border border-slate-200 bg-white p-4 shadow-sm">
        <div className="mb-4 flex items-start justify-between gap-4">
          <div>
            <h1 className="text-xl font-semibold text-slate-900">
              Folio Detail
            </h1>

            <p className="mt-1 text-sm text-slate-500">
              {folio.id}
            </p>
          </div>

          <span className="rounded-full bg-slate-100 px-3 py-1 text-sm font-medium text-slate-700">
            {folio.status}
          </span>
        </div>

        <dl className="divide-y divide-slate-200">
          <div className="grid grid-cols-[10rem_1fr] py-2 text-sm">
            <dt className="text-slate-500">
              Reservation ID
            </dt>

            <dd>
              <Link
                className="font-medium text-blue-700 underline"
                to={`/reservations/${folio.reservation_id}`}
              >
                {folio.reservation_id}
              </Link>
            </dd>
          </div>

          <div className="grid grid-cols-[10rem_1fr] py-2 text-sm">
            <dt className="text-slate-500">
              Billing Account
            </dt>

            <dd>
              {folio.billing_account_id ?? "-"}
            </dd>
          </div>

          <div className="grid grid-cols-[10rem_1fr] py-2 text-sm">
            <dt className="text-slate-500">
              Created At
            </dt>

            <dd>
              {formatDateTime(folio.created_at)}
            </dd>
          </div>
        </dl>
      </section>

      <section className="grid gap-4 md:grid-cols-3">
        <div className="rounded-lg border border-slate-200 bg-white p-4 shadow-sm">
          <div className="text-sm text-slate-500">
            Total Charges
          </div>

          <div className="mt-2 text-2xl font-semibold text-slate-900">
            {formatAmount(detail.total_charges)}
          </div>
        </div>

        <div className="rounded-lg border border-slate-200 bg-white p-4 shadow-sm">
          <div className="text-sm text-slate-500">
            Total Payments
          </div>

          <div className="mt-2 text-2xl font-semibold text-slate-900">
            {formatAmount(detail.total_payments)}
          </div>
        </div>

        <div className="rounded-lg border border-slate-200 bg-white p-4 shadow-sm">
          <div className="text-sm text-slate-500">
            Balance
          </div>

          <div className="mt-2 text-2xl font-semibold text-slate-900">
            {formatAmount(detail.balance)}
          </div>
        </div>
      </section>

      <section className="rounded-lg border border-slate-200 bg-white shadow-sm">
        <div className="border-b border-slate-200 px-4 py-3">
          <h2 className="text-base font-semibold text-slate-900">
            Entries
          </h2>
        </div>

        {entries.length === 0 ? (
          <div className="p-4 text-sm text-slate-500">
            No folio entries.
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-slate-200 text-sm">
              <thead className="bg-slate-50">
                <tr>
                  <th className="px-4 py-2 text-left font-medium text-slate-500">
                    Occurred At
                  </th>

                  <th className="px-4 py-2 text-left font-medium text-slate-500">
                    Type
                  </th>

                  <th className="px-4 py-2 text-right font-medium text-slate-500">
                    Amount
                  </th>

                  <th className="px-4 py-2 text-left font-medium text-slate-500">
                    Memo
                  </th>
                </tr>
              </thead>

              <tbody className="divide-y divide-slate-200 bg-white">
                {entries.map((entry) => (
                  <tr key={entry.id}>
                    <td className="whitespace-nowrap px-4 py-2 text-slate-700">
                      {formatDateTime(entry.occurred_at)}
                    </td>

                    <td className="whitespace-nowrap px-4 py-2 text-slate-700">
                      {entry.entry_type}
                    </td>

                    <td className="whitespace-nowrap px-4 py-2 text-right font-medium text-slate-900">
                      {formatAmount(entry.amount)}
                    </td>

                    <td className="px-4 py-2 text-slate-700">
                      {entry.memo ?? "-"}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </section>

      <section className="rounded-lg border border-slate-200 bg-white shadow-sm">
        <div className="border-b border-slate-200 px-4 py-3">
          <h2 className="text-base font-semibold text-slate-900">
            Billing Audit
          </h2>

          <p className="mt-1 text-sm text-slate-500">
            Operation events and audit logs related to this folio.
          </p>
        </div>

        {isAuditLoading && (
          <div className="p-4 text-sm text-slate-500">
            Loading billing audit...
          </div>
        )}

        {isAuditError && (
          <div className="p-4 text-sm text-red-700">
            Failed to load billing audit.
          </div>
        )}

        {!isAuditLoading && !isAuditError && audits.length === 0 && (
          <div className="p-4 text-sm text-slate-500">
            No billing audit events.
          </div>
        )}

        {!isAuditLoading && !isAuditError && audits.length > 0 && (
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-slate-200 text-sm">
              <thead className="bg-slate-50">
                <tr>
                  <th className="px-4 py-2 text-left font-medium text-slate-500">
                    Occurred At
                  </th>

                  <th className="px-4 py-2 text-left font-medium text-slate-500">
                    Operation
                  </th>

                  <th className="px-4 py-2 text-right font-medium text-slate-500">
                    Amount
                  </th>

                  <th className="px-4 py-2 text-left font-medium text-slate-500">
                    Method
                  </th>

                  <th className="px-4 py-2 text-left font-medium text-slate-500">
                    Reference
                  </th>

                  <th className="px-4 py-2 text-left font-medium text-slate-500">
                    Invoice
                  </th>

                  <th className="px-4 py-2 text-left font-medium text-slate-500">
                    Billing Account
                  </th>

                  <th className="px-4 py-2 text-left font-medium text-slate-500">
                    Actor
                  </th>

                  <th className="px-4 py-2 text-left font-medium text-slate-500">
                    Source
                  </th>

                  <th className="px-4 py-2 text-left font-medium text-slate-500">
                    Reason
                  </th>

                  <th className="px-4 py-2 text-left font-medium text-slate-500">
                    Trace
                  </th>
                </tr>
              </thead>

              <tbody className="divide-y divide-slate-200 bg-white">
                {audits.map((audit) => {
                  const displayAmount =
                    audit.amount ?? audit.issued_amount;

                  const displayInvoice =
                    audit.invoice_number ?? audit.invoice_id;

                  const displayBillingAccount =
                    audit.billing_account_name ??
                    audit.billing_account_id;

                  return (
                    <tr key={`${audit.operation_id}-${audit.aggregate_id}`}>
                      <td className="whitespace-nowrap px-4 py-2 text-slate-700">
                        {formatDateTime(audit.occurred_at)}
                      </td>

                      <td className="whitespace-nowrap px-4 py-2 font-medium text-slate-900">
                        {audit.operation_type}
                      </td>

                      <td className="whitespace-nowrap px-4 py-2 text-right font-medium text-slate-900">
                        {displayAmount
                          ? formatAmount(displayAmount)
                          : "-"}
                      </td>

                      <td className="whitespace-nowrap px-4 py-2 text-slate-700">
                        {audit.payment_method ?? "-"}
                      </td>

                      <td className="whitespace-nowrap px-4 py-2 text-slate-700">
                        {audit.payment_reference ?? "-"}
                      </td>

                      <td className="whitespace-nowrap px-4 py-2 text-slate-700">
                        {displayInvoice ?? "-"}
                      </td>

                      <td className="whitespace-nowrap px-4 py-2 text-slate-700">
                        {displayBillingAccount ?? "-"}
                      </td>

                      <td className="whitespace-nowrap px-4 py-2 text-slate-700">
                        {audit.actor}
                        {audit.actor_id ? `:${audit.actor_id}` : ""}
                      </td>

                      <td className="whitespace-nowrap px-4 py-2 text-slate-700">
                        {audit.source}
                      </td>

                      <td className="whitespace-nowrap px-4 py-2 text-slate-700">
                        {audit.reason ?? "-"}
                      </td>

                      <td className="px-4 py-2 text-xs text-slate-600">
                        <div>
                          folio_entry: {audit.folio_entry_id ?? "-"}
                        </div>

                        <div>
                          payment: {audit.payment_id ?? "-"}
                        </div>

                        <div>
                          invoice: {audit.invoice_id ?? "-"}
                        </div>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        )}
      </section>
    </div>
  );
}