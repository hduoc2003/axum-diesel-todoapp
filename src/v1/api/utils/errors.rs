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
