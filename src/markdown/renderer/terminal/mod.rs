mod mode;
mod utils;

use crossterm::{
    queue,
    ErrorKind
};
use std::io::Write;
use super::token;
use super::Renderer;
use mode::*;
use utils::InlineStyles;

pub struct TerminalRenderer {
    mode: Mode,
}

impl TerminalRenderer {
    pub fn new() -> Self {
        Self {
            mode: Mode::Text(InlineStyles::default()),
        }
    }
    
    const fn counter_space() -> usize {
        (utils::CODE_BLOCK_COUNTER_SPACE + utils::CODE_BLOCK_MARGIN * 2) as _
    }
    fn push_style(&mut self, style: token::InlineStyleToken) -> Result<(), ErrorKind> {
        match &mut self.mode {
            Mode::Text(styles) => styles.push_style(style),
            Mode::Header(header) => header.push_token(token::Token::InlineStyle(token::Marker::Begin(style))),
            _ => Ok(())
        }
    }
    fn pop_style(&mut self, style: token::InlineStyleToken) -> Result<(), ErrorKind> {
        match &mut self.mode {
            Mode::Text(styles) => styles.pop_style(),
            Mode::Header(header) => header.push_token(token::Token::InlineStyle(token::Marker::End(style))),
            _ => Ok(())
        }
    }
    fn reset_styles(&mut self) -> Result<(), ErrorKind> {
        match &mut self.mode {
            Mode::Text(styles) => styles.reset_styles(),
            _ => Ok(())
        }
    }
    fn draw_line() -> Result<(), ErrorKind> {
        let line_length = crossterm::terminal::size()?.0;
        queue!(std::io::stdout(), 
            crossterm::style::Print(utils::repeat_char(utils::CODE_BLOCK_LINE_CHAR[0], utils::CODE_BLOCK_MARGIN.max(line_length as usize)))
        )
    }
    fn draw_code_separator(sens: bool /* false: down, true: up */) -> Result<(), <Self as Renderer>::Error> {
        let term_width: usize = crossterm::terminal::size()?.0.into();
        let line = format!("{0}{1}{2}",
            utils::repeat_char(utils::CODE_BLOCK_LINE_CHAR[0], Self::counter_space()),
            utils::CODE_BLOCK_LINE_CHAR[2+sens as usize], 
            utils::repeat_char(utils::CODE_BLOCK_LINE_CHAR[0], term_width - Self::counter_space() - 1)
        );
        queue!(std::io::stdout(), crossterm::style::Print(line))
    }
    fn draw_code_line_begin(index: usize) -> Result<(), <Self as Renderer>::Error> {
        let line = format!("{3}{0:0>1$}{3}{2}{3}", 
            index, 
            utils::CODE_BLOCK_COUNTER_SPACE, 
            utils::CODE_BLOCK_LINE_CHAR[1],
            utils::repeat_char(' ', utils::CODE_BLOCK_MARGIN)
        );
        queue!(std::io::stdout(), crossterm::style::Print(line))
    }
    fn push_code_language(code_language: &mut Option<String>, word: &str) {
        match code_language {
            cl @ None => *cl = Some(word.to_string()),
            Some(s) => s.push_str(word)
        }
    }
}

impl Renderer for TerminalRenderer {
    type Error = ErrorKind;
    fn push_token(&mut self, style: token::Token) -> Result<(), Self::Error> {
        match style {
            token::Token::Text(s) => {
                if let Mode::Header(h) = &mut self.mode {
                    h.push_token(token::Token::Text(s))?;
                    return Ok(());
                }
                if let Mode::Code { index, is_line_begin: is_line_begin@ true, language } = &mut self.mode {
                    if *index == 0 {
                        Self::push_code_language(language, &s);
                        return Ok(());
                    } else {
                        Self::draw_code_line_begin(*index)?;
                        *is_line_begin = false;
                    }
                }
                queue!(std::io::stdout(), crossterm::style::Print(s))
            },
            token::Token::Heading(level) => {
                let level = level.into();
                let header = Header::new(level);
                header.init()?;
                self.mode = Mode::Header(header);
                Ok(())
            }
            token::Token::Line => {
                Self::draw_line()
            }
            token::Token::Newline => {
                if let Mode::Code { index, is_line_begin, .. } = &mut self.mode {
                    *index += 1;
                    *is_line_begin = true;
                } else if matches!(&self.mode, Mode::Header {..}) {
                    self.mode = Mode::Text(InlineStyles::default());
                }
                self.reset_styles()?;
                queue!(std::io::stdout(), crossterm::style::Print("\n"))?;
                Ok(())
            },
            token::Token::InlineStyle(token::Marker::Begin(inline_style)) => {
                self.push_style(inline_style)
            }
            token::Token::InlineStyle(token::Marker::End(inline_style)) => {
                self.pop_style(inline_style)
            }
            token::Token::BeginCode => {
                Self::draw_code_separator(false)?;
                self.mode = Mode::Code {
                    index: 0,
                    is_line_begin: true,
                    language: None
                };
                Ok(())
            }
            token::Token::EndCode => {
                Self::draw_code_separator(true)?;
                self.mode = Mode::Text(InlineStyles::default());
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