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