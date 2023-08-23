pub mod openai;
use thiserror::Error;
use std::{borrow::Cow, pin::Pin};

#[derive(Debug, Error)]
pub enum Error {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("An error ocurred: {0}")]
    Boxed(#[from] Box<dyn std::error::Error>),
    #[error("An error ocurred: {0}")]
    Custom(Cow<'static, str>)
}

pub type BoxedError = Box<dyn std::error::Error>;
pub type ResultStream = Result<String, Error>;
pub type ResultRun = Result<Pin<Box<dyn tokio_stream::Stream<Item = ResultStream>>>, Error>;