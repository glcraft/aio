mod program;
use anyhow::Result;
use crossterm::ExecutableCommand;
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
    interactive: bool,
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
        use std::io::IsTerminal;
        if !std::io::stdout().is_terminal() {
            // No code execution allowed if not in a terminal
            return Ok(())
        }
        match self.interactive {
            false => {
                for code_block in self.codes.iter() {
                    program::run(code_block)?;
                }
            },
            true => self.interactive_interface()?,
        }
        
        Ok(())
    }
}

impl Runner {
    pub fn new() -> Self {
        Self  {
            is_newline: true,
            interactive: true,
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
    fn interactive_interface(&mut self) -> Result<()> {
        use std::io::Write;
        if self.codes.is_empty() {
            return Ok(());
        }
        loop {
            print!("Execute code ?\n1-{}: index of the code block\nq: quit\n> ", self.codes.len());
            std::io::stdout().flush()?;
            let mut stdin_buf = String::new();
            std::io::stdin().read_line(&mut stdin_buf)?;
            let stdin_buf = stdin_buf.trim();
            if stdin_buf == "q" {
                return Ok(());
            }
            let index = match stdin_buf.parse::<isize>() {
                Ok(i) => i,
                Err(_) => {
                    println!("Not a number");
                    continue;
                }
            };
            if !(1..=self.codes.len() as isize).contains(&index) {
                println!("Index out of range");
                continue;
            }
            print!("\n");
            program::run(&self.codes[index as usize-1])?;
            print!("\n");
        }
        Ok(())
    }
}