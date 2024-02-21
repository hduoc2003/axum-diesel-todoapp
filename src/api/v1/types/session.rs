use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Session {
    pub user_id: i64,
    pub exp: usize
}
