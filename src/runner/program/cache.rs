use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Cache {
    programs: std::collections::HashMap<String, String>,
}

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("error while accessing to the cache file: {0}")]
    Io(#[from] std::io::Error),
    #[error("error while parsing or encoding the cache file: {0}")]
    Parse(#[from] serde_yaml::Error),
    #[error("Unable to access to cache directory")]
    NoParent,
}

impl Cache {
    fn get() -> Result<Self, CacheError> {
        let file_path = Self::cache_path();
        if !file_path.exists() {
            return Ok(Default::default());
        }
        let cache_file = std::fs::File::open(file_path)?;
        Ok(serde_yaml::from_reader(cache_file)?)
    }
    fn cache_path() -> std::path::PathBuf {
        std::path::Path::new(crate::home_dir()).join(".cache").join("aio.yaml")
    }
    pub fn get_program(program: &str) -> Result<Option<String>, CacheError> {
        let cache = Self::get()?;
        let Some(path) = cache.programs.get(program) else { return Ok(None); };
        if !std::path::Path::new(path).exists() {
            return Ok(None);
        }
        Ok(Some(path.clone()))
    }
    pub fn set_program(program: String, path: String) -> Result<(), CacheError> {
        let file_path = Self::cache_path();
        let Some(parent) = file_path.parent() else { return Err(CacheError::NoParent); };
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        let mut cache = Self::get()?;
        cache.programs.insert(program.to_string(), path.to_string());
        let mut cache_file = std::fs::File::create(file_path)?;
        serde_yaml::to_writer(&mut cache_file, &cache)?;
        Ok(())
    }
}