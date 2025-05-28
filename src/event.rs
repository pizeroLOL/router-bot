use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEvent {
    pub time: i64,
    pub self_id: i64,
    pub post_type: String,
}
