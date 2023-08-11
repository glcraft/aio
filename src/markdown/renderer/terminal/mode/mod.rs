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