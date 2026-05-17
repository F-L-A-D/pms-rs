use crate::db::connection::Db;

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
}
