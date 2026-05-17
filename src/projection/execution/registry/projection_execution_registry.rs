use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::AppResult,
    projection::{
        execution::binding::{
            daily_hotel_kpi_aggregate_rebuild::execute_daily_hotel_kpi_aggregate_rebuild,
            daily_hotel_kpi_aggregate_refresh::execute_daily_hotel_kpi_aggregate_refresh,
            daily_room_class_kpi_aggregate_rebuild::execute_daily_room_class_kpi_aggregate_rebuild,
            daily_room_class_kpi_aggregate_refresh::execute_daily_room_class_kpi_aggregate_refresh,
            guest_activity_signal_rebuild::execute_guest_activity_signal_rebuild,
            guest_activity_signal_refresh::execute_guest_activity_signal_refresh,
            guest_aggregate_rebuild::execute_guest_aggregate_rebuild,
            guest_aggregate_refresh::execute_guest_aggregate_refresh,
            housekeeping_daily_workload_aggregate_rebuild::execute_housekeeping_daily_workload_aggregate_rebuild,
            housekeeping_daily_workload_aggregate_refresh::execute_housekeeping_daily_workload_aggregate_refresh,
            inventory_aggregate_rebuild::execute_inventory_aggregate_rebuild,
            inventory_aggregate_refresh::execute_inventory_aggregate_refresh,
            monthly_hotel_kpi_aggregate_rebuild::execute_monthly_hotel_kpi_aggregate_rebuild,
            monthly_hotel_kpi_aggregate_refresh::execute_monthly_hotel_kpi_aggregate_refresh,
            monthly_room_class_kpi_aggregate_rebuild::execute_monthly_room_class_kpi_aggregate_rebuild,
            monthly_room_class_kpi_aggregate_refresh::execute_monthly_room_class_kpi_aggregate_refresh,
        },
        invalidation::projection_invalidation::ProjectionRefreshTarget,
        topology::projection_node::ProjectionNode,
    },
};

pub struct ProjectionExecutionRegistry;

impl ProjectionExecutionRegistry {
    pub async fn dispatch_refresh(
        tx: &mut Transaction<'_, Sqlite>,
        node: &ProjectionNode,
        target: &ProjectionRefreshTarget,
    ) -> AppResult<()> {
        match node {
            ProjectionNode::DailyRoomClassKpiAggregate => {
                execute_daily_room_class_kpi_aggregate_refresh(tx, target).await
            }
            ProjectionNode::DailyHotelKpiAggregate => {
                execute_daily_hotel_kpi_aggregate_refresh(tx, target).await
            }
            ProjectionNode::GuestAggregate => execute_guest_aggregate_refresh(tx, target).await,
            ProjectionNode::GuestActivitySignal => {
                execute_guest_activity_signal_refresh(tx, target).await
            }
            ProjectionNode::HousekeepingDailyWorkloadAggregate => {
                execute_housekeeping_daily_workload_aggregate_refresh(tx, target).await
            }
            ProjectionNode::InventoryAggregate => {
                execute_inventory_aggregate_refresh(tx, target).await
            }
            ProjectionNode::MonthlyHotelKpiAggregate => {
                execute_monthly_hotel_kpi_aggregate_refresh(tx, target).await
            }
            ProjectionNode::MonthlyRoomClassKpiAggregate => {
                execute_monthly_room_class_kpi_aggregate_refresh(tx, target).await
            }
        }
    }

    pub async fn dispatch_rebuild(
        tx: &mut Transaction<'_, Sqlite>,
        node: &ProjectionNode,
    ) -> AppResult<()> {
        match node {
            ProjectionNode::DailyRoomClassKpiAggregate => {
                execute_daily_room_class_kpi_aggregate_rebuild(tx).await?;
            }
            ProjectionNode::DailyHotelKpiAggregate => {
                execute_daily_hotel_kpi_aggregate_rebuild(tx).await?;
            }
            ProjectionNode::GuestAggregate => {
                execute_guest_aggregate_rebuild(tx).await?;
            }
            ProjectionNode::GuestActivitySignal => {
                execute_guest_activity_signal_rebuild(tx).await?;
            }
            ProjectionNode::HousekeepingDailyWorkloadAggregate => {
                execute_housekeeping_daily_workload_aggregate_rebuild(tx).await?;
            }
            ProjectionNode::InventoryAggregate => {
                execute_inventory_aggregate_rebuild(tx).await?;
            }
            ProjectionNode::MonthlyHotelKpiAggregate => {
                execute_monthly_hotel_kpi_aggregate_rebuild(tx).await?;
            }
            ProjectionNode::MonthlyRoomClassKpiAggregate => {
                execute_monthly_room_class_kpi_aggregate_rebuild(tx).await?;
            }
        }

        Ok(())
    }
}
