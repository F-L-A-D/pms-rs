#[derive(Debug)]
pub enum AppError {
    Validation(String),
    NotFound(String),
    Conflict(String),
    Infrastructure(String),
}

pub type AppResult<T> = Result<T, AppError>;