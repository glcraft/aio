use std::borrow::Cow;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    arguments as args, 
    serde_io::DeserializeExt,
    generators::openai::config::Config as ConfigOpenAI
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub openai: ConfigOpenAI
}

impl DeserializeExt for Config {}

impl Default for Config {
    fn default() -> Self {
        Self {
            openai: ConfigOpenAI::default()
        }
    }
}

pub fn format_content<'a>(content: &'a str, args: &args::ProcessedArgs) -> Cow<'a, str> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<prefix>\$\$?)(?P<name>\w+)").expect("Failed to compile regex"));
    RE.replace_all(content, |caps: &regex::Captures| {
        let prefix = &caps["prefix"];
        if prefix == "$$" {
            return format!("${}", &caps["name"]);
        }
        let name = &caps["name"];
        match name {
            "input" => args.input.clone(),
            _ => String::new(),
        }
    })
}

