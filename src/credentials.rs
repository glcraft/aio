use std::collections::HashMap;
use serde::Deserialize;
use crate::serde_io::DeserializeExt;


#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub apikey: HashMap<String, String>
}

impl DeserializeExt for Credentials {}

impl Credentials {
    pub fn get_api_key(&self, name: &str, env_var: Option<&str>) -> Option<String> {
        self.apikey
            .get(name)
            .cloned()
            .or_else(|| env_var.and_then(|v| std::env::var(v).ok()))
    }
}