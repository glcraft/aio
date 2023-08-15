use std::path::Path;
use serde::Deserialize;
use crate::{
    serde_io::DeserializeExt,
    generators::openai::credentials::Credentials as CredsOpenAI
};


#[derive(Debug, Deserialize)]
struct Credentials {
    openai: CredsOpenAI
}

impl DeserializeExt for Credentials {}

