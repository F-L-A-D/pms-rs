#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettlementState {
    Open,
    PartiallySettled,
    Settled,
    Disputed,
    WrittenOff,
}
