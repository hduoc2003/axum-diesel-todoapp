use crate::api::v1::utils::errors::ErrorResponse;

pub trait DbBase {
    async fn add_user(&mut self, username: &str, pass: &str) -> Result<(), ErrorResponse>;

}
