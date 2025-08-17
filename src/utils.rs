use ntex::http::{
    Uri,
    uri::{InvalidUriParts, PathAndQuery},
};

use crate::adapters::http::HttpEntry;

pub(crate) trait PipeOps {
    #[inline]
    fn pipe<F, O>(self, f: F) -> O
    where
        Self: Sized,
        F: FnOnce(Self) -> O,
    {
        f(self)
    }
    #[inline]
    fn mut_pipe(mut self, f: impl FnOnce(&mut Self)) -> Self
    where
        Self: std::marker::Sized,
    {
        f(&mut self);
        self
    }
}

pub trait WithPathAndQuery {
    fn with_path_and_query(
        self,
        path_and_query: PathAndQuery,
    ) -> Result<HttpEntry, InvalidUriParts>;
}

impl WithPathAndQuery for Uri {
    fn with_path_and_query(
        self,
        path_and_query: PathAndQuery,
    ) -> Result<HttpEntry, InvalidUriParts> {
        self.into_parts()
            .mut_pipe(|parts| {
                parts.path_and_query = Some(path_and_query);
            })
            .pipe(Uri::from_parts)?
            .pipe(HttpEntry)
            .pipe(Ok)
    }
}
