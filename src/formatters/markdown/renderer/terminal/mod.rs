mod mode;
mod utils;

use crossterm::queue;
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
}

impl Renderer for TerminalRenderer {
    type Error = std::io::Error;
    fn push_token(&mut self, style: token::Token) -> Result<(), Self::Error> {
        match (style, &mut self.mode) {
            (token::Token::Text(s), Mode::Text(_)) => {
                queue!(std::io::stdout(), crossterm::style::Print(s))?;
            }
            (token::Token::Text(s), Mode::Header(h)) => {
                h.push_token(token::Token::Text(s))?;
            }
            (token::Token::BeginCode, mode @ Mode::Text(_)) => {
                let code = Code::new();
                code.init()?;
                *mode = Mode::Code(code);
            }
            (token @ token::Token::EndCode, mode @ Mode::Code(_)) => {
                let Mode::Code(code) = mode else { unreachable!() };
                code.push_token(token)?;
                *mode = Mode::Text(InlineStyles::default());
            }
            (token, Mode::Code(code)) => {
                code.push_token(token)?;
            }
            (token::Token::Heading(level), Mode::Text(_)) => {
                let level = level.into();
                let header = Header::new(level);
                header.init()?;
                self.mode = Mode::Header(header);
            }
            (token::Token::Heading(_), _) => unreachable!("Header expected only in text mode"),
            (token::Token::Line, Mode::Text(_)) => {
                utils::draw_line()?;
            }
            (token::Token::Line, _) => unreachable!("Line expected only in text mode"),
            (token::Token::Newline, mode) => {
                match mode {
                    Mode::Text(inline_styles) => inline_styles.reset_styles()?,
                    mode @ Mode::Header(_) => {
                        let inline_styles = InlineStyles::default();
                        inline_styles.apply_styles()?;
                        *mode = Mode::Text(inline_styles);
                    },
                    Mode::Code(_) => unreachable!()
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
            (token::Token::BeginCode, _) => unreachable!("Code beginning expected only in text mode"),
            (token::Token::EndCode, _) => unreachable!("Code ending expected only in code mode"),
            (token::Token::EndDocument, mode) => { 
                *mode = Mode::default();
                queue!(std::io::stdout(), crossterm::style::Print("\n"))?;
            },
        }
        Ok(())
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
        std::io::stdout().flush()
    }
}