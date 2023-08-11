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
            mode: Mode::default(),
        }
    }
    
    const fn counter_space() -> usize {
        (utils::CODE_BLOCK_COUNTER_SPACE + utils::CODE_BLOCK_MARGIN * 2) as _
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
        match (style, &mut self.mode) {
            (token::Token::Text(s), Mode::Header(h)) => {
                h.push_token(token::Token::Text(s))?;
            }
            (token::Token::Text(s), Mode::Code { index, is_line_begin: is_line_begin@ true, language }) => {
                if *index == 0 {
                    Self::push_code_language(language, &s);
                    return Ok(());
                } else {
                    Self::draw_code_line_begin(*index)?;
                    *is_line_begin = false;
                }
                queue!(std::io::stdout(), crossterm::style::Print(s))?;
            },
            (token::Token::Heading(level), Mode::Text(_)) => {
                let level = level.into();
                let header = Header::new(level);
                header.init()?;
                self.mode = Mode::Header(header);
            }
            (token::Token::Heading(_), _) => unreachable!("Header expected only in text mode"),
            (token::Token::Line, Mode::Text(_)) => {
                Self::draw_line()?;
            }
            (token::Token::Line, _) => unreachable!("Line expected only in text mode"),
            (token::Token::Newline, mode) => {
                match mode {
                    Mode::Code { index, is_line_begin, .. } => {
                        *index += 1;
                        *is_line_begin = true;
                    },
                    mode @ Mode::Header(_) => *mode = Mode::Text(InlineStyles::default()),
                    Mode::Text(inline_styles) => inline_styles.reset_styles()?,
                }
                queue!(std::io::stdout(), crossterm::style::Print("\n"))?;
            },
            (token::Token::InlineStyle(token::Marker::Begin(inline_style)), Mode::Text(inline_styles)) => {
                inline_styles.push_style(inline_style)?;
            }
            (token::Token::InlineStyle(token::Marker::End(_)), Mode::Text(inline_styles)) => {
                inline_styles.pop_style()?;
            }
            (token @ token::Token::InlineStyle(token::Marker::Begin(_) | token::Marker::End(_)), Mode::Header(header)) => {
                header.push_token(token)?;
            }
            (token::Token::BeginCode, mode @ Mode::Text(_)) => {
                Self::draw_code_separator(false)?;
                *mode = Mode::Code {
                    index: 0,
                    is_line_begin: true,
                    language: None
                };
            }
            (token::Token::BeginCode, _) => unreachable!("Code beginning expected only in text mode"),
            (token::Token::EndCode, mode @ Mode::Code { .. }) => {
                Self::draw_code_separator(true)?;
                *mode = Mode::Text(InlineStyles::default());
            }
            (token::Token::EndCode, _) => unreachable!("Code ending expected only in code mode"),
            (token::Token::EndDocument, mode) => { 
                *mode = Mode::default();
                queue!(std::io::stdout(), crossterm::style::Print("\n"))?;
            },
            _ => todo!("not implemented"),
        }
        Ok(())
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
        std::io::stdout().flush()
    }
}