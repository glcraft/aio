mod parser;
mod renderer;

#[derive(Debug)]
enum StyleType {
    Bold,
    Italic,
    BoldItalic,
    Code,
}

pub use parser::Parser;
pub use renderer::Renderer;