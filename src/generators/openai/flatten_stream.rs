use tokio_stream::Stream;

pub trait FlattenTrait: Stream
{
    // type Item: Stream;
    fn flatten_stream(self) -> Flatten<Self>
    where 
        Self: Sized,
    {
        Flatten {
            stream: self,
            current: None
        }
    }
}

pub struct Flatten<I> 
where
    I: Stream,
{
    stream: I,
    current: Option<<I as Stream>::Item>
}

// impl<I> FlattenTrait for I 
// where
//     I: Stream,
//     <I as Stream>::Item: Stream + Unpin,
//     <<I as Stream>::Item as Stream>::Item: Stream + Unpin,
// {
//     fn flatten_stream(self) -> Flatten<Self> {
//         Flatten {
//             stream: self,
//             current: None
//         }
//     }
// }

// impl<I> Stream for Flatten<I>
// where
//     I: Stream,
//     <I as Stream>::Item: Stream + Unpin,
//     <<I as Stream>::Item as Stream>::Item: Stream + Unpin,
// {
//     type Item = <<I as Stream>::Item as Stream>::Item;
//     fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
//         use std::task::Poll::*;
//         use std::pin::pin;
//         match self.current {
//             Some(ref mut item) => {
//                 let item = pin!(item).poll_next(cx);
//                 match item {
//                     Ready(None) => {
//                         self.current = None;
//                         self.poll_next(cx)
//                     }
//                     v => v,
//                 }
//             }
//             None => {
//                 let item = pin!(self.stream).poll_next(cx);
//                 match item {
//                     Ready(Some(item)) => {
//                         self.current = Some(item);
//                         self.poll_next(cx)
//                     }
//                     Ready(None) => Ready(None),
//                     Pending => Pending,
//                 }
//             }
//         }
//     }
// }


impl<St: ?Sized> FlattenTrait for St where St: Stream {}
// impl<St: ?Sized, It> FlattenTrait for St 
// where 
//     It: Iterator,
//     St: Stream<Item = Result<It, super::Error>>,
// {}


impl<I> Stream for Flatten<I>
where
    I: Stream,
    <I as Stream>::Item: Iterator,
{
    type Item = <<I as Stream>::Item as Iterator>::Item;
    fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<<Self as Stream>::Item>> {
        use std::task::Poll::*;
        use std::pin::pin;
        match self.current {
            Some(ref mut item) => {
                let item = item.next();
                match item {
                    None => {
                        self.current = None;
                        self.poll_next(cx)
                    }
                    v => Ready(v),
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