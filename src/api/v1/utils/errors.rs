use std::error::Error;

use axum::{http::StatusCode, response::IntoResponse};

pub struct ErrorResponse {
    pub status: StatusCode,
    pub msg: String
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.msg).into_response()
    }
}

pub struct UnauthorizedError;
impl IntoResponse for UnauthorizedError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::UNAUTHORIZED, "Unauthorized error").into_response()
    }
}

pub fn internal_error<E>(err: E) -> ErrorResponse
where
    E: Error,
{
    ErrorResponse {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        msg: err.to_string()
    }
}
