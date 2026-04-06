use crate::types::{LlmResponse, Message, StreamChunk, ToolDefinition};
use async_trait::async_trait;
use tokio::sync::mpsc;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn chat(
        &self,
        messages: &[Message],
        tools: &[ToolDefinition],
        stream_tx: Option<mpsc::Sender<StreamChunk>>,
    ) -> anyhow::Result<LlmResponse>;

    fn model_name(&self) -> &str;
}
