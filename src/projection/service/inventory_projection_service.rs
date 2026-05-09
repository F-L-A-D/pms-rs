use sqlx::{Sqlite, Transaction};

use crate::domain::reservation::Reservation;

use crate::error::app_error::{
    AppError,
    AppResult,
};

use crate::repository::sqlite::projection::
    inventory_projection_repository::SqliteInventoryProjectionRepository;

use crate::projection::orchestrator::
    refresh_projection_chain::refresh_projection_chain;

use crate::projection::topology::
    projection_node::ProjectionNode;
    
pub async fn apply_reservation_projection(
    tx: &mut Transaction<'_, Sqlite>,
    reservation: &Reservation,
) -> AppResult<()> {

    for d in reservation.nights() {

        SqliteInventoryProjectionRepository::add(
            tx,
            &d.to_string(),
            &reservation.room_class,
            1,
        )
        .await
        .map_err(
            AppError::Infrastructure
        )?;

        refresh_projection_chain(
            tx,
            ProjectionNode::Inventory,
            &d.to_string(),
        )
        .await?;
    }

    Ok(())
}

pub async fn remove_reservation_projection(
    tx: &mut Transaction<'_, Sqlite>,
    reservation: &Reservation,
) -> AppResult<()> {

    for d in reservation.nights() {

        SqliteInventoryProjectionRepository::add(
            tx,
            &d.to_string(),
            &reservation.room_class,
            -1,
        )
        .await
        .map_err(
            AppError::Infrastructure
        )?;
    
        refresh_projection_chain(
            tx,
            ProjectionNode::Inventory,
            &d.to_string(),
        )
        .await?;
    }

    Ok(())
}

pub async fn transition_reservation_projection(
    tx: &mut Transaction<'_, Sqlite>,
    old_reservation: &Reservation,
    new_reservation: &Reservation,
) -> AppResult<()> {

    remove_reservation_projection(
        tx,
        old_reservation,
    )
    .await?;

    apply_reservation_projection(
        tx,
        new_reservation,
    )
    .await?;

    Ok(())
}