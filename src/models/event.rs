use serde::{Deserialize, Serialize};

use super::basic_type::Sex;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sender {
    pub user_id: i64,
    pub nickname: String,
    pub sex: Sex,
    pub age: i32,
}
