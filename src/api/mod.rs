pub mod error;
pub mod router;
pub mod state;
pub mod handlers {
    pub mod billing;
    pub mod guest;
    pub mod health;
    pub mod housekeeping;
    pub mod reservation;
    pub mod room;
    pub mod stay;
    pub mod timeline;
}
pub mod dto;
