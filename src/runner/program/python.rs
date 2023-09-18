use super::*;
pub struct PythonProgram(String);
    
impl Program for PythonProgram {
    fn run(&self, code_block: &CodeBlock) -> Result<std::process::Output, std::io::Error> {
        use std::io::Write;
        let mut process = std::process::Command::new(&self.0);
        process
            .arg("-")
            .stdin(std::process::Stdio::piped());
        let mut child = process.spawn()?;
        child.stdin.take().expect("Failed to get stdin of python").write_all(code_block.code.as_bytes())?;
        Ok(child.wait_with_output()?)
    }
}
impl PythonProgram {
    pub(super) fn search() -> SearchStatus {
        for shell in ["python3", "python"] {
            match search_program(shell) {
                Ok(Some(found)) => return SearchStatus::Found(Box::new(Self(found))),
                // Err(e) => println!("Warning during search for {}: {}", shell, e),
                _ => continue
            }
        }
        SearchStatus::NotFound
    }
}