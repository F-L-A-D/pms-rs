use sqlx::{
    Sqlite,
    Transaction,
};

use uuid::Uuid;

use crate::{
    error::app_error::AppResult,

    projection::{
        execution::execution_trace::push_trace, 
        
        signal::{
            materializer::
                materialize_guest_activity_signal::
                    materialize_guest_activity_signal,
            
            model::guest_activity_signal::GuestActivitySignal,

        }, topology::projection_node::ProjectionNode
    }, repository::sqlite::projection::save::save_guest_activity::save_guest_activity,
};

pub async fn refresh_guest_activity_signal(
    tx: &mut Transaction<'_, Sqlite>,
    guest_id: Uuid,
) -> AppResult<GuestActivitySignal>
{

    push_trace(
        ProjectionNode::GuestActivitySignal,
    );

    let signal =
        materialize_guest_activity_signal(    
            tx,
            guest_id,
        )
        .await?;

    save_guest_activity(
        tx,
        &signal,
    )
    .await?;

    Ok(signal)

}