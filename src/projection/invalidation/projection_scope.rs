#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub enum ProjectionScope {
    Global,
    Inventory,
    Guest,
    Billing,
    Timeline,
}