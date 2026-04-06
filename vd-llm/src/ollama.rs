use crate::provider::LlmProvider;
use crate::types::{LlmResponse, Message, StreamChunk, ToolDefinition};
use async_trait::async_trait;
use reqwest::Client;
use tokio::sync::mpsc;

pub struct OllamaProvider {
    pub base_url: String,
    pub model_name: String,
    pub http_client: Client,
}

impl OllamaProvider {
    pub fn new(base_url: String, model_name: String) -> Self {
        Self {
            base_url,
            model_name,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    async fn chat(
        &self,
        messages: &[Message],
        tools: &[ToolDefinition],
        stream_tx: Option<mpsc::Sender<StreamChunk>>,
    ) -> anyhow::Result<LlmResponse> {
        todo!();
    }
    fn model_name(&self) -> &str {
        todo!();
    }
}
