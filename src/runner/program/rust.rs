use super::*;
pub struct RustProgram(String);
    
impl Program for RustProgram {
    fn run(&self, code_block: &CodeBlock) -> Result<std::process::Output, std::io::Error> {
        use std::io::Write;
        let tmp = tempfile::NamedTempFile::new()?.into_temp_path();
        let tmp_path = tmp.to_str().expect("Failed to convert temp path to string");

        let mut rustc_process = std::process::Command::new(&self.0);
        rustc_process
            .args([
                "-o",
                tmp_path,
                "-",
            ])
            .stdin(std::process::Stdio::piped());
        let mut child = rustc_process.spawn()?;
        child.stdin.take().expect("Failed to get stdin of rustc").write_all(code_block.code.as_bytes())?;
        child.wait_with_output()?;

        let output = std::process::Command::new(tmp_path)
            .spawn()?
            .wait_with_output()?;

        Ok(output)
    }
}
impl RustProgram {
    pub(super) fn search() -> SearchStatus {
        match search_program("rustc") {
            Ok(Some(found)) => SearchStatus::Found(Box::new(Self(found))),
            Err(e) => SearchStatus::Error(e),
            _ => SearchStatus::NotFound
        }
    }
}