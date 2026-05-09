pub mod router;
pub mod state;
pub mod error;
pub mod handlers {
    pub mod health;
    pub mod billing;
    pub mod reservation;
    pub mod room;
    pub mod stay;
    pub mod guest;
    pub mod housekeeping;
    pub mod timeline;
    pub mod guest_summary;
}
pub mod dto{
    pub mod billing;
    pub mod reservation;
    pub mod room;
    pub mod stay;
    pub mod guest;
    pub mod error;
    pub mod timeline;
    pub mod guest_summary;
}
