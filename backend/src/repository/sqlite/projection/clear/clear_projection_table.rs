use sqlx::{Sqlite, Transaction};

use crate::{
    error::app_error::{infra, AppResult},
    projection::topology::projection_node::ProjectionNode,
};

fn projection_table_name(node: &ProjectionNode) -> &'static str {
    match node {
        ProjectionNode::ChangePattern => "change_patterns",

        ProjectionNode::ConfidenceProfile => "confidence_profiles",

        ProjectionNode::DailyRoomClassKpiAggregate => "daily_room_class_kpi_aggregates",

        ProjectionNode::DailyHotelKpiAggregate => "daily_hotel_kpi_aggregates",

        ProjectionNode::GuestAggregate => "guest_aggregates",

        ProjectionNode::GuestActivitySignal => "guest_activities",

        ProjectionNode::HousekeepingDailyWorkloadAggregate => {
            "housekeeping_daily_workload_aggregates"
        }

        ProjectionNode::InventoryAggregate => "inventory_aggregates",

        ProjectionNode::MonthlyHotelKpiAggregate => "monthly_hotel_kpi_aggregates",

        ProjectionNode::MonthlyRoomClassKpiAggregate => "monthly_room_class_kpi_aggregates",

        ProjectionNode::SemanticActivation => "semantic_activations",
    }
}

pub async fn clear_projection_table(
    tx: &mut Transaction<'_, Sqlite>,
    node: ProjectionNode,
) -> AppResult<()> {
    let table_name = projection_table_name(&node);

    let query = format!("DELETE FROM {}", table_name,);

    sqlx::query(&query)
        .execute(&mut **tx)
        .await
        .map_err(infra)?;

    Ok(())
}
