use chrono::Utc;

use uuid::Uuid;

use crate::{
    api::dto::input::guest::AddGuestPreferenceInput,
    db::connection::Db,
    domain::semantic::guest_preference::GuestPreference,
    error::app_error::{infra, not_found, validation, AppResult},
    repository::sqlite::operational::guest::{
        guest_preference_repository::SqliteGuestPreferenceRepository,
        guest_repository::SqliteGuestRepository,
    },
};

pub async fn execute(db: &Db, input: AddGuestPreferenceInput) -> AppResult<GuestPreference> {
    let mut tx = db.begin_tx().await;

    let result = async {
        SqliteGuestRepository::find_by_id(&mut tx, input.guest_id)
            .await?
            .ok_or_else(|| not_found("guest not found"))?;

        if input.value.trim().is_empty() {
            return Err(validation("guest preference value is required"));
        }

        let preference = GuestPreference {
            id: Uuid::new_v4(),
            guest_id: input.guest_id,
            preference_type: input.preference_type,
            value: input.value,
            notes: input.notes,
            created_at: Utc::now(),
        };

        SqliteGuestPreferenceRepository::save(&mut tx, &preference).await?;

        Ok(preference)
    }
    .await;

    match result {
        Ok(preference) => {
            tx.commit().await.map_err(infra)?;

            Ok(preference)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}
