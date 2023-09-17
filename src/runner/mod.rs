mod run;
use anyhow::Result;
use super::Formatter;

#[derive(Default, Debug)]
pub struct CodeBlock {
    code: String,
    language: String,
}

impl CodeBlock {
    fn new(language: String) -> Self {
        Self { code: String::new(), language }
    }
}
#[derive(Default, Debug)]
pub struct Runner{
    is_code: bool,
    is_newline: bool,
    current_token: String,
    codes: Vec<CodeBlock>
}

impl Formatter for Runner {
    fn push(&mut self, text: &str) -> Result<()> {
        for c in text.chars() {
            match c {
                '`' => {
                    if self.is_newline { 
                        self.current_token.push(c);
                    }
                },
                '\n' => {
                    if self.current_token.starts_with("```") {
                        self.switch_code_block();
                    } else if self.is_code {
                        self.codes.last_mut().unwrap().code.push(c);
                    }
                    self.current_token.clear();
                    self.is_newline = true;
                },
                _ => {
                    if self.is_code {
                        self.codes.last_mut().unwrap().code.push(c);
                    } else if self.is_newline && self.current_token.starts_with("```") {
                        self.current_token.push(c);
                    } else {
                        self.is_newline = false;
                    }
                },
            }
        }
        Ok(())
    }
    fn end_of_document(&mut self) -> Result<()> {
        run::run(&self.codes[0])?;
        Ok(())
    }
}

impl Runner {
    pub fn new() -> Self {
        Self  {
            is_newline: true,
            .. Default::default()
        }
    }
    fn switch_code_block(&mut self) {
        self.is_code = !self.is_code;
        if self.is_code {
            let language = self.current_token[3..].trim();
            self.codes.push(CodeBlock::new(language.into()));
        } else {
            // remove last newline
            self.codes.last_mut().unwrap().code.pop();
        }
    }
}