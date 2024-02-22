use axum::{extract::Request, http::{header, StatusCode}, middleware::Next, response::Response};

use crate::api::v1::utils::{errors::ErrorResponse, hashing::verify_token};

pub async fn authentication(mut req: Request, next: Next) -> Result<Response, ErrorResponse> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_string())
            } else {
                None
            }
        });
    let token = token.ok_or_else(|| {
        ErrorResponse {
            status: StatusCode::UNAUTHORIZED,
            msg: "Bearer token not found".to_string(),
        }
    })?;
    // println!("{}", token);
    let session = match verify_token(token) {
        Ok(session) => session,
        Err(_e) => {
            // println!("{}", e.to_string());
            return Err(ErrorResponse {
                status: StatusCode::UNAUTHORIZED,
                msg: "Invalid token".to_string()
            })
        },
    };
    // println!("{:?}", session);
    req.extensions_mut().insert(session.user_id);
    Ok(next.run(req).await)
}
