use sqlx::{
    Sqlite,
    Transaction,
};

use crate::{
    error::app_error::AppResult,

    projection::{
        execution::execution_trace::push_trace,

        topology::projection_node::ProjectionNode,
    },
};

pub async fn rebuild_guest_activity_signal(
    _tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()>
{    
    
    push_trace(
        ProjectionNode::GuestActivitySignal,
    );
    
    Ok(())
}