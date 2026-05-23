import { apiGet } from "./client";

export type Folio = {
  id: string;
  reservation_id: string;
  billing_account_id: string | null;
  status: string;
  created_at: string;
};

export type FolioEntry = {
  id: string;
  folio_id: string;
  entry_type: string;
  amount: string;
  occurred_at: string;
  memo: string | null;
};

export type FolioDetail = {
  folio: Folio;
  entries: FolioEntry[];
  total_charges: string;
  total_payments: string;
  balance: string;
};

export function getFolioDetail(
  folioId: string,
): Promise<FolioDetail> {
  return apiGet<FolioDetail>(
    `/folios/${folioId}`,
  );
}

export type BillingAudit = {
  operation_id: string;

  folio_id: string | null;
  folio_entry_id: string | null;
  payment_id: string | null;
  invoice_id: string | null;

  aggregate_type: string;
  aggregate_id: string;

  operation_type: string;
  actor: string;
  actor_id: string | null;
  source: string;
  
  action: string | null;
  reason: string | null;

  amount: string | null;
  payment_method: string | null;
  payment_reference: string | null;

  invoice_number: string | null;
  issued_amount: string | null;

  billing_account_id: string | null;
  billing_account_name: string | null;

  before_json: string | null;
  after_json: string;
  changed_fields_json: string;

  occurred_at: string;
};

export function getFolioAudit(
  folioId: string,
): Promise<BillingAudit[]> {
  return apiGet<BillingAudit[]>(
    `/folios/${folioId}/audit`,
  );
}