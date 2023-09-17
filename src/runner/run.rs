
use std::borrow::Cow;
use thiserror::Error;
use super::CodeBlock;

#[derive(Error, Debug)]
pub enum RunError {
    #[error("search error: {0}")]
    Search(#[from] SearchError),
    #[error("program not found")]
    ProgramNotFount,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
#[derive(Error, Debug)]
pub enum SearchError {
    #[error("env var {0} not found: {1}")]
    EnvVarNotFound(Cow<'static, str>, std::env::VarError),
    #[error("io error: {0}")]
    Io(std::io::Error),
    #[error("bad utf-8 encoding in path")]
    BadUTF8,
    #[error("no corresponding program found")]
    NoCorrespondingProgram,
}

#[cfg(target_family = "unix")]
fn search_program(program: &str) -> Result<Option<String>, SearchError> {
    let path = std::env::var("PATH").map_err(|e| SearchError::EnvVarNotFound("PATH".into(), e))?;
    let found = path.split(':').find_map(|p| {
        let Ok(mut directory) = std::fs::read_dir(p) else { return None; };
        let found_program = directory.find(|res_item| {
            let Ok(item) = res_item else { return false };
            let Ok(file_type) = item.file_type() else { return false };
            (file_type.is_file() || file_type.is_symlink()) && item.file_name() == program
        });
        found_program
            .and_then(Result::ok)
            .map(|v| v.path())
    });
    match found {
        None => Ok(None),
        Some(found) => {
            let found = found.to_str().ok_or(SearchError::BadUTF8)?;
            Ok(Some(found.to_string()))
        },
    }
}
trait IntoBox {
    fn into_box(self) -> Result<Option<Box<dyn Program>>, SearchError>;
}
impl<T: 'static + Program> IntoBox for Result<Option<T>, SearchError> {
    fn into_box(self) -> Result<Option<Box<dyn Program>>, SearchError> {
        self.map(|opt_program| opt_program.map(|program| -> Box<dyn Program> { Box::new(program) }))
    }
    
}

fn get_program(language: &str) -> Result<Option<Box<dyn Program>>, SearchError> {
    match language {
        "bash" | "sh" | "shell" | "zsh" => ShellProgram::search().into_box(),
        _ => Err(SearchError::NoCorrespondingProgram),
    }
}

trait Program {
    fn run(&self, code_block: &CodeBlock) -> Result<std::process::Output, std::io::Error>;
}
struct ShellProgram(String);

impl Program for ShellProgram {
    
    fn run(&self, code_block: &CodeBlock) -> Result<std::process::Output, std::io::Error> {
        let mut process = std::process::Command::new(&self.0);
        process
            .arg("-c")
            .arg(&code_block.code);
        let child = process.spawn()?;
        Ok(child.wait_with_output()?)
    }
}
impl ShellProgram {
    fn search() -> Result<Option<Self>, SearchError> {
        if let Some(found) = search_program("zsh")? {
            return Ok(Some(Self(found)));
        }
        if let Some(found) = search_program("bash")? {
            return Ok(Some(Self(found)));
        }
        if let Some(found) = search_program("sh")? {
            return Ok(Some(Self(found)));
        }
        Ok(None)
    }
}
pub fn run(code_block: &CodeBlock) -> Result<std::process::Output, RunError> {
    let program = get_program(code_block.language.as_str())?;
    let program = program.expect("Program not found");
    Ok(program.run(code_block)?)
}

