use std::sync::atomic::{AtomicU64, Ordering};

use chrono::{Duration, NaiveDate, Utc};

use uuid::Uuid;

use pms_rs::{
    api::dto::request::{
        guest::{
            CreateGuestRequest,
            UpdateGuestRequest,
        },
        reservation::{
            CreateReservationRequest,
            ReservationDailyDetailRequest,
            ReservationPackageBreakdownRequest,
            ReservationParticipantRequest,
        },
        room::CreateRoomRequest,
    },
    domain::{
        entity::guest::Gender,
        semantic::{
            reservation_booking::ReservationBookingChannel,
            reservation_guest_relation::ReservationGuestRelationType,
        },
    },
};

static COUNTER: AtomicU64 = AtomicU64::new(1);

pub struct GuestBuilder {
    last_name: String,
    first_name: String,
    phone: Option<String>,
    email: Option<String>,
    nationality: Option<String>,
    birth_date: Option<String>,
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
    birth_date: Option<String>,
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
    check_in: String,
    check_out: String,
    room_class: String,
    booking_channel: Option<ReservationBookingChannel>,
    source_channel: Option<String>,
    plan_code: Option<String>,
    participants: Vec<ReservationParticipantRequest>,
    daily_details: Vec<ReservationDailyDetailRequest>,
    package_breakdowns: Vec<ReservationPackageBreakdownRequest>,
}

pub struct RoomBuilder {
    room_no: String,
    room_class: String,
    area_sqm: rust_decimal::Decimal,
    capacity: Option<u32>,
    is_physical: bool,
}

impl GuestBuilder {
    pub fn new() -> Self {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);

        Self {
            last_name: format!("last_name_{id}"),
            first_name: format!("first_name_{id}"),
            phone: None,
            email: None,
            nationality: None,
            birth_date: None,
            gender: None,
            membership_code: None,
            marketing_opt_in: false,
        }
    }

    #[allow(dead_code)]
    pub fn with_email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());

        self
    }

    pub fn build(self) -> CreateGuestRequest {
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
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);

        Self {
            last_name: format!("updated_last_name_{id}"),
            first_name: format!("updated_first_name_{id}"),
            phone: None,
            email: Some(format!("updated_{id}@test.com")),
            nationality: None,
            birth_date: None,
            gender: None,
            membership_code: None,
            marketing_opt_in: true,
        }
    }

    pub fn build(self) -> UpdateGuestRequest {
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
    pub fn new(guest_id: Uuid) -> Self {
        Self {
            guest_id,
            relation_type: ReservationGuestRelationType::Primary,
        }
    }

    pub fn with_relation_type(
        mut self,
        value: ReservationGuestRelationType,
    ) -> Self {
        self.relation_type = value;
        self
    }

    pub fn build(self) -> ReservationParticipantRequest {
        ReservationParticipantRequest {
            guest_id: self.guest_id.to_string(),
            relation_type: self.relation_type,
        }
    }
}

impl ReservationBuilder {
    pub fn new() -> Self {
        let today = Utc::now().date_naive();

        Self {
            external_id: None,
            check_in: today.to_string(),
            check_out: (today + Duration::days(1)).to_string(),
            room_class: "standard".to_string(),
            booking_channel: Some(ReservationBookingChannel::Direct),
            source_channel: None,
            plan_code: None,
            participants: vec![],
            daily_details: vec![
                ReservationDailyDetailRequest {
                    service_date: today.to_string(),
                    room_class: "standard".to_string(),
                    plan_code: None,
                    adult_count: 2,
                    child_count: 0,
                    sleep_sharing_child_count: 0,
                    sleep_sharing_children: vec![],
                    package_breakdowns: vec![],
                },
            ],
            package_breakdowns: vec![],
        }
    }

    pub fn with_participant(
        mut self,
        participant: ReservationParticipantRequest,
    ) -> Self {
        self.participants.push(participant);
        self
    }

    pub fn with_check_in(mut self, value: NaiveDate) -> Self {
        self.check_in = value.to_string();
        self.rebuild_daily_details();

        self
    }

    pub fn with_check_out(mut self, value: NaiveDate) -> Self {
        self.check_out = value.to_string();
        self.rebuild_daily_details();

        self
    }

    pub fn with_external_id(
        mut self,
        value: impl Into<String>,
    ) -> Self {
        self.external_id =
            Some(value.into());

        self
    }

    pub fn build(self) -> CreateReservationRequest {
        CreateReservationRequest {
            external_id: self.external_id,
            check_in: self.check_in,
            check_out: self.check_out,
            room_class: self.room_class,
            booking_channel: self.booking_channel,
            source_channel: self.source_channel,
            plan_code: self.plan_code,
            participants: self.participants,
            daily_details: self.daily_details,
            package_breakdowns: self.package_breakdowns,
        }
    }

    fn rebuild_daily_details(&mut self) {
        let Ok(check_in) = chrono::NaiveDate::parse_from_str(&self.check_in, "%Y-%m-%d") else {
            return;
        };

        let Ok(check_out) = chrono::NaiveDate::parse_from_str(&self.check_out, "%Y-%m-%d") else {
            return;
        };

        self.daily_details = vec![];

        let mut service_date = check_in;

        while service_date < check_out {
            self.daily_details.push(ReservationDailyDetailRequest {
                service_date: service_date.to_string(),
                room_class: self.room_class.clone(),
                plan_code: self.plan_code.clone(),
                adult_count: 2,
                child_count: 0,
                sleep_sharing_child_count: 0,
                sleep_sharing_children: vec![],
                package_breakdowns: vec![],
            });

            service_date += chrono::Duration::days(1);
        }
    }
}

impl RoomBuilder {
    pub fn new() -> Self {
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);

        Self {
            room_no: format!("room_{id}"),
            room_class: "standard".to_string(),
            area_sqm: rust_decimal::Decimal::new(2000, 2),
            capacity: Some(2),
            is_physical: true,
        }
    }

    pub fn build(self) -> CreateRoomRequest {
        CreateRoomRequest {
            room_no: self.room_no,
            room_class: self.room_class,
            area_sqm: self.area_sqm,
            capacity: self.capacity,
            is_physical: self.is_physical,
        }
    }
}