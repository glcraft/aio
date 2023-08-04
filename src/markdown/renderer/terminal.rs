use crossterm::{
    queue,
    ErrorKind
};
use std::io::Write;
use super::token;
use super::Renderer;


pub struct TerminalRenderer {
    states: Vec<token::InlineStyleToken>,
}

impl TerminalRenderer {
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
        }
    }
    pub fn apply_inline_style(&self, inline_style: &token::InlineStyleToken) -> Result<(), <Self as Renderer>::Error> {
        use crossterm::style::*;
        match inline_style {
            token::InlineStyleToken::OneStar => queue!(std::io::stdout(), SetAttribute(Attribute::Italic)),
            token::InlineStyleToken::TwoStars => queue!(std::io::stdout(), SetAttribute(Attribute::Bold)),
            token::InlineStyleToken::ThreeStars => queue!(std::io::stdout(), SetAttribute(Attribute::Italic), SetAttribute(Attribute::Bold)),
            token::InlineStyleToken::OneDash => queue!(std::io::stdout(), SetAttribute(Attribute::Italic)),
            token::InlineStyleToken::TwoDashes => queue!(std::io::stdout(), SetAttribute(Attribute::Underlined)),
            token::InlineStyleToken::OneQuote => queue!(std::io::stdout(), SetForegroundColor(Color::Yellow)),
        }
    }
    pub fn apply_styles(&self) -> Result<(), <Self as Renderer>::Error> {
        //reset attributes
        queue!(std::io::stdout(), crossterm::style::SetAttribute(crossterm::style::Attribute::Reset))?;
        for c in self.states.iter() {
            self.apply_inline_style(c)?;
        }
        Ok(())
    }
    pub fn push_style(&mut self, style: token::InlineStyleToken) -> Result<(), <Self as Renderer>::Error> {
        self.states.push(style);
        self.apply_styles()?;
        Ok(())
    }
    pub fn pop_style(&mut self) -> Result<(), <Self as Renderer>::Error> {
        self.states.pop();
        self.apply_styles()?;
        Ok(())
    }
    pub fn reset_styles(&mut self) -> Result<(), <Self as Renderer>::Error> {
        self.states.clear();
        self.apply_styles()?;
        Ok(())
    }
}

impl Renderer for TerminalRenderer {
    type Error = ErrorKind;
    fn push_token(&mut self, style: token::Token) -> Result<(), Self::Error> {
        match style {
            token::Token::Text(s) => queue!(std::io::stdout(), crossterm::style::Print(s)),
            token::Token::Newline => {
                queue!(std::io::stdout(), crossterm::style::Print("\n"))?;
                self.reset_styles()
            },
            token::Token::InlineStyle(token::Marker::Begin(inline_style)) => {
                self.push_style(inline_style)
            }
            token::Token::InlineStyle(token::Marker::End(_)) => {
                self.pop_style()
            }
            token::Token::EndDocument => queue!(std::io::stdout(), crossterm::style::Print("\n")),
            _ => todo!("not implemented"),
        }
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
        std::io::stdout().flush()
    }
}