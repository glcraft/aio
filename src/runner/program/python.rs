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
        child.wait_with_output()
    }
}
impl PythonProgram {
    pub(super) fn search() -> SearchStatus {
        if let Ok(Some(found)) = search_program("python3") {
            return SearchStatus::Found(Box::new(Self(found)));
        }
        if let Ok(Some(found)) = search_program("python") {
            if let Ok(true) = Self::check_python_version(&found) {
                return SearchStatus::Found(Box::new(Self(found)));
            }
        }
        SearchStatus::NotFound
    }
    fn check_python_version(path: &str) -> Result<bool, std::io::Error> {
        let mut command = std::process::Command::new(path);
        command.arg("-V");
        let output = command.output()?;
        if !output.status.success() {
            return Ok(false);
        }
        let str_output = String::from_utf8_lossy(&output.stdout);

        let Some(capture_version) = regex::Regex::new(r"Python (\d+)\.\d+\.\d+").unwrap().captures(&str_output) else { return Ok(false); };
        let Ok(major) = capture_version[1].parse::<u8>() else { return Ok(false); };
        Ok(major >= 3)
    }
}