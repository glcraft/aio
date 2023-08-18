use tokio_stream::Stream;

trait FlattenTrait: Stream + Sized 
where 
    <Self as Stream>::Item: Stream + Unpin,
    <<Self as Stream>::Item as Stream>::Item: Stream + Unpin,
{
    // type Item: Stream;
    fn flatten_stream(self) -> Flatten<Self>;
}

struct Flatten<I> 
where
    I: Stream,
    <I as Stream>::Item: Stream + Unpin,
    <<I as Stream>::Item as Stream>::Item: Stream + Unpin,
{
    stream: I,
    current: Option<<I as Stream>::Item>
}

impl<I> FlattenTrait for I 
where
    I: Stream,
    <I as Stream>::Item: Stream + Unpin,
    <<I as Stream>::Item as Stream>::Item: Stream + Unpin,
{
    fn flatten_stream(self) -> Flatten<Self> {
        Flatten {
            stream: self,
            current: None
        }
    }
}

impl<I> Stream for Flatten<I>
where
    I: Stream,
    <I as Stream>::Item: Stream + Unpin,
    <<I as Stream>::Item as Stream>::Item: Stream + Unpin,
{
    type Item = <<I as Stream>::Item as Stream>::Item;
    fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        use std::task::Poll::*;
        use std::pin::pin;
        match self.current {
            Some(ref mut item) => {
                let item = pin!(item).poll_next(cx);
                match item {
                    Ready(None) => {
                        self.current = None;
                        self.poll_next(cx)
                    }
                    v => v,
                }
            }
            None => {
                let item = pin!(self.stream).poll_next(cx);
                match item {
                    Ready(Some(item)) => {
                        self.current = Some(item);
                        self.poll_next(cx)
                    }
                    Ready(None) => Ready(None),
                    Pending => Pending,
                }
            }
        }
    }
}
