use std::sync::Arc;

use serde::{Deserialize, Serialize, Serializer};

pub mod adapter;

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
pub enum MsgRsp<T> {
    Ok { retcode: u32, data: Option<T> },
    Async { retcode: u32 },
    Failed { retcode: u32 },
}
