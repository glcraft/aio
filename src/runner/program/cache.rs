use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use thiserror::Error;
use serde::{Deserialize, Serialize};

use crate::filesystem;

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

static CACHE: once_cell::sync::Lazy<Result<RwLock<Cache>, CacheError>> = once_cell::sync::Lazy::new(|| {
    Cache::load().map(RwLock::new)
});

impl Cache {
    fn load() -> Result<Self, CacheError> {
        let file_path = Cache::cache_path();
        if !file_path.exists() {
            return Ok(Default::default());
        }
        let cache_file = match std::fs::File::open(file_path) {
            Ok(file) => file,
            Err(e) => return Err(e.into()),
        };
        match serde_yaml::from_reader(cache_file) {
            Ok(cache) => Ok(cache),
            Err(e) => Err(e.into()),
        }
    }
    fn save(&self) -> Result<(), CacheError> {
        let file_path = Self::cache_path();
        let Some(parent) = file_path.parent() else { return Err(CacheError::NoParent); };
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        let mut cache_file = std::fs::File::create(file_path)?;
        serde_yaml::to_writer(&mut cache_file, self)?;
        Ok(())
    }
    fn get() -> RwLockReadGuard<'static, Self> {
        match *CACHE {
            Ok(ref cache) => cache.read().expect("Error while accessing to the cache memory"),
            Err(_) => panic!("Unable to access to the cache file"),
        }
    }
    fn get_mut() -> RwLockWriteGuard<'static, Self> {
        match *CACHE {
            Ok(ref cache) => cache.write().expect("Error while accessing to the cache memory"),
            Err(_) => panic!("Unable to access to the cache file"),
        }
    }
    fn cache_path() -> std::path::PathBuf {
        std::path::Path::new(filesystem::cache_dir()).join("aio.yaml")
    }
}
pub fn get_program(program: &str) -> Option<String> {
    let cache = Cache::get();
    let path = cache.programs.get(program)?;
    if !std::path::Path::new(path).exists() {
        return None;
    }
    Some(path.clone())
}
pub fn set_program(program: String, path: String) -> Result<(), CacheError> {
    let mut cache = Cache::get_mut();
    cache.programs.insert(program.to_string(), path.to_string());
    cache.save()?;
    Ok(())
}