use chrono::Utc;

use uuid::Uuid;

use crate::{
    db::connection::Db,
    domain::entity::{
        folio_entry::{FolioEntry, FolioEntryType},
        night_audit_room_charge_posting::NightAuditRoomChargePosting,
    },
    error::app_error::{infra, AppResult},
    repository::sqlite::operational::{
        billing::folio_entry_repository::SqliteFolioEntryRepository,
        business_date::night_audit_room_charge_posting_repository::SqliteNightAuditRoomChargePostingRepository,
    },
    usecase::business_date::{
        night_audit_worklist::{collect_room_charge_candidates, NightAuditRoomChargeCandidate},
        validation::ensure_active_business_date_closing,
    },
};

pub struct PostRoomChargesResult {
    pub posted_room_charges: Vec<NightAuditRoomChargeCandidate>,
}

pub async fn execute(db: &Db) -> AppResult<PostRoomChargesResult> {
    let mut tx = db.begin_tx().await;

    let result = async {
        let business_date = ensure_active_business_date_closing(&mut tx).await?;
        let candidates = collect_room_charge_candidates(&mut tx, &business_date).await?;

        for candidate in &candidates {
            let now = Utc::now();
            let folio_entry = FolioEntry {
                id: Uuid::new_v4(),
                folio_id: candidate.folio_id,
                entry_type: FolioEntryType::RoomCharge,
                amount: candidate.amount,
                occurred_at: now,
                memo: Some(format!(
                    "Night audit room charge for {}",
                    candidate.service_date
                )),
            };

            SqliteFolioEntryRepository::save(&mut tx, &folio_entry).await?;

            let posting = NightAuditRoomChargePosting {
                id: Uuid::new_v4(),
                business_date_id: business_date.id,
                business_date: business_date.business_date,
                reservation_id: candidate.reservation_id,
                folio_id: candidate.folio_id,
                service_date: candidate.service_date,
                amount: candidate.amount,
                folio_entry_id: folio_entry.id,
                posted_at: now,
            };

            SqliteNightAuditRoomChargePostingRepository::save(&mut tx, &posting).await?;
        }

        Ok(PostRoomChargesResult {
            posted_room_charges: candidates,
        })
    }
    .await;

    match result {
        Ok(result) => {
            tx.commit().await.map_err(infra)?;

            Ok(result)
        }

        Err(e) => {
            let _ = tx.rollback().await;

            Err(e)
        }
    }
}
