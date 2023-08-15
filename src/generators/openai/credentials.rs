use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Credentials {
    #[serde(default = "Credentials::api_key_from_env")]
    pub api_key: String
}

impl Credentials {
    fn api_key_from_env() -> String {
        std::env::var("OPENAI_API_KEY").unwrap_or_default()
    }
}