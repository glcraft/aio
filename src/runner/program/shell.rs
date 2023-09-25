use super::*;
pub struct ShellProgram(String);
    
impl Program for ShellProgram {
    fn run(&self, code_block: &CodeBlock) -> Result<std::process::Output, std::io::Error> {
        let mut process = std::process::Command::new(&self.0);
        process
            .arg("-c")
            .arg(&code_block.code);
        let child = process.spawn()?;
        child.wait_with_output()
    }
}
impl ShellProgram {
    pub(super) fn search(list_shells: &[&'static str]) -> SearchStatus {
        for shell in list_shells {
            match search_program(shell) {
                Ok(Some(found)) => return SearchStatus::Found(Box::new(Self(found))),
                // Err(e) => println!("Warning during search for {}: {}", shell, e),
                _ => continue
            }
        }
        SearchStatus::NotFound
    }
}