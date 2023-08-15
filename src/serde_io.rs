use std::path::Path;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Yaml(serde_yaml::Error),
    Json(serde_json::Error),
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Self::Yaml(e)
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Json(e)
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO Error: {}", e),
            Error::Yaml(e) => write!(f, "YAML Error: {}", e),
            Error::Json(e) => write!(f, "JSON Error: {}", e),
        }
    }
}
pub trait DeserializeExt: serde::de::DeserializeOwned {
    fn from_yaml_memory<T>(memory: T) -> Result<Self, Error>
    where 
        T: AsRef<str>
    {
        serde_yaml::from_str(memory.as_ref()).map_err(Error::from)
    }
    fn from_yaml_file<T>(filepath: T) -> Result<Self, Error> 
    where 
        T: AsRef<Path>
    {
        let file = std::fs::File::open(filepath.as_ref())?;
        serde_yaml::from_reader(file).map_err(Error::from)
    }
    fn from_json_memory<T>(memory: T) -> Result<Self, Error>
    where
        T: AsRef<str>
    {
        serde_json::from_str(memory.as_ref()).map_err(Error::from)
    }
    fn from_json_file<T>(filepath: T) -> Result<Self, Error>
    where
        T: AsRef<Path>
    {
        let file = std::fs::File::open(filepath.as_ref())?;
        serde_json::from_reader(file).map_err(Error::from)
    }
}