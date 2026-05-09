use sqlx::{
    Sqlite,
    Transaction,
};

use crate::{
    error::app_error::
        AppResult,

    projection::{
        rebuild::{
            guest_summary_rebuild::
                rebuild_guest_summary_projection,

            inventory_projection_rebuild::
                rebuild_inventory_projection,

            reservation_search_rebuild::
                rebuild_reservation_search_projection,

            hotel_inventory_projection_rebuild::
                rebuild_hotel_inventory_projection,
        },

        topology::{
            projection_node::
                ProjectionNode,

            projection_ordering::
                rebuild_order,
        },
    },
};

async fn rebuild_single_projection(
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

pub async fn rebuild_projection_chain(
    tx: &mut Transaction<'_, Sqlite>,
    start: ProjectionNode,
) -> AppResult<()>
{
    let ordered =
        rebuild_order(start);

    for node in ordered {

        rebuild_single_projection(
            tx,
            node,
        )
        .await?;
    }

    Ok(())
}