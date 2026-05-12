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
    },
};

pub async fn refresh_hotel_inventory_projection(
    _tx: &mut Transaction<'_, Sqlite>,
    _target: &ProjectionRefreshTarget,
) -> AppResult<()>
{
    Ok(())
}

pub async fn rebuild_hotel_inventory_projection(
    _tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()>
{
    Ok(())
}