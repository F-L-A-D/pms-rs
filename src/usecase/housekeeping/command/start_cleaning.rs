use crate::{
    api::dto::input::housekeeping::HousekeepingRoomDailyStateInput,
    db::connection::Db,
    domain::semantic::room_daily_state::{RoomDailyHousekeepingStatus, RoomDailyState},
    error::app_error::{domain, infra, not_found, AppResult},
    repository::sqlite::operational::room_daily_state_repository::SqliteRoomDailyStateRepository,
    repository::sqlite::operational::room_repository::SqliteRoomRepository,
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
        .ok_or_else(|| domain("room daily state is not dirty"))?;

        if state.housekeeping_status != RoomDailyHousekeepingStatus::Dirty {
            return Err(domain("room daily state is not dirty"));
        }

        state.start_cleaning();

        SqliteRoomDailyStateRepository::save(&mut tx, &state).await?;

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
