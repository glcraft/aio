mod shell;
use shell::*;

use std::borrow::Cow;
use thiserror::Error;
use super::CodeBlock;

trait Program {
    fn run(&self, code_block: &CodeBlock) -> Result<std::process::Output, std::io::Error>;
}

#[derive(Error, Debug)]
pub enum RunError {
    #[error("error while searching a program: {0}")]
    Search(#[from] SearchError),
    #[error("program not found for `{0}`")]
    ProgramNotFound(String),
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
    #[error("no corresponding program found for `{0}`")]
    NoCorrespondingProgram(String),
}

enum SearchStatus {
    Found(Box<dyn Program>),
    NotFound,
    Error(SearchError)
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

fn get_program(language: &str) -> SearchStatus {
    match language {
        "bash" | "sh" | "shell" | "zsh" => ShellProgram::search(&["zsh", "bash", "sh"]),
        "nu" => ShellProgram::search(&["nu"]),
        _ => SearchStatus::Error(SearchError::NoCorrespondingProgram(language.to_string())),
    }
}


pub fn run(code_block: &CodeBlock) -> Result<std::process::Output, RunError> {
    let program = match get_program(code_block.language.as_str()) {
        SearchStatus::Found(found) => found,
        SearchStatus::NotFound => return Err(RunError::ProgramNotFound(code_block.language.clone())),
        SearchStatus::Error(e) => return Err(RunError::Search(e)),
    };
    Ok(program.run(code_block)?)
}

