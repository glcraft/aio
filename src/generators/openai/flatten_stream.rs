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

#[pin_project::pin_project]
pub struct Flatten<St> 
where
    St: Stream,
{
    #[pin]
    stream: St,
    current: Option<<St as Stream>::Item>
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

/// Implementation of Flatten for Result of Iterator
impl<It, E, St> Stream for Flatten<St>
where
    St: Stream<Item = Result<It, E>>,
    It: Iterator
{
    type Item = Result<<It as Iterator>::Item, E>;
    fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<<Self as Stream>::Item>> {
        use std::task::Poll::*;
        use std::pin::pin;
        
        match self.as_mut().project().current {
            Some(Ok(ref mut item)) => {
                let item = item.next();
                match item {
                    None => {
                        *self.as_mut().project().current = None;
                        self.poll_next(cx)
                    }
                    Some(v) => Ready(Some(Ok(v))),
                }
            }
            e @ Some(Err(_)) => {
                let mut emv = None;
                std::mem::swap(&mut emv, e);
                let Some(Err(emv)) = emv else { unreachable!() };
                Ready(Some(Err(emv)))
            },
            None => {
                let item = self.as_mut().project().stream.poll_next(cx);
                match item {
                    Ready(Some(item)) => {
                        *self.as_mut().project().current = Some(item);
                        self.poll_next(cx)
                    }
                    Ready(None) => Ready(None),
                    Pending => Pending,
                }
            }
        }
    }
}


// Implementation of Flatten for Iterator

// impl<St, It> Stream for Flatten<St>
// where
//     St: Stream<Item = It>,
//     It: Iterator
// {
//     type Item = <It as Iterator>::Item;
//     fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<<Self as Stream>::Item>> {
//         use std::task::Poll::*;
//         use std::pin::pin;
//         match self.current {
//             Some(ref mut item) => {
//                 let item = item.next();
//                 match item {
//                     None => {
//                         self.current = None;
//                         self.poll_next(cx)
//                     }
//                     v => Ready(v),
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