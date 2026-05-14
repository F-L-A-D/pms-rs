#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum ProjectionNode {
    GuestAggregate,
    GuestActivitySignal,
}