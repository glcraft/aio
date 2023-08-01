use super::renderer::Renderer;

#[derive(Debug)]
pub enum ParseError<RendererErr> {
    RendererError(RendererErr),
}

impl<E> From<E> for ParseError<E> {
    fn from(err: E) -> Self {
        ParseError::RendererError(err)
    }
}

pub struct Parser<R: Renderer> {
    pub renderer: R,
}
impl<R: Renderer> Parser<R> {
    // type Result<T> = Result<T, ParseError<R::Error>>;
    pub fn new(renderer: R) -> Self {
        Self {
            renderer
        }
    }
    pub fn push(&mut self, text: &str) -> Result<(), ParseError<R::Error>> {
        for c in text.chars() {
            self.analyse_char(c)?;
        }
        Ok(self.renderer.flush()?)
    }
    
}