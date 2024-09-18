use serde::Serialize;
use vercel_runtime::{Body, Error, Response};

pub type ApiResult<T> = Result<SuccessResult<T>, ErrorResult>;

#[derive(Debug, Serialize)]
pub struct SuccessResult<T>
where
    T: Serialize,
{
    pub status_code: u16,
    pub body: T,
}

pub trait VercelResponse {
    fn vercel(self) -> Result<Response<Body>, Error>;
}

impl<T> SuccessResult<T>
where
    T: Serialize,
{
    pub fn ok(body: T) -> SuccessResult<T> {
        SuccessResult {
            status_code: 200,
            body,
        }
    }
    pub fn created(body: T) -> SuccessResult<T> {
        SuccessResult {
            status_code: 201,
            body,
        }
    }
}

impl<T> VercelResponse for SuccessResult<T>
where
    T: Serialize,
{
    fn vercel(self) -> Result<Response<Body>, Error> {
        Ok(build_http_response(self.status_code, self.body))
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub message: Option<String>,
    pub key: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorResult {
    pub status_code: u16,
    pub body: ErrorDetail,
}

impl ErrorResult {
    pub fn not_found() -> ErrorResult {
        ErrorResult {
            body: ErrorDetail {
                key: None,
                message: None,
            },
            status_code: 404,
        }
    }
    pub fn bad_request(message: &str) -> ErrorResult {
        ErrorResult {
            body: ErrorDetail {
                key: None,
                message: Some(message.to_owned()),
            },
            status_code: 400,
        }
    }
    pub fn server_error(message: &str) -> ErrorResult {
        ErrorResult {
            body: ErrorDetail {
                key: None,
                message: Some(message.to_owned()),
            },
            status_code: 400,
        }
    }
    pub fn with_key(mut self, key: &str) -> Self {
        self.body.key = Some(key.to_owned());
        self
    }
    pub fn with_message(mut self, message: &str) -> Self {
        self.body.message = Some(message.to_owned());
        self
    }
}

impl VercelResponse for ErrorResult {
    fn vercel(self) -> Result<Response<Body>, vercel_runtime::Error> {
        Ok(build_http_response(self.status_code, self.body))
    }
}

fn build_http_response<T>(status_code: u16, body: T) -> Response<Body>
where
    T: Serialize,
{
    let json = serde_json::to_string(&body).unwrap();

    Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(Body::Text(json))
        .unwrap()
}
