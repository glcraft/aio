use super::super::super::token::Token;
use super::super::utils;
use crossterm::queue;
use std::io::Error;

#[derive(Default)]
pub struct Code {
    index: usize,
    language: Option<String>
}

impl Code {
    pub fn new() -> Self {
        Code {
            index: 0,
            language: None
        }
    }
    #[inline]
    pub fn init(&self) -> Result<(), Error> {
        Self::draw_code_separator(false)
    }
    pub fn push_token(&mut self, token: Token) -> Result<(), Error> {
        match token {
            Token::Text(s) if self.index == 0 => {
                self.push_language(&s);
                Ok(())
            },
            Token::Text(s) => queue!(std::io::stdout(), crossterm::style::Print(s)),
            Token::Newline => {
                self.index += 1;
                queue!(std::io::stdout(), crossterm::style::Print("\n"))?;
                self.draw_newline()
            },
            Token::EndCode => Self::draw_code_separator(true),
            _ => unreachable!("Token not supported in code mode: {:?}", token),
        }
    }
    fn push_language(&mut self, word: &str) {
        match &mut self.language {
            cl @ None => *cl = Some(word.to_string()),
            Some(s) => s.push_str(word)
        }
    }
    const fn counter_space() -> u16 {
        (utils::CODE_BLOCK_COUNTER_SPACE + utils::CODE_BLOCK_MARGIN * 2) as _
    }
    fn draw_code_separator(sens: bool /* false: down, true: up */) -> Result<(), Error> {
        queue!(std::io::stdout(), 
            crossterm::cursor::MoveToColumn(0)
        )?;
        utils::draw_line()?;
        crossterm::terminal::enable_raw_mode()?;
        queue!(std::io::stdout(), 
            crossterm::cursor::MoveToColumn(Self::counter_space() as _),
            crossterm::style::Print(utils::CODE_BLOCK_LINE_CHAR[2+sens as usize]),
            crossterm::cursor::MoveToColumn(0)
        )?;
        crossterm::terminal::disable_raw_mode()?;
        Ok(())
    }
    fn draw_newline(&self) -> Result<(), Error> {
        let line = format!("{3}{0:0>1$}{3}{2}{3}", 
            self.index, 
            utils::CODE_BLOCK_COUNTER_SPACE, 
            utils::CODE_BLOCK_LINE_CHAR[1],
            utils::repeat_char(' ', utils::CODE_BLOCK_MARGIN)
        );
        queue!(std::io::stdout(), crossterm::style::Print(line))
    }
}
