use std::sync::Arc;

use ntex::http::{
    client::{
        Client,
        error::{JsonPayloadError, SendRequestError},
    },
};

use crate::{Msg, MsgRsp};

#[derive(Debug, Clone, Copy)]
pub enum HttpSendMode {
    Query,
    Urlencodeed,
    Json,
}

#[derive(Debug)]
pub enum SendHttpReqError {
    Json(JsonPayloadError),
    Query(serde_urlencoded::ser::Error),
    Payload(SendRequestError),
}

#[derive(Debug, Clone)]
pub struct Http {
    client: Client,
    host: Arc<str>,
    send_mode: HttpSendMode,
}

impl Http {
    pub fn new(url: &str, send_mode: HttpSendMode, auth: Option<Arc<str>>) -> Self {
        Self {
            host: url.into(),
            client: if let Some(ref a) = auth {
                Client::build().bearer_auth(a).finish()
            } else {
                Client::default()
            },
            send_mode,
        }
    }

    pub async fn exec<T>(&self, msg: T) -> Result<MsgRsp<T::Rsp>, SendHttpReqError>
    where
        T: Msg,
        T::Rsp: for<'de> serde::Deserialize<'de>,
    {
        let path = format!("{}/{}", self.host, msg.msg_type());
        match self.send_mode {
            HttpSendMode::Query => self
                .client
                .get(path)
                .query(&msg)
                .map_err(|e| SendHttpReqError::Query(e))?
                .send()
                .await
                .map_err(|e| SendHttpReqError::Payload(e))?
                .json()
                .await
                .map_err(|e| SendHttpReqError::Json(e)),
            HttpSendMode::Urlencodeed => self
                .client
                .post(path)
                .send_form(&msg)
                .await
                .map_err(|e| SendHttpReqError::Payload(e))?
                .json()
                .await
                .map_err(|e| SendHttpReqError::Json(e)),
            HttpSendMode::Json => self
                .client
                .post(path)
                .send_json(&msg)
                .await
                .map_err(|e| SendHttpReqError::Payload(e))?
                .json()
                .await
                .map_err(|e| SendHttpReqError::Json(e)),
        }
    }
}
