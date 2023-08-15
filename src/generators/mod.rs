pub mod openai;
use std::borrow::Cow;

pub use openai::run as openai_run;

pub enum Error {
    Reqwest(reqwest::Error),
    Boxed(Box<dyn std::error::Error>),
    Custom(Cow<'static, str>)
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}
impl From<Box<dyn std::error::Error>> for Error {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        Self::Boxed(e)
    }
}

pub type BoxedError = Box<dyn std::error::Error>;
pub type ResultStream = Result<String, Error>;
pub type ResultRun = Result<Box<dyn tokio_stream::Stream<Item = ResultStream>>, Error>;