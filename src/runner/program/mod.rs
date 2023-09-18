mod cache;

mod shell;
use shell::*;
mod rust;
use rust::*;
mod python;
use python::*;

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
    #[error("bad utf-8 encoding in path")]
    BadUTF8,
    #[error("no corresponding program found for `{0}`")]
    NoCorrespondingProgram(String),
    #[error("cache error: {0}")]
    Cache(#[from] cache::CacheError)
}

enum SearchStatus {
    Found(Box<dyn Program>),
    NotFound,
    Error(SearchError)
}

fn search_program(program: &str) -> Result<Option<String>, SearchError> {
    if let Some(found) = cache::Cache::get_program(program)? {
        return Ok(Some(found));
    }
    #[cfg(target_family = "unix")]
    const SEPARATOR: char = ':';
    #[cfg(target_family = "windows")]
    const SEPARATOR: char = ';';

    let path = std::env::var("PATH").map_err(|e| SearchError::EnvVarNotFound("PATH".into(), e))?;
    let found = path.split(SEPARATOR).find_map(|p| {
        let mut directory = std::fs::read_dir(p).ok()?;
        let found_program = directory.find(|res_item| {
            let Ok(item) = res_item else { return false };
            let Ok(file_type) = item.file_type() else { return false };
            if !(file_type.is_file() || file_type.is_symlink()) {
                return false;
            }
            #[cfg(target_family = "unix")]
            return item.file_name() == program;
            #[cfg(target_family = "windows")]
            {
                use std::ffi::{OsString, OsStr};
                let os_program = OsString::from(program.to_lowercase());
                let os_extension = OsString::from("exe");
                let path = item.path();
                let Some(filestem) = path.file_stem().map(OsStr::to_ascii_lowercase) else { return false };
                let Some(extension) = path.file_stem().map(OsStr::to_ascii_lowercase) else { return false };
                return filestem == os_program && extension == os_extension;
            }
        });
        found_program
            .and_then(Result::ok)
            .map(|v| v.path())
    });
    let Some(found) = found else { return Ok(None); };
    let found = found.to_str().ok_or(SearchError::BadUTF8)?.to_string();
    cache::Cache::set_program(program.into(), found.clone())?;
    Ok(Some(found))
}

fn get_program(language: &str) -> SearchStatus {
    match language {
        "bash" | "sh" | "shell" | "zsh" => ShellProgram::search(&["zsh", "bash", "sh"]),
        "nu" => ShellProgram::search(&["nu"]),
        "pwsh" | "powershell" => ShellProgram::search(&["pwsh", "powershell"]),
        "rust" | "rs" => RustProgram::search(),
        "py" | "python" => PythonProgram::search(),
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

