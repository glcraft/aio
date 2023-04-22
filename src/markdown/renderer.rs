use super::StyleType;
pub trait Renderer {
    fn apply_style(&mut self, style: StyleType);
    fn print_text(&mut self, text: &str);
}