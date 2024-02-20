use crate::v1::api::types::env::ENV;

pub fn get_env(key: ENV) -> String {
    std::env::var(key.as_ref()).expect(&format!("{} must be set", key.as_ref()))
}