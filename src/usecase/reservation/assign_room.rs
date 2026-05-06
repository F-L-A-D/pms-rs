use crate::db::connection::Db;
use crate::repository::sqlite::repository::SqliteReservationRepository;
use crate::repository::sqlite::room_repository::SqliteRoomRepository;
use crate::domain::room::OccupancyStatus;

pub async fn assign_room(
    db: &Db,
    reservation_id: &str,
    room_id: &str,
) -> Result<(), String> {

    let mut tx = db.begin_tx().await;

    let result = async {

        let mut res = SqliteReservationRepository::find_by_id_tx(&mut tx, reservation_id)
            .await?
            .ok_or("reservation not found")?;

        let room = SqliteRoomRepository::find_by_id(&db.pool, room_id)
            .await?
            .ok_or("room not found")?;

        if res.room_id.is_some() {
            return Err("already assigned".into());
        }

        if room.occupancy_status != OccupancyStatus::Vacant {
            return Err("room not vacant".into());
        }

        res.room_id = Some(room_id.to_string());

        SqliteReservationRepository::save_tx(&mut tx, &res).await?;

        Ok(())
    }.await;

    match result {
        Ok(_) => {
            tx.commit().await.unwrap();
            Ok(())
        }
        Err(e) => {
            tx.rollback().await.unwrap();
            Err(e)
        }
    }
}