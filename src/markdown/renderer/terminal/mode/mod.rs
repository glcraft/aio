mod header;

pub use header::Header;
use super::InlineStyles;

pub enum Mode {
    Text(InlineStyles),
    Code {
        index: usize,
        is_line_begin: bool,
        
        language: Option<String>
    },
    Header(Header),
}

impl Default for Mode {
    fn default() -> Self {
        Self::Text(InlineStyles::default())
    }
}