use sqlx::{
    Sqlite,
    Transaction,
};

use chrono::Utc;

use uuid::Uuid;

use crate::error::app_error::AppResult;

use crate::projection::operational::
    reservation_search::
    ReservationSearchProjection;

use crate::usecase::reservation::
    get_reservation_search_view::
    get_reservation_search_view;

pub async fn materialize_reservation_search_projection(
    tx: &mut Transaction<'_, Sqlite>,
    reservation_id: Uuid,
) -> AppResult<
    ReservationSearchProjection,
> {

    let view =
        get_reservation_search_view(
            tx,
            reservation_id,
        )
        .await?;

    Ok(
        ReservationSearchProjection {
            reservation_id:
                view.reservation_id,

            external_id:
                view.external_id,

            primary_guest_name:
                view.primary_guest_name,

            participant_names:
                view.participant_names,

            check_in:
                view.check_in,

            check_out:
                view.check_out,

            room_class:
                view.room_class,

            room_id:
                view.room_id,

            reservation_status:
                view.reservation_status,

            stay_status:
                view.stay_status,

            projection_version: 1,

            updated_at: Utc::now(),
            
        }
    )
}