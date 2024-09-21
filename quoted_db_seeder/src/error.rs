use quoted_db::error::DBError;
use sea_orm::DbErr;
use thiserror::Error;

use crate::parse_csv::CsvError;

#[derive(Error, Debug)]
pub enum SeedError {
    #[error("Database error")]
    DBError(#[from] DBError),

    #[error("Database error")]
    DbErr(#[from] DbErr),

    #[error("CSV Error")]
    Csv(#[from] CsvError),

    #[error("Auth error")]
    Auth(#[from] std::io::Error),

    #[error("Google error")]
    GoogleError(#[from] google_sheets4::Error),

    #[error("Invalid Sheet Data: `{0}`")]
    InvalidSheetData(String),
}
