use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VoidInvoiceRequest {
    pub reason: Option<String>,
}