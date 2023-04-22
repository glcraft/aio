use super::StyleType;
use std::io::{stdout, Write};

#[derive(Debug)]
struct CodeBlock {
    language: Option<String>,
    line: u32,
}


#[derive(Debug)]
pub struct Parser {
    current_line: String,
    // current_format: Format,
    modifiers: Vec<StyleType>,
    code_block: Option<CodeBlock>,
    previous_char: Option<char>,
    current_modifier: Option<String>,
}


impl Parser {
    pub fn new() -> Self {
        Self {
            current_line: String::new(),
            modifiers: Vec::new(),
            code_block: None,
            previous_char: None,
            current_modifier: None,
        }
    }
    pub fn push(&mut self, text: &str) {
        text.chars().for_each(|c| self.analyse_char(c));
        stdout().flush().expect("Failed to flush stdout");
    }
    fn analyse_char(&mut self, c: char) {
        if let Some(StyleType::Code) = self.modifiers.last() {
            self.analyse_code_char(c);
        }
        match c {
            '*' | '_' | '`' => {
                match &mut self.current_modifier {
                    Some(ref mut modifier) => {
                        match modifier.chars().last() {
                            Some(cmod) if cmod == c => {
                                modifier.push(c);
                            },
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
            c if c.is_whitespace() => {
                if let Some(modifier) = &self.current_modifier {
                    self.current_line.push_str(modifier);
                    self.current_modifier = None;
                }
                self.current_line.push(c);
            }
            c => {
                if let Some(modifier_str) = &self.current_modifier {
                    self.print();
                    let modifier = match modifier_str.as_str() {
                        "*" | "_" => StyleType::Italic,
                        "**" | "__" => StyleType::Bold,
                        "***" | "___" => StyleType::BoldItalic,
                        "`" => StyleType::Code,
                        _ => unreachable!("modifier not recognized"),
                    };
                    self.modifiers.push(modifier);
                    self.current_modifier = None;
                }
                self.current_line.push(c);
            }
        }
    }
    fn analyse_code_char(&mut self, c: char) {
        match self.previous_char {
            Some(c) if c.is_whitespace() => return,
            None => return,
            _ => (),
        };
        if c == '`' {
            // Print then disable inline code modifier
            self.print();
            self.modifiers.pop();
        }
    }
    fn print(&mut self) {
        draw::text(&self.current_line);
        self.current_line.clear();
    }
    pub fn flush(&mut self) {
        self.print();
        self.modifiers.clear();
    }
    
}