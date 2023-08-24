pub mod openai;

use tokio_stream::Stream;
use thiserror::Error;
use std::{borrow::Cow, pin::Pin};

#[derive(Debug, Error)]
pub enum Error {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("An error occured while serializing json response: {0}")]
    SerializeJSON(#[from] serde_json::Error),
    #[error("An error ocurred: {0}")]
    Boxed(#[from] Box<dyn std::error::Error>),
    #[error("An error ocurred: {0}")]
    Custom(Cow<'static, str>)
}

pub type ResultStream = Result<String, Error>;
pub type ResultRun = Result<Pin<Box<dyn Stream<Item = ResultStream>>>, Error>;