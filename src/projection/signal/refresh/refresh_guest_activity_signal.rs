use sqlx::{
    Sqlite,
    Transaction,
};

use crate::{
    error::app_error::AppResult,

    projection::{
        invalidation::
            projection_invalidation::
                ProjectionRefreshTarget,
        execution::execution_trace::push_trace,

        topology::projection_node::ProjectionNode,
    },
};

pub async fn refresh_guest_activity_signal(
    _tx: &mut Transaction<'_, Sqlite>,
    _target: &ProjectionRefreshTarget,
) -> AppResult<()>
{

    push_trace(
        ProjectionNode::GuestActivitySignal,
    );
    
    Ok(())
}