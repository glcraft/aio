pub mod split_bytes;
pub mod flatten_stream;

pub use split_bytes::{SplitBytes, SplitBytesFactory};
pub use flatten_stream::FlattenTrait;

pub mod box_error {
    use std::error::Error;
    pub trait BoxError {
        fn box_error(self) -> Box<dyn std::error::Error>;
    }
    impl<E: Error + 'static> BoxError for E 
    {
        fn box_error(self) -> Box<dyn std::error::Error> {
            Box::new(self)
        }
    }
    pub trait IntoBoxedError<T> {
        fn into_boxed_error(self) -> Result<T, Box<dyn Error>>;
    }
    impl <T, E: Error + 'static> IntoBoxedError<T> for Result<T, E> {
        fn into_boxed_error(self) -> Result<T, Box<dyn Error>> {
            self.map_err(|e| -> Box<dyn Error> { Box::new(e) })
        }
    }
}
