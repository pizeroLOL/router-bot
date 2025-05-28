use std::sync::Arc;

use serde::{Deserialize, Serialize, Serializer};

pub mod adapter;
pub mod action;
pub mod event;
pub mod ws;
pub mod processor;

pub trait Event: Serializer {
    fn is_async(self: Arc<Self>) -> bool;
    fn fn_name(self: Arc<Self>) -> Arc<str>;
}

pub trait Msg: Serialize {
    type Rsp;
    fn msg_type(&self) -> Arc<str>;
    fn is_async(&self) -> bool;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Ok,
    Async,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgRsp<T> {
    pub status: Status,
    pub retcode: u32,
    #[serde(skip_serializing_if = "Option::is_none")] // Keep data field optional in serialization
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")] // Add echo field for actions
    pub echo: Option<serde_json::Value>,
}
