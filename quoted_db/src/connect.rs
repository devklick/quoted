use std::{env, str::FromStr};

use sea_orm::{ConnectOptions, DatabaseConnection};

use crate::error::DBError;

enum ConnectionEnvVar {
    Host,
    Port,
    User,
    Password,
    Url,
}

impl ToString for ConnectionEnvVar {
    fn to_string(&self) -> String {
        match self {
            ConnectionEnvVar::Host => "DATABASE_HOST".to_owned(),
            ConnectionEnvVar::Port => "DATABASE_PORT".to_owned(),
            ConnectionEnvVar::User => "DATABASE_USER".to_owned(),
            ConnectionEnvVar::Password => "DATABASE_PASSWORD".to_owned(),
            ConnectionEnvVar::Url => "DATABASE_URL".to_owned(),
        }
    }
}

struct ConnectionParams {
    protocol: String,
    host: String,
    port: u16,
    user: String,
    password: String,
    database: String,
}

pub async fn get_default_connection() -> Result<DatabaseConnection, DBError> {
    let connection_string = get_connection_string()?;

    // May want to configure these further.
    // See https://www.sea-ql.org/SeaORM/docs/install-and-config/connection/#connect-options
    let mut connection_opts = ConnectOptions::new(connection_string);
    connection_opts.sqlx_logging(false);

    return Ok(sea_orm::Database::connect(connection_opts).await?);
}

/// Returns the `DATABASE_URL` env var if it's set, otherwise
/// builds a connection string from the destructed env vars,
/// DATABASE_HOST, DATABASE_PORT, DATABASE_USER, DATABASE_PASSWORD
///
/// # Errors
/// When `DATABASE_URL` is not set and one or more of the destructured
/// env vars are not set.
///
/// When one of the destructured env vars is of the wrong type.
///
/// This function will return an error if .
fn get_connection_string() -> Result<String, DBError> {
    if let Some(url) = get_optional_env_var(ConnectionEnvVar::Url)? {
        return Ok(url);
    }

    let params = get_connection_params()?;
    Ok(format!(
        "{}://{}:{}@{}:{}/{}",
        &params.protocol, params.user, params.password, params.host, params.port, params.database
    ))
}

fn get_connection_params() -> Result<ConnectionParams, DBError> {
    return Ok(ConnectionParams {
        protocol: "postgres".to_owned(),
        database: "quoted".to_owned(),
        host: get_required_env_var::<String>(ConnectionEnvVar::Host)?,
        password: get_required_env_var::<String>(ConnectionEnvVar::Password)?,
        port: get_required_env_var::<u16>(ConnectionEnvVar::Port)?,
        user: get_required_env_var::<String>(ConnectionEnvVar::User)?,
    });
}

fn get_required_env_var<T>(env_var: ConnectionEnvVar) -> Result<T, DBError>
where
    T: FromStr,
{
    let raw = env::var(env_var.to_string())
        .or(Err(DBError::ConnectionParamRequired(env_var.to_string())))?;

    let parsed = raw
        .parse::<T>()
        .or(Err(DBError::ConnectionParamInvalid(env_var.to_string())))?;

    return Ok(parsed);
}

fn get_optional_env_var<T>(env_var: ConnectionEnvVar) -> Result<Option<T>, DBError>
where
    T: FromStr,
{
    Ok(match env::var(env_var.to_string()) {
        Err(_) => None,
        Ok(val) => Some(
            val.parse::<T>()
                .or(Err(DBError::ConnectionParamInvalid(env_var.to_string())))?,
        ),
    })
}
