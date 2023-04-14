use lazy_static::lazy_static;
use regex::{Regex, Replacer};
use serde::Deserialize;
use crate::openai::{ChatRequestParameters, Role, Message};
use crate::arguments as args;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub prompts: Vec<Prompt>,
    pub api_key: Option<String>,
}
#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Yaml(serde_yaml::Error),
}

impl Config {
    pub fn load(args: &args::Args) -> Result<Config, ConfigError> {
        let config = std::fs::read_to_string("config.yml").map_err(ConfigError::Io)?;
        let mut config: Config = serde_yaml::from_str(&config).map_err(ConfigError::Yaml)?;
        
        Ok(config)
    }
}
#[derive(Debug, Deserialize)]
pub struct Parameters {
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    presence_penalty: Option<f32>,
    frequency_penalty: Option<f32>,
    best_of: Option<u32>,
    n: Option<u32>,
    stop: Option<String>,
}
impl From<Parameters> for ChatRequestParameters {
    fn from(parameters: Parameters) -> Self {
        Self {
            max_tokens: parameters.max_tokens,
            temperature: parameters.temperature,
            top_p: parameters.top_p,
            presence_penalty: parameters.presence_penalty,
            frequency_penalty: parameters.frequency_penalty,
            best_of: parameters.best_of,
            n: parameters.n,
            stop: parameters.stop,
            ..Default::default()
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct Prompt {
    pub name: String,
    pub messages: Vec<Message>,
    pub parameters: Parameters,
}

impl Prompt {
    pub fn format_messages(mut self, args: &args::Args) -> Self{
        let re = Regex::new(r"\$(\w+)").unwrap();
        for message in &mut self.messages {
            let data_captures = re.captures_iter(&message.content).filter_map(|cap| {
                let range = cap.get(0).unwrap().range();
                if range.start > 0 && message.content.get(range.start - 1..range.start).and_then(|c| if c == "$" { Some(()) } else { None }).is_some() {
                    return None;
                }
                let name = cap.get(1).unwrap().as_str();
                Some((String::from(name), range))
            }).collect::<Vec<_>>();
            for (name, range) in data_captures {
                match name.as_str() {
                    "input" => message.content.replace_range(range, &args.input),
                    _ => {}
                }
            }
            while let Some(found) = message.content.find("$$") {
                message.content.replace_range(found..found + 2, "$");
            }
        }
        self
    }
}