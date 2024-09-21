use google_sheets4::{
    hyper::{self, client::HttpConnector},
    hyper_rustls::{self, HttpsConnector},
};

pub fn get() -> hyper::Client<HttpsConnector<HttpConnector>> {
    hyper::Client::builder().build(
        hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .unwrap()
            .https_only()
            .enable_http1()
            .enable_http2()
            .build(),
    )
}
