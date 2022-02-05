use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("required field missing: {0})")]
    Field(String),
    #[error("unknown error")]
    Unknown,
}

