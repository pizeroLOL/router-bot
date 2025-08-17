use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum ForwardMethod {
    Query(String),
    Header(String),
    None,
}

#[derive(Debug, Clone)]
pub enum BackwardMethod {
    Header(String),
    None,
}

impl From<BackwardMethod> for ForwardMethod {
    fn from(value: BackwardMethod) -> Self {
        match value {
            BackwardMethod::Header(x) => Self::Header(x),
            BackwardMethod::None => Self::None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ForwardToBackwardError {
    UnsupportQuery { token: String },
}

impl Display for ForwardToBackwardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self:?}"))
    }
}

impl Error for ForwardToBackwardError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl TryInto<BackwardMethod> for ForwardMethod {
    type Error = ForwardToBackwardError;

    fn try_into(self) -> Result<BackwardMethod, Self::Error> {
        match self {
            ForwardMethod::Query(e) => Err(ForwardToBackwardError::UnsupportQuery { token: e }),
            ForwardMethod::Header(o) => Ok(BackwardMethod::Header(o)),
            ForwardMethod::None => Ok(BackwardMethod::None),
        }
    }
}
