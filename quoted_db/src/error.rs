use sea_orm::DbErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Failed to connect to the database")]
    Connect(#[from] DbErr),
    #[error("Missing required connection parameter {0}")]
    ConnectionParamRequired(String),
    #[error("Invalid value for connection parameter {0}")]
    ConnectionParamInvalid(String),
    #[error("Missing dependency for insert operation")]
    MissingInsertDependency,
}
