use crate::error::OllamaError;
use crate::generation::chat::{ChatMessage, ChatMessageResponse};
use crate::generation::functions::tools::Tool;
use async_trait::async_trait;
use std::sync::Arc;

pub mod meta_llama;
pub mod nous_hermes;
pub mod openai;

#[async_trait]
pub trait RequestParserBase: Send + Sync {
    async fn parse(
        &self,
        input: &str,
        model_name: String,
        tools: Vec<Arc<dyn Tool>>,
    ) -> Result<ChatMessageResponse, OllamaError>;
    fn format_query(&self, input: &str) -> String {
        input.to_string()
    }
    fn format_response(&self, response: &str) -> String {
        response.to_string()
    }
    async fn get_system_message(&self, tools: &[Arc<dyn Tool>]) -> ChatMessage;
}
