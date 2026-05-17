#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectionNode {
    DailyRoomClassKpiAggregate,
    DailyHotelKpiAggregate,
    GuestAggregate,
    GuestActivitySignal,
    HousekeepingDailyWorkloadAggregate,
    InventoryAggregate,
    MonthlyHotelKpiAggregate,
    MonthlyRoomClassKpiAggregate,
}
