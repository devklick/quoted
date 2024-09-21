use google_sheets4::{
    hyper::{client::HttpConnector, Client},
    hyper_rustls::HttpsConnector,
    oauth2::{self, authenticator::Authenticator, ServiceAccountKey},
};

use crate::error::SeedError;

pub async fn get_key(key_path: &String) -> Result<oauth2::ServiceAccountKey, SeedError> {
    oauth2::read_service_account_key(&key_path)
        .await
        .or_else(|e| Err(SeedError::Auth(e)))
}

pub async fn get_authenticator(
    key: ServiceAccountKey,
    client: &Client<HttpsConnector<HttpConnector>>,
) -> Result<Authenticator<HttpsConnector<HttpConnector>>, SeedError> {
    oauth2::ServiceAccountAuthenticator::with_client(key, client.clone())
        .build()
        .await
        .or_else(|e| Err(SeedError::Auth(e)))
}
