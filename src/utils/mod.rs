pub mod split_bytes;
pub mod flatten_stream;

pub use split_bytes::{SplitBytes, SplitBytesFactory};
pub use flatten_stream::FlattenTrait;

macro_rules! hashmap {
    ($($name:ident => $value:expr),*) => {{
        let mut map = ::std::collections::HashMap::new();
        $(map.insert({stringify!($name)}.into(), {$value}.into());)*
        map
    }};
}

pub(crate) use hashmap;