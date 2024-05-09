use serde::{Deserialize, Serialize};
use super::template::PromptTemplate;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Config {
    pub models: Vec<Model>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Model {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub template: PromptTemplate,
    pub parameters: ModelParameters,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct ModelParameters {
    pub n_gpu_layers: u32,
    pub split_mode: SplitMode,
    pub main_gpu: u32,
    pub vocab_only: bool,
    pub use_mmap: bool,
    pub use_mlock: bool,
}

impl Default for ModelParameters {
    fn default() -> Self {
        let def = llama_cpp::LlamaParams::default();
        Self {
            n_gpu_layers: def.n_gpu_layers,
            split_mode: def.split_mode.into(),
            main_gpu: def.main_gpu,
            vocab_only: def.vocab_only,
            use_mmap: def.use_mmap,
            use_mlock: def.use_mlock,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SplitMode {
    None,
    Layer,
    Row,
}
impl From<SplitMode> for llama_cpp::SplitMode {
    fn from(x: SplitMode) -> Self {
        match x {
            SplitMode::None => Self::None,
            SplitMode::Layer => Self::Layer,
            SplitMode::Row => Self::Row,
        }
    }
}

impl From<llama_cpp::SplitMode> for SplitMode {
    fn from(x: llama_cpp::SplitMode) -> Self {
        match x {
            llama_cpp::SplitMode::None => Self::None,
            llama_cpp::SplitMode::Layer => Self::Layer,
            llama_cpp::SplitMode::Row => Self::Row,
            _ => unreachable!("Unsupported split mode"),
        }
    }
}