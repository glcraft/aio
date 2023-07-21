use super::StyleKind;
use super::Renderer;
use super::renderer;
use smartstring::alias::String;
#[derive(Debug, Clone, Copy, PartialEq)]
enum ParserState {
    Normal,
    CodeBlock,
    InlineCode
}

#[inline]
fn char_to_string(c: char) -> String {
    let mut s = String::new();
    s.push(c);
    s
}

#[derive(Debug)]
pub struct Parser<R: Renderer> {
    current_text: String,
    // current_format: Format,
    // styles: Vec<StyleKind>,
    padding: Option<String>,
    current_state: ParserState,
    previous_char: Option<char>,
    current_token: Option<String>,

    renderer: R,
}

impl<R: Renderer> Parser<R> {
    pub fn new(renderer: R) -> Self {
        Self {
            padding: None,
            current_state: ParserState::Normal,
            previous_char: None,
            current_text: String::new(),
            current_token: None,
            renderer,
        }
    }
    pub fn push(&mut self, text: &str) -> renderer::Result<(), R::BackendErrorType> {
        match text.chars().find_map(|c| self.analyse_common_char(c).err()) {
            Some(err) => Err(err),
            None => Ok(())
        }?;
        self.print()?;
        self.renderer.flush()
    }
    fn analyse_common_char(&mut self, c: char) -> renderer::Result<(), R::BackendErrorType> {
        match self.current_state {
            ParserState::CodeBlock => self.analyse_code_block_char(c),
            ParserState::InlineCode => self.analyse_code_char(c),
            ParserState::Normal => self.analyse_normal_char(c),
        }
    }
    fn analyse_code_char(&mut self, c: char) -> renderer::Result<(), R::BackendErrorType> {
        match (c, self.previous_char) {
            ('`', Some(prevc)) if prevc.is_whitespace() => {
                self.current_text.push(c);
                self.previous_char = Some(c);
            },
            ('`', _) => {
                self.renderer.print_text(&self.current_text)?;
                self.renderer.pop_style()?;
                self.current_text.clear();
                self.current_state = ParserState::Normal;
            },
            _ => {
                self.current_text.push(c);
                self.previous_char = Some(c);
            },
        }
        Ok(())
    }
    fn analyse_code_block_char(&mut self, c: char) -> renderer::Result<(), R::BackendErrorType> {
        Ok(())
    }
    fn analyse_normal_char(&mut self, c: char) -> renderer::Result<(), R::BackendErrorType> {
        match c {
            '\n' => {
                self.renderer.newline()?;
                self.padding = Some(String::new());
                self.current_token = None;
            },
            // ' ' if self.padding.is_some() => {
            //     self.padding.as_mut().unwrap().push(c);
            //     Ok(())
            // },
            // '#' if self.padding.map(String::is_empty) == Some(true) => {
            //     self.renderer
            //     self.current_token.push(c);
            //     Ok(())
            // },
            '*' | '_' | '`' => {
                let last_char_token = self.current_token.as_ref().map(|token| token.chars().last()).flatten();
                match last_char_token {
                    Some(last_char) if last_char == c => self.current_token.as_mut().unwrap().push(c),
                    _ => {
                        if let Some(token) = self.current_token.take() {
                            self.current_text.push_str(&token);
                        }
                        self.current_token = Some(char_to_string(c));
                    }
                }
            }
            c => {
                self.apply_modifier(Some(c))?;
                self.current_text.push(c);
                self.previous_char = Some(c);
            }
        }
        Ok(())
    }

    fn apply_modifier(&mut self, current_char: Option<char>) -> renderer::Result<(), R::BackendErrorType> {
        let token = match self.current_token.take() {
            Some(token) => token,
            None => return Ok(())
        };
        let left = self.previous_char.map(char::is_whitespace);
        let right = current_char.map(char::is_whitespace);

        match (left, right) {
            (None | Some(true), Some(false)) => { // Enter style

            }
            (Some(false), Some(true)) => { // Exit style

            }
            (_,_) => {
                self.current_text.push_str(&token);
                if let Some(c) = current_char {
                    self.current_text.push(c);
                }
            }
        }
        Ok(())
    }
    fn pop_style(&mut self) -> renderer::Result<(), R::BackendErrorType> {
        self.print()?;
        self.renderer.pop_style()
    }
    fn is_normal(&self) -> bool {
        self.current_state == ParserState::Normal
    }
    
    fn print(&mut self) -> renderer::Result<(), R::BackendErrorType> {
        if !self.current_text.is_empty() {
            self.renderer.print_text(&self.current_text)?;
            self.current_text.clear();
        }
        Ok(())
    }
    fn flush(&mut self, flush_io: bool) -> renderer::Result<(), R::BackendErrorType> {
        self.print()
    }
    pub fn end_of_document(&mut self) -> renderer::Result<(), R::BackendErrorType> {
        self.flush(true)
    }
    
}