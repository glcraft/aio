use once_cell::sync::Lazy;
use super::{token, queue};
use std::io::Error;


pub const CODE_BLOCK_COUNTER_SPACE: usize = 3;
pub const CODE_BLOCK_LINE_CHAR: [char; 4] = ['─', '│', '┬', '┴'];
pub const CODE_BLOCK_MARGIN: usize = 1;
// pub const TEXT_BULLETS: [char; 4] = ['•', '◦', '▪', '▫'];

pub struct InlineStyles {
    styles: Vec<token::InlineStyleToken>,
    default_style: crossterm::style::Attributes,
}

impl Default for InlineStyles {
    fn default() -> Self {
        Self {
            styles: Vec::new(),
            default_style: crossterm::style::Attribute::Reset.into(),
        }
    }
}
impl InlineStyles {
    pub fn new(default_style: crossterm::style::Attributes) -> Self {
        Self { styles: Vec::new(), default_style }
    }
    fn apply_inline_style(&self, inline_style: &token::InlineStyleToken) -> Result<(), Error> {
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
    pub fn apply_styles(&self) -> Result<(), Error> {
        queue!(std::io::stdout(), 
            crossterm::style::ResetColor, 
            crossterm::style::SetAttribute(crossterm::style::Attribute::Reset), 
            crossterm::style::SetAttributes(self.default_style)
        )?;
        for c in self.styles.iter() {
            self.apply_inline_style(c)?;
        }

        Ok(())
    }
    pub fn push_style(&mut self, style: token::InlineStyleToken) -> Result<(), Error> {
        self.styles.push(style);
        self.apply_styles()?;
        Ok(())
    }
    pub fn pop_style(&mut self) -> Result<(), Error> {
        self.styles.pop();
        self.apply_styles()?;
        Ok(())
    }
    pub fn reset_styles(&mut self) -> Result<(), Error> {
        self.styles.clear();
        self.apply_styles()?;
        Ok(())
    }
}

pub fn repeat_char(c: char, n: usize) -> String {
    // let mut s = String::with_capacity(n);
    let mut s = String::new();
    for _ in 0..n {
        s.push(c);
    }
    s
}

#[inline]
pub fn draw_line() -> Result<(), Error> {
    static LINE_STRING: Lazy<String> = Lazy::new(|| {
        repeat_char(CODE_BLOCK_LINE_CHAR[0], CODE_BLOCK_MARGIN.max(crossterm::terminal::size().unwrap_or_default().0 as usize))
    });
    queue!(std::io::stdout(), 
        crossterm::style::Print(&*LINE_STRING)
    )
}