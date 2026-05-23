use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    api::dto::{
        input::room::ListRoomsInput,
        response::room::RoomListItemResponse,
    },
    db::connection::Db,
    domain::semantic::room_daily_state::RoomDailyState,
    error::app_error::{infra, AppResult},
    repository::sqlite::operational::room::{
        room_daily_state_repository::SqliteRoomDailyStateRepository,
        room_repository::SqliteRoomRepository,
    },
};

pub async fn list_rooms(
    db: &Db,
    input: ListRoomsInput,
) -> AppResult<Vec<RoomListItemResponse>> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let rooms = if input.include_inactive {
            SqliteRoomRepository::find_all(&mut tx).await?
        } else {
            SqliteRoomRepository::find_active(&mut tx).await?
        };

        let daily_states_by_room_id =
            if let Some(service_date) = input.service_date {
                let daily_states =
                    SqliteRoomDailyStateRepository::list_by_service_date(
                        &mut tx,
                        service_date,
                    )
                    .await?;

                daily_states
                    .into_iter()
                    .map(|state| (state.room_id, state))
                    .collect::<HashMap<Uuid, RoomDailyState>>()
            } else {
                HashMap::new()
            };

        let response =
            rooms
                .into_iter()
                .map(|room| {
                    let daily_state =
                        daily_states_by_room_id.get(&room.id).cloned();

                    RoomListItemResponse::from_parts(room, daily_state)
                })
                .collect();

        Ok(response)
    }
    .await;

    match result {
        Ok(response) => {
            tx.commit().await.map_err(infra)?;

            Ok(response)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}