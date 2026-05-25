use crate::{
    api::dto::{
        input::room::GetRoomInput,
        response::room::{
            RoomAssignmentVisibilityResponse, RoomDetailResponse,
        },
    },
    db::connection::Db,
    error::app_error::{infra, not_found, AppResult},
    repository::sqlite::operational::{
        reservation::reservation_repository::SqliteReservationRepository,
        room::{
            room_daily_state_repository::SqliteRoomDailyStateRepository,
            room_repository::SqliteRoomRepository,
        },
    },
    usecase::room::assignment_visibility::build_assignment_visibility,
};

pub async fn get_room(
    db: &Db,
    input: GetRoomInput,
) -> AppResult<RoomDetailResponse> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let room = SqliteRoomRepository::find_by_id(&mut tx, input.room_id)
            .await?
            .ok_or_else(|| not_found("room not found"))?;

        let daily_state =
            if let Some(service_date) = input.service_date {
                SqliteRoomDailyStateRepository::find_by_room_and_service_date(
                    &mut tx,
                    input.room_id,
                    service_date,
                )
                .await?
            } else {
                None
            };

        let assignment =
            if let Some(service_date) = input.service_date {
                let reservations =
                    SqliteReservationRepository::list_room_assignments_by_room_and_service_date(
                        &mut tx,
                        input.room_id,
                        service_date,
                    )
                    .await?;

                build_assignment_visibility(reservations)
            } else {
                RoomAssignmentVisibilityResponse::unassigned()
            };

        Ok(RoomDetailResponse::from_parts(
            room,
            daily_state,
            assignment,
        ))
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