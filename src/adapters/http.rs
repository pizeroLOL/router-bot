use std::str::FromStr;

use ntex::http::{
    Client, Uri,
    client::{
        ClientRequest, SendClientRequest,
        error::{JsonPayloadError, SendRequestError},
    },
    uri::{InvalidUri, InvalidUriParts, PathAndQuery},
};
use serde::{Deserialize, Serialize};

use crate::{PipeOps, utils::WithPathAndQuery};

use super::auth::ForwardMethod;

#[derive(Debug, Clone)]
pub struct HttpEntry(pub Uri);

#[derive(Debug, Clone)]
pub struct HttpExecUnit<S: Serialize> {
    pub client: Client,
    pub url: HttpEntry,
    pub auth: ForwardMethod,
    pub data: S,
}

pub enum ExecError {
    EncodePathAndQuery(InvalidUri),
    EncodeUrl(InvalidUriParts),
    EncodeQueryData(serde_urlencoded::ser::Error),
    Send(SendRequestError),
    Decode(JsonPayloadError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Urlencoded,
    Json,
    Query,
}

fn add_bearer_header(auth: &ForwardMethod, client: ClientRequest) -> ClientRequest {
    match auth {
        ForwardMethod::Header(token) => client.bearer_auth(token),
        _ => client,
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum CustomResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> From<CustomResult<T, E>> for Result<T, E> {
    fn from(result: CustomResult<T, E>) -> Self {
        match result {
            CustomResult::Ok(value) => Ok(value),
            CustomResult::Err(error) => Err(error),
        }
    }
}

impl<T, E> From<Result<T, E>> for CustomResult<T, E> {
    fn from(value: Result<T, E>) -> CustomResult<T, E> {
        match value {
            Ok(o) => Self::Ok(o),
            Err(e) => Self::Err(e),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Response<O, E> {
    pub status: String,
    pub retcode: i32,
    pub data: Option<CustomResult<O, E>>,
}

impl<D: Serialize> HttpExecUnit<D> {
    async fn raw_exec<O, E>(
        self,
        method: impl Fn(Client, Uri) -> ClientRequest,
        sender: impl Fn(ClientRequest, D) -> SendClientRequest,
    ) -> Result<Response<O, E>, ExecError>
    where
        O: for<'de> Deserialize<'de>,
        E: for<'de> Deserialize<'de>,
    {
        self.client
            .pipe(|client| method(client, self.url.0))
            .pipe(|client| add_bearer_header(&self.auth, client))
            .pipe(|req| sender(req, self.data))
            .await
            .map_err(ExecError::Send)?
            .json::<Response<O, E>>()
            .await
            .map_err(ExecError::Decode)
    }

    pub async fn exec_query<O, E>(self) -> Result<Response<O, E>, ExecError>
    where
        D: CqReq,
        O: for<'de> Deserialize<'de>,
        E: for<'de> Deserialize<'de>,
    {
        self.raw_exec(|client, uri| client.get(uri), |req, _| req.send())
            .await
    }

    pub async fn exec_form<O, E>(self) -> Result<Response<O, E>, ExecError>
    where
        D: CqReq,
        O: for<'de> Deserialize<'de>,
        E: for<'de> Deserialize<'de>,
    {
        self.raw_exec(
            |client, uri| client.post(uri),
            |req, data| req.send_form(&data),
        )
        .await
    }

    pub async fn exec_json<O, E>(self) -> Result<Response<O, E>, ExecError>
    where
        D: JsonReq,
        O: for<'de> Deserialize<'de>,
        E: for<'de> Deserialize<'de>,
    {
        self.raw_exec(
            |client, uri| client.post(uri),
            |req, data| req.send_json(&data),
        )
        .await
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Default,
    Async,
    RateLimited,
}

impl Mode {
    pub fn to_suffix(self) -> &'static str {
        match self {
            Mode::Default => "",
            Mode::Async => "_async",
            Mode::RateLimited => "_rate_limited",
        }
    }
}

pub trait Entry: Serialize {
    type Output: for<'de> Deserialize<'de>;
    const ENTRY: &'static str;
}

trait ReplacePath: WithPathAndQuery {
    fn replace_path(self, path: &str) -> Result<HttpEntry, ExecError>
    where
        Self: Sized,
    {
        PathAndQuery::from_str(&path)
            .map_err(ExecError::EncodePathAndQuery)?
            .pipe(|path| self.with_path_and_query(path))
            .map_err(ExecError::EncodeUrl)
    }
}

impl ReplacePath for Uri {}

pub trait CqReq: Entry {
    fn send_query<E>(
        self,
        client: Client,
        base: &Uri,
        mode: Mode,
        auth: ForwardMethod,
    ) -> impl Future<Output = Result<Response<Self::Output, E>, ExecError>>
    where
        E: for<'de> Deserialize<'de>,
        Self: Sized,
    {
        async move {
            let data = serde_urlencoded::to_string(&self).map_err(ExecError::EncodeQueryData)?;
            match &auth {
                ForwardMethod::Query(t) => "&access_token=".to_owned() + t,
                _ => "".into(),
            }
            .pipe(|auth| format!("/{}{}?{}{}", Self::ENTRY, mode.to_suffix(), data, auth))
            .pipe(|x| base.clone().replace_path(&x))?
            .pipe(|url| HttpExecUnit {
                client,
                url,
                auth,
                data: self,
            })
            .exec_query()
            .await
        }
    }
    fn send_form<E>(
        self,
        client: Client,
        base: &Uri,
        mode: Mode,
        auth: ForwardMethod,
    ) -> impl Future<Output = Result<Response<Self::Output, E>, ExecError>>
    where
        E: for<'de> Deserialize<'de>,
        Self: Sized,
    {
        async move {
            match &auth {
                ForwardMethod::Query(t) => "?access_token=".to_owned() + t,
                _ => "".into(),
            }
            .pipe(|auth| format!("/{}{}{}", Self::ENTRY, mode.to_suffix(), auth))
            .pipe(|x| base.clone().replace_path(&x))?
            .pipe(|url| HttpExecUnit {
                client,
                url,
                auth,
                data: self,
            })
            .exec_form()
            .await
        }
    }
}

pub trait JsonReq: Entry {
    fn send_json<E>(
        self,
        client: Client,
        base: &Uri,
        mode: Mode,
        auth: ForwardMethod,
    ) -> impl Future<Output = Result<Response<Self::Output, E>, ExecError>>
    where
        E: for<'de> Deserialize<'de>,
        Self: Sized,
    {
        async move {
            match &auth {
                ForwardMethod::Query(t) => "?access_token=".to_owned() + t,
                _ => "".into(),
            }
            .pipe(|auth| format!("/{}{}{}", Self::ENTRY, mode.to_suffix(), auth))
            .pipe(|x| base.clone().replace_path(&x))?
            .pipe(|url| HttpExecUnit {
                client,
                url,
                auth,
                data: self,
            })
            .exec_json()
            .await
        }
    }
}
