use super::renderer::Renderer;
use super::token;

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
    current_text: String,
    current_token: String,
    previous_char: Option<char>,
    mode_func: fn(&mut Self, char) -> Result<(), ParseError<R::Error>>,
}
impl<R: Renderer> Parser<R> {
    // type Result<T> = Result<T, ParseError<R::Error>>;
    pub fn new(renderer: R) -> Self {
        Self {
            renderer,
            current_text: String::new(),
            current_token: String::with_capacity(3),
            previous_char: None,
            mode_func: Self::analyse_text,
        }
    }
    pub fn push(&mut self, text: &str) -> Result<(), ParseError<R::Error>> {
        for c in text.chars() {
            (self.mode_func)(self, c)?;
        }
        self.push_current_text()?;
        Ok(self.renderer.flush()?)
    }
    fn analyse_text(&mut self, c: char) -> Result<(), ParseError<R::Error>> {
        match c {
            '\n' => {
                self.apply_text_token(c)?;
                self.push_current_text()?;
                self.renderer.push_token(token::Token::Newline)?;
                self.previous_char = None;
            }
            c @ ('*' | '_' | '`') => {
                self.current_token.push(c);
            }
            _ => {
                self.apply_text_token(c)?;
                self.current_text.push(c);
                self.previous_char = Some(c);
            }
        }
        
        Ok(())
    }
    fn analyse_code_block(&mut self, c: char) -> Result<(), ParseError<R::Error>> {
        match c {
            '\n' => {
                self.apply_code_token(c)?;
                self.push_current_text()?;
                self.renderer.push_token(token::Token::Newline)?;
                self.previous_char = None;
            }
            '`' if self.previous_char.is_none() => self.current_token.push(c),
            _ => {
                self.apply_code_token(c)?;
                self.current_text.push(c);
                self.previous_char = Some(c);
            }
        }
        Ok(())
    }
    fn apply_code_token(&mut self, _: char) -> Result<(), ParseError<R::Error>> {
        if self.current_token == "```" {
            self.current_token.clear();
            self.mode_func = Self::analyse_text;
            self.renderer.push_token(token::Token::EndCode)?;
        }
        return Ok(());
    }
    fn apply_text_token(&mut self, current_char: char) -> Result<(), ParseError<R::Error>> {
        'skip: {
            if self.current_token.is_empty() {
                break 'skip;
            }

            if self.previous_char.is_none() {
                if self.current_token == "```" {
                    self.renderer.push_token(token::Token::BeginCode { language: None })?;
                    self.current_token.clear();
                    self.mode_func = Self::analyse_code_block;
                    return Ok(());
                }
            }

            let is_begin = self.previous_char.map(|c| !c.is_alphanumeric()) == Some(true) || self.previous_char == None;
            let is_end = !current_char.is_alphanumeric(); // note: newline MUST resets state, so no need to check
            if is_begin == is_end {
                break 'skip;
            }
            let inline_style = match self.current_token.as_str() {
                "*" => token::InlineStyleToken::OneStar,
                "**" => token::InlineStyleToken::TwoStars,
                "***" => token::InlineStyleToken::ThreeStars,
                "_" => token::InlineStyleToken::OneDash,
                "__" => token::InlineStyleToken::TwoDashes,
                "`" => token::InlineStyleToken::OneQuote,
                _ => break 'skip,
            };
            let inline_style = if is_begin { token::Marker::Begin(inline_style) } else { token::Marker::End(inline_style) };
            self.push_current_text()?;
            self.renderer.push_token(token::Token::InlineStyle(inline_style))?;
            self.current_token.clear();
        }
        if !self.current_token.is_empty() {
            self.current_text.push_str(&self.current_token);
            self.current_token.clear();
        }
        Ok(())
    }
    pub fn end_of_document(&mut self) -> Result<(), ParseError<R::Error>> {
        self.push_current_text()?;
        self.renderer.push_token(token::Token::EndDocument)?;
        Ok(())
    }

    fn push_current_text(&mut self) -> Result<(), ParseError<R::Error>> {
        if !self.current_text.is_empty() {
            let mut s = String::new();
            std::mem::swap(&mut s, &mut self.current_text);
            self.renderer.push_token(token::Token::Text(s))?
        }
        Ok(())
    }
}