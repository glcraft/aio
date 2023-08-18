use std::path::Path;
use serde::Deserialize;
use crate::{
    serde_io::DeserializeExt,
    generators::openai::credentials::Credentials as CredsOpenAI
};


#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub openai: CredsOpenAI
}

impl DeserializeExt for Credentials {}

