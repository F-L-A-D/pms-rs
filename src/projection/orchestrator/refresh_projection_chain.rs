use sqlx::{
    Sqlite,
    Transaction,
};

use crate::{
    error::app_error::
        AppResult,

    projection::{
        service::{
            hotel_inventory_projection_service::
                refresh_hotel_inventory_projection,
        },

        topology::{
            projection_node::
                ProjectionNode,

            projection_ordering::
                downstream_of,
        },
    },
};

async fn refresh_single_projection(
    tx: &mut Transaction<'_, Sqlite>,
    node: ProjectionNode,
    date: &str,
) -> AppResult<()>
{
    match node {

        ProjectionNode::HotelInventory => {

            refresh_hotel_inventory_projection(
                tx,
                date,
            )
            .await?;
        }

        _ => {
            // no-op
        }
    }

    Ok(())
}

pub async fn refresh_projection_chain(
    tx: &mut Transaction<'_, Sqlite>,
    start: ProjectionNode,
    date: &str,
) -> AppResult<()>
{
    let downstream =
        downstream_of(start);

    for node in downstream {

        refresh_single_projection(
            tx,
            node,
            date,
        )
        .await?;
    }

    Ok(())
}