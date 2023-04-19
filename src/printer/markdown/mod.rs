use std::io::{stdout, Write};

use super::draw;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum FormatKind {
    Text,
    Code,
    Title(u8),
    CodeBlock { language: Option<String>, line: u32 },
}
#[derive(Debug)]
struct Format {
    kind: FormatKind,
    text: Vec<Format>,
}

#[derive(Debug)]
struct CodeBlock {
    language: Option<String>,
    line: u32,
}

#[derive(Debug)]
pub struct Printer {
    current_line: String,
    // current_format: Format,
    code_block: Option<CodeBlock>,
}


impl Printer {
    pub fn new() -> Self {
        Self {
            current_line: String::new(),
            // current_format: Format {
            //     kind: FormatKind::Text,
            //     text: Vec::new(),
            // },
            code_block: None,
        }
    }
    pub fn push(&mut self, text: &str) {
        // let text = text
        //     .replace("\r", "")
        //     .replace("\t", "    ");
        if let Some(newline) = text.find('\n') {
            self.current_line.push_str(&text[..newline]);
            self.update_style_line();
            draw::text("\n");
            self.current_line.clear();
            if let Some(code_block) = &mut self.code_block {
                draw::code_block_line(code_block.line, "");
            }
            if newline + 1 < text.len() {
                self.push(&text[newline+1..]);
            }
            // self.current_line.push_str(&text[newline+1..]);
            // draw::text(&text[newline+1..]);
        } else {
            self.current_line.push_str(&text);
            draw::text(text);
        };
        stdout().flush().expect("Failed to flush stdout");
    }
    pub fn set(&mut self, text: &str) {
        for line in text.lines() {
            self.current_line = line.to_string();
            self.update_style_line();
        }
    }
    pub fn flush(&mut self) {
        self.update_style_line();
        stdout().flush().expect("Failed to flush stdout");
    }
    fn update_style_line(&mut self) {
        lazy_static! {
            static ref RE_INLINE_CODE: Regex = Regex::new(r"`([^`]+)`").unwrap();
            static ref RE_TITLE: Regex = Regex::new(r"^(#+)\s+(.+)$").unwrap();
        }
        draw::clear_line();
        if self.current_line.starts_with("```") {
            self.code_block = match self.code_block {
                Some(_) => {
                    draw::line(draw::LineKind::Bottom);
                    None
                }, 
                None => {
                    draw::line(draw::LineKind::Top);
                    Some(CodeBlock {
                        language: {
                            let lang: String = self.current_line.chars().skip(3).collect();
                            (!lang.is_empty()).then_some(lang)
                        },
                        line: 1
                    })
                }
            };
            return;
        }
        // Code blocks are prioritary over other styles
        if let Some(code_block) = &mut self.code_block {
            draw::code_block_line(code_block.line, &self.current_line);
            code_block.line += 1;
            return;
        }
        // if let Some(cap) = RE_TITLE.captures(&self.current_line) {
        //     let level = cap.get(1).unwrap().as_str().len();
        //     let title = cap.get(2).unwrap().as_str();
        //     draw::title(level, title);
        //     return;
        // }
        // draw::text(&self.current_line);
        draw::markdown(markdown::tokenize(&self.current_line));
    }
}