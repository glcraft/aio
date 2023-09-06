use std::borrow::Cow;
use regex::Regex;
use serde::Deserialize;

use crate::{
    arguments as args, 
    serde_io::DeserializeExt,
    generators::openai::config::Config as ConfigOpenAI
};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub openai: ConfigOpenAI
}
impl DeserializeExt for Config {}

pub fn format_content<'a>(content: &'a str, args: &args::ProcessedArgs) -> Cow<'a, str> {
    lazy_static::lazy_static!{
        static ref RE: Regex = Regex::new(r"(?P<prefix>\$\$?)(?P<name>\w+)").expect("Failed to compile regex");
    }
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
