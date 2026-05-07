pub mod router;
pub mod state;
pub mod error;
pub mod handlers {
    pub mod billing;
    pub mod health;
    pub mod reservation;
    pub mod room;
    pub mod stay;
    pub mod guest;
    pub mod housekeeping;
}
pub mod dto{
    pub mod billing;
    pub mod reservation;
    pub mod room;
    pub mod stay;
    pub mod guest;
    pub mod error;
}