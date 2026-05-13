use std::sync::atomic::{
    AtomicU64,
    Ordering,
};

use chrono::{
    Duration,
    NaiveDate,
    Utc,
};

use uuid::Uuid;

use pms_rs::{
    api::dto::{
        guest::{
            CreateGuestRequest,
            UpdateGuestRequest,
        },
        reservation::{
            CreateReservationRequest,
            ReservationParticipantInput,
        },
        room::CreateRoomRequest,
    },
    domain::{
        guest::Gender,

        reservation_guest_relation::
            ReservationGuestRelationType,
    },
};

static COUNTER: AtomicU64 =
    AtomicU64::new(1);

pub struct GuestBuilder {
    last_name: String,
    first_name: String,
    phone: Option<String>,
    email: Option<String>,
    nationality: Option<String>,
    birth_date: Option<NaiveDate>,
    gender: Option<Gender>,
    membership_code: Option<String>,
    marketing_opt_in: bool,
}

pub struct UpdateGuestBuilder {
    last_name: String,
    first_name: String,
    phone: Option<String>,
    email: Option<String>,
    nationality: Option<String>,
    birth_date: Option<NaiveDate>,
    gender: Option<Gender>,
    membership_code: Option<String>,
    marketing_opt_in: bool,
}

pub struct ReservationParticipantBuilder {
    guest_id: Uuid,
    relation_type: ReservationGuestRelationType,
}

pub struct ReservationBuilder {
    external_id: Option<String>,
    check_in: NaiveDate,
    check_out: NaiveDate,
    room_class: String,
    participants: Vec<ReservationParticipantInput>,
}

pub struct RoomBuilder {
    room_no: String,
    room_class: String,
}

impl GuestBuilder {

    pub fn new() -> Self {

        let id =
            COUNTER.fetch_add(
                1,
                Ordering::SeqCst,
            );

        Self {
            last_name:
                format!("last_name_{id}"),

            first_name:
                format!("first_name_{id}"),

            phone: None,
            email: None,
            nationality: None,
            birth_date: None,
            gender: None,
            membership_code: None,

            marketing_opt_in: false,
        }
    }

    pub fn with_email(
        mut self,
        value: impl Into<String>,
    ) -> Self {

        self.email = Some(value.into());

        self
    }

    pub fn build(
        self,
    ) -> CreateGuestRequest {

        CreateGuestRequest {
            last_name: self.last_name,
            first_name: self.first_name,
            phone: self.phone,
            email: self.email,
            nationality: self.nationality,
            birth_date: self.birth_date,
            gender: self.gender,
            membership_code: self.membership_code,
            marketing_opt_in: self.marketing_opt_in,
        }
    }
}

impl UpdateGuestBuilder {

    pub fn new() -> Self {

        let id =
            COUNTER.fetch_add(
                1,
                Ordering::SeqCst,
            );

        Self {
            last_name:
                format!("updated_last_name_{id}"),

            first_name:
                format!("updated_first_name_{id}"),

            phone: None,

            email: Some(
                format!("updated_{id}@test.com")
            ),

            nationality: None,

            birth_date: None,

            gender: None,

            membership_code: None,

            marketing_opt_in: true,
        }
    }

    pub fn build(
        self,
    ) -> UpdateGuestRequest {

        UpdateGuestRequest {
            last_name: self.last_name,
            first_name: self.first_name,
            phone: self.phone,
            email: self.email,
            nationality: self.nationality,
            birth_date: self.birth_date,
            gender: self.gender,
            membership_code: self.membership_code,
            marketing_opt_in: self.marketing_opt_in,
        }
    }
}


impl ReservationParticipantBuilder {

    pub fn new(
        guest_id: Uuid,
    ) -> Self {

        Self {
            guest_id,

            relation_type:
                ReservationGuestRelationType::Primary,
        }
    }

    pub fn with_relation_type(
        mut self,
        value: ReservationGuestRelationType,
    ) -> Self {

        self.relation_type = value;

        self
    }

    pub fn build(
        self,
    ) -> ReservationParticipantInput {

        ReservationParticipantInput {
            guest_id: self.guest_id,
            relation_type: self.relation_type,
        }
    }
}

impl ReservationBuilder {

    pub fn new() -> Self {

        let today =
            Utc::now()
                .date_naive();

        Self {
            external_id: None,

            check_in: today,

            check_out:
                today + Duration::days(1),

            room_class:
                "standard".to_string(),

            participants: vec![],
        }
    }

    pub fn with_participant(
        mut self,
        participant: ReservationParticipantInput,
    ) -> Self {

        self.participants
            .push(participant);

        self
    }

    pub fn with_check_in(
        mut self,
        value: NaiveDate,
    ) -> Self {
        
        self.check_in = value;

        self
    }

    pub fn with_check_out(
        mut self,
        value: NaiveDate,
    ) -> Self {

        self.check_out = value;

        self
    }

    pub fn build(
        self,
    ) -> CreateReservationRequest {

        CreateReservationRequest {
            external_id: self.external_id,
            check_in: self.check_in,
            check_out: self.check_out,
            room_class: self.room_class,
            participants: self.participants,
        }
    }
}

impl RoomBuilder {
    
    pub fn new() -> Self {

        let id =
            COUNTER.fetch_add(
                1, 
                Ordering::SeqCst,
            );
        
        Self {
            room_no:
                format!("room_{id}"),

            room_class:
                format!("standard").into(),
        }
    }

    pub fn build(
        self,
    ) -> CreateRoomRequest {

        CreateRoomRequest { 
            room_no: self.room_no, 
            room_class: self.room_class, 
        }
    }
}