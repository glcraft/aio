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
    renderer: R,
    mode_func: fn(&mut Self, char) -> Result<(), ParseError<R::Error>>,
}
impl<R: Renderer> Parser<R> {
    // type Result<T> = Result<T, ParseError<R::Error>>;
    pub fn new(renderer: R) -> Self {
        Self {
            renderer,
            mode_func: Self::analyse_text,
        }
    }
    pub fn push(&mut self, text: &str) -> Result<(), ParseError<R::Error>> {
        for c in text.chars() {
            (self.mode_func)(self, c)?;
        }
        Ok(self.renderer.flush()?)
    }
    pub fn analyse_text(&mut self, c: char) -> Result<(), ParseError<R::Error>> {
        self.mode_func = Self::analyse_text;
        Ok(())
    }
    pub fn end_of_document(&mut self) -> Result<(), ParseError<R::Error>> {
        todo!("End of document");
    }
}