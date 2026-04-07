use crate::provider::LlmProvider;
use crate::types::{LlmResponse, Message, StreamChunk, ToolDefinition};
use anyhow::Ok;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
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
        let req_body = json!({
            "model": self.model_name,
            "messages": messages,
            "tools": tools.iter().map(|tool|{
                json!({
                    "type":"function",
                    "function":tool
                })
            }).collect::<Vec<_>>(),
            "stream": stream_tx.is_some(),
        });

        let response = self
            .http_client
            .post(&format!("{}/api/chat", self.base_url))
            .json(&req_body)
            .send()
            .await?;

        if (response.status().is_success()) {
            let response_text = response.text().await?;
            let response_json: serde_json::Value = serde_json::from_str(&response_text)?;
        } else {
            return Err(anyhow::anyhow!("LLM Error: {}", response.status()));
        }
        Ok(LlmResponse {
            content: "Success".to_string(),
            tool_calls: vec![],
        })
    }
    fn model_name(&self) -> &str {
        todo!();
    }
}
