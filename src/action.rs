use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequest {
    pub action: String,
    pub params: Value,
}
