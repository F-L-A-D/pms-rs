use async_trait::async_trait;

use sqlx::{
    Sqlite,
    Transaction,
};

use crate::{
    error::app_error::
        AppResult,

    projection::{
        invalidation::
            projection_invalidation::
                ProjectionRefreshTarget,

        topology::
            projection_node::
                ProjectionNode,

        execution::{
            handlers::{
                hotel_inventory::{
                    refresh_hotel_inventory_projection,
                    rebuild_hotel_inventory_projection,
                },
            },

            projection_convergence_executor::
                ProjectionConvergenceExecutor,
        },

        rebuild::{
            guest_summary_rebuild::
                rebuild_guest_summary_projection,

            inventory_projection_rebuild::
                rebuild_inventory_projection,

            reservation_search_rebuild::
                rebuild_reservation_search_projection,
        },
    },
};

pub struct ProjectionExecutionRegistry;

#[async_trait]
impl ProjectionConvergenceExecutor
    for ProjectionExecutionRegistry
{
    async fn execute(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        node: ProjectionNode,
        target: &ProjectionRefreshTarget,
    ) -> AppResult<()>
    {
        match node {

            ProjectionNode::HotelInventory => {

                refresh_hotel_inventory_projection(
                    tx,
                    target,
                )
                .await?;
            }

            ProjectionNode::GuestSummary => {}

            ProjectionNode::ReservationSearch => {}

            ProjectionNode::Inventory => {}
        }

        Ok(())
    }

    async fn execute_rebuild(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        node: ProjectionNode,
    ) -> AppResult<()>
    {
        match node {

            ProjectionNode::GuestSummary => {

                rebuild_guest_summary_projection(
                    tx,
                )
                .await?;
            }

            ProjectionNode::ReservationSearch => {

                rebuild_reservation_search_projection(
                    tx,
                )
                .await?;
            }

            ProjectionNode::Inventory => {

                rebuild_inventory_projection(
                    tx,
                )
                .await?;
            }

            ProjectionNode::HotelInventory => {

                rebuild_hotel_inventory_projection(
                    tx,
                )
                .await?;
            }
        }

        Ok(())
    }
}