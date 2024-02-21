use super::base::DbBase;

#[derive(Clone)]
pub struct InMemoryDb;

impl DbBase for InMemoryDb {
    async fn add_user(&mut self, username: &str, pass: &str) -> Result<(), crate::api::v1::utils::errors::ErrorResponse> {
        todo!()
    }
}
