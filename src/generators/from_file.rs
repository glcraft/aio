use crate::args;
use super::{ResultRun, ResultStream, Error};

pub async fn run(_: crate::config::Config, _args: args::FromFileArgs, input: &str) -> ResultRun {
    use tokio_stream::StreamExt;
    let file = tokio::fs::File::open(&input).await.map_err(|e| Error::Custom(std::borrow::Cow::Owned(e.to_string())))?;

    let stream = tokio_util::io::ReaderStream::new(file).map(|r| -> ResultStream {
        let bytes = r.map_err(|e| Error::Custom(std::borrow::Cow::Owned(e.to_string())))?;
        String::from_utf8(bytes.as_ref().to_vec()).map_err(|e| Error::Custom(std::borrow::Cow::Owned(e.to_string())))
    });
    Ok(Box::pin(stream))
}