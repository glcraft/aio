use super::renderer::Renderer;

pub struct Parser<R: Renderer> {
    pub renderer: R,
}
impl<R: Renderer> Parser<R> {
    pub fn new(renderer: R) -> Self {
        Self {
            renderer
        }
    }
    
}