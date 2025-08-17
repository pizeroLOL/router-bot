use ntex::http::uri::{InvalidUri, InvalidUriParts};

#[derive(Debug)]
pub enum WithPathAndQueryError {
    IntoPart(InvalidUri),
    IntoUri(InvalidUriParts),
}
