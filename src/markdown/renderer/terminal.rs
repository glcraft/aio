use crossterm::{
    queue,
    ErrorKind
};
use std::io::Write;
use super::token;
use super::Renderer;

enum Mode {
    Text(Vec<token::InlineStyleToken>),
    Code {
        index: usize,
        is_line_begin: bool,
    },
}


pub struct TerminalRenderer {
    mode: Mode,
}

impl TerminalRenderer {
    const CODE_BLOCK_COUNTER_SPACE: u16 = 3;
    const CODE_BLOCK_LINE_CHAR: [char; 4] = ['─', '│', '┬', '┴'];
    pub fn new() -> Self {
        Self {
            mode: Mode::Text(Vec::new()),
        }
    }
    fn apply_inline_style(&self, inline_style: &token::InlineStyleToken) -> Result<(), <Self as Renderer>::Error> {
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
    fn apply_styles(&self) -> Result<(), <Self as Renderer>::Error> {
        if let Mode::Text(ref states) = self.mode {
            queue!(std::io::stdout(), crossterm::style::SetAttribute(crossterm::style::Attribute::Reset))?;
            for c in states.iter() {
                self.apply_inline_style(c)?;
            }
        }
        Ok(())
    }
    fn push_style(&mut self, style: token::InlineStyleToken) -> Result<(), <Self as Renderer>::Error> {
        if let Mode::Text(ref mut states) = self.mode {
            states.push(style);
            self.apply_styles()?;
        }
        Ok(())
    }
    fn pop_style(&mut self) -> Result<(), <Self as Renderer>::Error> {
        if let Mode::Text(ref mut states) = self.mode {
            states.pop();
            self.apply_styles()?;
        }
        Ok(())
    }
    fn reset_styles(&mut self) -> Result<(), <Self as Renderer>::Error> {
        if let Mode::Text(ref mut states) = self.mode {
            states.clear();
            self.apply_styles()?;
        }
        Ok(())
    }
    fn draw_code_separator(sens: bool /* false: down, true: up */) -> Result<(), <Self as Renderer>::Error> {
        let term_size = crossterm::terminal::size()?;
        let line = format!("{}{}{}", 
            std::iter::repeat(Self::CODE_BLOCK_LINE_CHAR[0])
                .take(Self::CODE_BLOCK_COUNTER_SPACE.into())
                .collect::<String>(), 
            Self::CODE_BLOCK_LINE_CHAR[2+sens as usize], 
            std::iter::repeat(Self::CODE_BLOCK_LINE_CHAR[0])
                .take((term_size.0 - Self::CODE_BLOCK_COUNTER_SPACE - 1).into())
                .collect::<String>()
        );
        queue!(std::io::stdout(), crossterm::style::Print(line))
    }
    fn draw_code_line_begin(index: usize) -> Result<(), <Self as Renderer>::Error> {
        let line = format!("{0:0>1$}{2}", index, Self::CODE_BLOCK_COUNTER_SPACE.into(), Self::CODE_BLOCK_LINE_CHAR[1]);
        queue!(std::io::stdout(), crossterm::style::Print(line))
    }
}

impl Renderer for TerminalRenderer {
    type Error = ErrorKind;
    fn push_token(&mut self, style: token::Token) -> Result<(), Self::Error> {
        match style {
            token::Token::Text(s) => {
                if let Mode::Code { index, is_line_begin: is_line_begin@ true } = &mut self.mode {
                    Self::draw_code_line_begin(*index)?;
                    *is_line_begin = false;
                }
                queue!(std::io::stdout(), crossterm::style::Print(s))
            },
            token::Token::Newline => {
                if let Mode::Code { index, is_line_begin } = &mut self.mode {
                    *index += 1;
                    *is_line_begin = true;
                }
                queue!(std::io::stdout(), crossterm::style::Print("\n"))?;
                self.reset_styles()
            },
            token::Token::InlineStyle(token::Marker::Begin(inline_style)) => {
                self.push_style(inline_style)
            }
            token::Token::InlineStyle(token::Marker::End(_)) => {
                self.pop_style()
            }
            token::Token::BeginCode { .. } => {
                Self::draw_code_separator(false)?;
                self.mode = Mode::Code {
                    index: 0,
                    is_line_begin: true,
                };
                Ok(())
            }
            token::Token::EndCode => {
                Self::draw_code_separator(true)?;
                self.mode = Mode::Text(Vec::new());
                Ok(())
            }
            token::Token::EndDocument => queue!(std::io::stdout(), crossterm::style::Print("\n")),
            _ => todo!("not implemented"),
        }
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
        std::io::stdout().flush()
    }
}