#[derive(Debug)]
pub enum AppError {
    Validation(String),
    Domain(String),
    NotFound(String),
    Conflict(String),
    Infrastructure(String),
}

pub type AppResult<T> =
    Result<T, AppError>;

impl From<String> for AppError {

    fn from(
        value: String
    ) -> Self {

        AppError::Domain(value)
    }
}