use std::env;

use quoted_db::enable_query_logging;

pub fn setup() {
    if let Ok(log) = env::var("QUERY_LOGGING") {
        if log.to_lowercase() == "true".to_owned() {
            enable_query_logging();
        }
    }
}
