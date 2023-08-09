use super::InlineStyles;
use super::super::utils;

use crossterm::{
    queue, 
    style::*,
    ErrorKind,
};

pub struct Header {
    level: usize,
    text: String,
    styles: InlineStyles,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            level: 1,
            text: String::new(),
            styles: InlineStyles::default(),
        }
    }
}

impl Header {
    pub fn new(level: usize) -> Self {
        Self {
            level,
            text: String::new(),
            styles: InlineStyles::new(Attributes::from([Attribute::Reverse, Attribute::Bold].as_slice())),
        }
    }
    pub fn draw_line(&self) -> Result<(), ErrorKind> {
        let pos_cursor = crossterm::cursor::position()?.0 as isize;
        let line_length = self.header_width()? - pos_cursor;

        queue!(std::io::stdout(), 
            Print(utils::repeat_char(utils::CODE_BLOCK_LINE_CHAR[0], utils::CODE_BLOCK_MARGIN.max(line_length as usize)))
        )?;
        Ok(())
    }
    pub fn draw_text(&self) -> Result<(), ErrorKind> {
        let pos_cursor = crossterm::cursor::position()?;
        let new_cursor_pos = (0.max((self.header_width()? - self.text.len() as isize) as u16), pos_cursor.1);
        
        queue!(std::io::stdout(), 
            crossterm::cursor::MoveTo(new_cursor_pos.0, new_cursor_pos.1),
            Print(self.text)
        )
    }
    fn header_width(&self) -> Result<isize, ErrorKind> {
        let term_width = crossterm::terminal::size()?.0 as isize;
        Ok(term_width / (1<<(self.level-1)))
    }
}