use super::StyleType;
use std::io::{stdout, Write};
use super::Renderer;

#[derive(Debug)]
struct CodeBlock {
    language: Option<String>,
    line: u32,
}


#[derive(Debug)]
pub struct Parser<R: Renderer> {
    current_line: String,
    // current_format: Format,
    styles: Vec<StyleType>,
    code_block: Option<CodeBlock>,
    previous_char: Option<char>,
    current_modifier: Option<String>,

    renderer: R,
}

impl<R: Renderer> Parser<R> {
    pub fn new(renderer: R) -> Self {
        Self {
            current_line: String::new(),
            styles: Vec::new(),
            code_block: None,
            previous_char: None,
            current_modifier: None,
            renderer,
        }
    }
    pub fn push(&mut self, text: &str) {
        text.chars().for_each(|c| self.analyse_char(c));
        self.print();
        stdout().flush().expect("Failed to flush stdout");
    }
    fn analyse_char(&mut self, c: char) {
        match c {
            '\n' => {
                self.apply_modifier(c);
                self.flush(false);
                self.renderer.newline();
            },
            c if self.is_inline_code() => self.analyse_code_char(c),
            '*' | '_' | '`' => {
                match &mut self.current_modifier {
                    Some(ref mut modifier) => {
                        match modifier.chars().last() {
                            Some(cmod) if cmod == c => modifier.push(c),
                            None => unreachable!("modifier should not be empty at this point"),
                            _ => {
                                self.current_line.push_str(&modifier);
                                self.current_modifier = Some(c.to_string());
                            },
                        }
                    } 
                    None => self.current_modifier = Some(c.to_string())
                };
            }
            c => {
                self.apply_modifier(c);
                self.current_line.push(c);
                self.previous_char = Some(c);
            }
        }
        
    }
    fn analyse_code_char(&mut self, c: char) {
        match (c, self.previous_char) {
            ('`', Some(prevc)) if prevc.is_whitespace() => {
                self.current_line.push(c);
                self.previous_char = Some(c);
            },
            ('`', _) => self.pop_style(),
            _ => {
                self.current_line.push(c);
                self.previous_char = Some(c);
            },
        }
    }
    fn apply_modifier(&mut self, current_char: char) {
        if let None = &self.current_modifier {
            return;
        }

        let modifier = self.current_modifier.as_ref().unwrap().clone();
        let style = (&modifier).into();
        match self.styles.last() {
            Some(StyleType::CodeBlock) => self.current_line.push_str(&modifier),
            Some(s) if s == &style => {
                if self.previous_char.map(char::is_whitespace).unwrap_or(false) {
                    self.current_line.push_str(&modifier);
                } else {
                    self.pop_style()
                }
            },
            _ if !current_char.is_whitespace() => {
                self.print();
                self.styles.push(style);
                match self.renderer.apply_style(style) {
                    Ok(_) => Ok(()),
                    Err(super::renderer::Error::NotSupported) => {
                        self.current_line.push_str(&modifier);
                        Ok(())
                    },
                    e => e,
                }.expect("Failed to apply style");
            }
            _ => self.current_line.push_str(&modifier)
        }
        self.current_modifier = None;
    }
    fn pop_style(&mut self) {
        self.print();
        self.styles.pop();
        self.renderer.reset_style().expect("Failed to reset style");
        self.styles.iter()
            .map(|s| {
                match self.renderer.apply_style(*s) {
                    Ok(_) => Ok(()),
                    Err(super::renderer::Error::NotSupported) => self.renderer.print_text(self.current_modifier.as_ref().unwrap()),
                    Err(e) => Err(e),
                }
            }).fold(Ok(()), Result::and).expect("Failed to apply style");
    }
    fn is_inline_code(&self) -> bool {
        self.styles.last() == Some(&StyleType::Code)
    }
    
    fn print(&mut self) {
        self.renderer.print_text(&self.current_line);
        self.current_line.clear();
    }
    fn flush(&mut self, flush_io: bool) {
        self.print();
        self.current_modifier = None;
        self.styles.clear();
        self.renderer.reset_style().expect("Failed to reset style");
        if flush_io {
            stdout().flush().expect("Failed to flush stdout");
        }
    }
    pub fn end_of_document(&mut self) {
        self.flush(true);
    }
    
}