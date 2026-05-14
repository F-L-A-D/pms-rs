use sqlx::{
    Sqlite,
    Transaction,
};

use crate::{
    error::app_error::AppResult,

    projection::{
        
        access::
                list_guest_ids::list_guest_ids,
                
        signal::
            materializer::materialize_guest_activity_signal::
                materialize_guest_activity_signal,

        execution::execution_trace::push_trace,

        topology::projection_node::ProjectionNode,
    },

    repository::sqlite::projection::save::
        save_guest_activity::
            save_guest_activity,
};

pub async fn rebuild_guest_activity_signal(
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()> {

    push_trace(
        ProjectionNode::GuestActivitySignal,
    );

    let guest_ids =
        list_guest_ids(tx)
            .await?;

    for guest_id in guest_ids {

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
    }

    Ok(())
}