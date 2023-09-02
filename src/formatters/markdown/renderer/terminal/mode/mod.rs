mod header;
mod code;

pub use header::Header;
pub use code::Code;
use super::InlineStyles;


pub enum Mode {
    Text(InlineStyles),
    Code(Code),
    Header(Header),
}

impl Default for Mode {
    fn default() -> Self {
        Self::Text(InlineStyles::default())
    }
}