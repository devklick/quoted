mod connect;
pub mod error;

use error::DBError;
use sea_orm::DatabaseConnection;

pub async fn get_default_connection() -> Result<DatabaseConnection, DBError> {
    connect::get_default_connection().await
}

pub fn enable_query_logging() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing_subscriber::filter::LevelFilter::DEBUG)
        .with_test_writer()
        .try_init();
}
