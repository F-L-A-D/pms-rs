#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectionNode {
    ChangePattern,
    ConfidenceProfile,
    DailyRoomClassKpiAggregate,
    DailyHotelKpiAggregate,
    GuestAggregate,
    GuestActivitySignal,
    HousekeepingDailyWorkloadAggregate,
    InventoryAggregate,
    MonthlyHotelKpiAggregate,
    MonthlyRoomClassKpiAggregate,
    SemanticActivation,
}
