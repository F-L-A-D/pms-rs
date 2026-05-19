use crate::{
    api::dto::input::housekeeping::HousekeepingRoomDailyStateInput,
    db::connection::Db,
    domain::semantic::{
        operation_context::OperationContext,
        room_daily_state::{RoomDailyHousekeepingStatus, RoomDailyState},
    },
    error::app_error::{domain, infra, not_found, AppResult},
    projection::{
        invalidation::{
            projection_invalidation::{ProjectionInvalidation, ProjectionRefreshTarget},
            projection_scope::ProjectionScope,
        },
        orchestrator::refresh_projection_chain::refresh_projection_chain,
        topology::projection_node::ProjectionNode,
    },
    repository::sqlite::operational::room_daily_state_repository::SqliteRoomDailyStateRepository,
    repository::sqlite::operational::room_repository::SqliteRoomRepository,
    usecase::audit::command::record_audit_log::{record_audit_log, RecordAuditLogInput},
};

pub async fn execute(db: &Db, input: HousekeepingRoomDailyStateInput) -> AppResult<RoomDailyState> {
    let mut tx = db.begin_tx().await;

    let result = async {
        SqliteRoomRepository::find_by_id(&mut tx, input.room_id)
            .await?
            .ok_or(not_found("room not found"))?;

        let mut state = SqliteRoomDailyStateRepository::find_by_room_and_service_date(
            &mut tx,
            input.room_id,
            input.service_date,
        )
        .await?
        .ok_or_else(|| domain("room daily state is not cleaned"))?;

        if state.housekeeping_status != RoomDailyHousekeepingStatus::Cleaned {
            return Err(domain("room daily state is not cleaned"));
        }

        state.inspect();

        SqliteRoomDailyStateRepository::save(&mut tx, &state).await?;

        refresh_projection_chain(
            &mut tx,
            ProjectionInvalidation::new(
                ProjectionNode::HousekeepingDailyWorkloadAggregate,
                ProjectionScope::Inventory,
                ProjectionRefreshTarget::RoomDate {
                    date: input.service_date.to_string(),
                },
            ),
        )
        .await?;

        record_audit_log(
            &mut tx,
            &OperationContext::api_system(),
            RecordAuditLogInput {
                aggregate_type: "room_daily_state".to_string(),
                aggregate_id: input.room_id,
                action: "housekeeping.inspect".to_string(),
                before_json: None,
                after_json: serde_json::json!({
                    "room_id": state.room_id,
                    "service_date": state.service_date,
                    "housekeeping_status": state.housekeeping_status,
                })
                .to_string(),
                changed_fields_json: serde_json::json!([
                    {"field_name": "housekeeping_status", "before_value": "cleaned", "after_value": state.housekeeping_status.to_snake()}
                ])
                .to_string(),
                reason: None,
            },
        )
        .await?;

        Ok(state)
    }
    .await;

    match result {
        Ok(state) => {
            tx.commit().await.map_err(infra)?;

            Ok(state)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}
