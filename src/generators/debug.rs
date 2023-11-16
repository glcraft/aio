use super::{ResultRun, Error};
use tokio::fs;
use tokio_stream::StreamExt;
use bytes::Bytes;
use crate::{
    args,
    config,
    utils::{
        self,
        FlattenTrait,
        box_error::*
    }
};

pub async fn run(_: config::Config, args: args::ProcessedArgs) -> ResultRun {
    use super::openai::ChatResponse;
    let input = args.input;
    let file_content = fs::read_to_string(input).await.into_boxed_error()?;
    let mut split_bytes_factory = utils::SplitBytesFactory::new(b"\n\n");
    let it = tokio_stream::iter(utils::SplitBytes::new(Bytes::from(file_content), b"\n---\n"));
    
    let it = it
        .map(move |seq| {
            split_bytes_factory.new_iter(seq)
        })
        .flatten_iter()
        .map(|v| {
            // let v = v?;
            let chat_resp = ChatResponse::from_bytes(v);
            match chat_resp {
                Ok(resp) => Ok(resp),
                Err(e) => Err(Error::SerializeJSON(e))
            }
        })
        .map_while(|resp| {
            match resp {
                Ok(msg @ (ChatResponse::Message { .. } | ChatResponse::Status{ .. })) => Some(Ok(msg.to_string())),
                Ok(ChatResponse::Done) => None,
                Err(e) => Some(Err(e)),
            }
        });
    Ok(Box::pin(it))
}