use uuid::Uuid;

use crate::{
    api::dto::input::room::CreateRoomInput,
    db::connection::Db,
    domain::entity::room::Room,
    error::app_error::{infra, AppResult},
    projection::{
        invalidation::{
            projection_invalidation::{ProjectionInvalidation, ProjectionRefreshTarget},
            projection_scope::ProjectionScope,
        },
        orchestrator::refresh_projection_chain::refresh_projection_chain,
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::operational::room_repository::SqliteRoomRepository,
};

pub async fn execute(db: &Db, input: CreateRoomInput) -> AppResult<Room> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let room_id = Uuid::new_v4();

        let room = if input.is_physical {
            Room::new(
                room_id,
                input.room_no,
                input.room_class,
                input.capacity.unwrap_or(1),
                input.area_sqm,
            )
        } else {
            Room::new_virtual(room_id, input.room_no, input.room_class)
        };

        SqliteRoomRepository::save(&mut tx, &room).await?;

        refresh_projection_chain(
            &mut tx,
            ProjectionInvalidation::new(
                ProjectionNode::InventoryAggregate,
                ProjectionScope::Inventory,
                ProjectionRefreshTarget::Global,
            ),
        )
        .await?;

        refresh_projection_chain(
            &mut tx,
            ProjectionInvalidation::new(
                ProjectionNode::DailyRoomClassKpiAggregate,
                ProjectionScope::Inventory,
                ProjectionRefreshTarget::Global,
            ),
        )
        .await?;

        refresh_projection_chain(
            &mut tx,
            ProjectionInvalidation::new(
                ProjectionNode::DailyHotelKpiAggregate,
                ProjectionScope::Inventory,
                ProjectionRefreshTarget::Global,
            ),
        )
        .await?;

        refresh_projection_chain(
            &mut tx,
            ProjectionInvalidation::new(
                ProjectionNode::MonthlyRoomClassKpiAggregate,
                ProjectionScope::Inventory,
                ProjectionRefreshTarget::Global,
            ),
        )
        .await?;

        refresh_projection_chain(
            &mut tx,
            ProjectionInvalidation::new(
                ProjectionNode::MonthlyHotelKpiAggregate,
                ProjectionScope::Inventory,
                ProjectionRefreshTarget::Global,
            ),
        )
        .await?;

        Ok(room)
    }
    .await;

    match result {
        Ok(room) => {
            tx.commit().await.map_err(infra)?;

            Ok(room)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}
