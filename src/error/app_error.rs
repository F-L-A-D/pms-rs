#[derive(Debug)]
pub enum AppError {
    Validation(String),
    Domain(String),
    NotFound(String),
    Conflict(String),
    Infrastructure(String),
}

pub type AppResult<T> = Result<T, AppError>;

pub fn validation<E>(error: E) -> AppError
where
    E: ToString,
{
    AppError::Validation(error.to_string())
}

pub fn domain<E>(error: E) -> AppError
where
    E: ToString,
{
    AppError::Domain(error.to_string())
}

pub fn not_found<E>(error: E) -> AppError
where
    E: ToString,
{
    AppError::NotFound(error.to_string())
}

pub fn conflict<E>(error: E) -> AppError
where
    E: ToString,
{
    AppError::Conflict(error.to_string())
}

pub fn infra<E>(error: E) -> AppError
where
    E: ToString,
{
    AppError::Infrastructure(error.to_string())
}
