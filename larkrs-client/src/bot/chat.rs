#![allow(dead_code)]

use crate::LarkApiResponse;
use crate::auth::FeishuTokenManager;
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde_json::Value;
use thiserror::Error;

use super::{ChatInfoItem, ChatListResponse, SendMessageRequest};

#[derive(Error, Debug)]
pub enum ChatApiError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("JSON serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("API error: {message} (code: {code})")]
    ApiError { code: i32, message: String },
}

pub struct ChatClient {
    token_manager: FeishuTokenManager,
}

impl ChatClient {
    pub fn new() -> Self {
        Self {
            token_manager: FeishuTokenManager::new(),
        }
    }

    /// Send a message to a chat
    ///
    /// See: https://open.feishu.cn/document/server-docs/im-v1/message/create
    pub async fn send_message(&self, request: SendMessageRequest) -> Result<Value> {
        let token = self.token_manager.get_token().await?;

        let url = "https://open.feishu.cn/open-apis/im/v1/messages?receive_id_type=chat_id";

        let resp = Client::new()
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow!(e).context("Failed to send request for sending message"))?
            .json::<LarkApiResponse<Value>>()
            .await
            .map_err(|e| anyhow!(e).context("Failed to parse send message response"))?;

        match resp.is_success() {
            true => Ok(resp.data),
            false => Err(anyhow!(ChatApiError::ApiError {
                code: resp.code,
                message: resp.msg.clone(),
            })
            .context(format!("API returned error code: {}", resp.code))),
        }
    }

    pub async fn send_text_message(&self, chat_id: &str, text: &str) -> Result<Value> {
        let request = SendMessageRequest::text(chat_id, text);
        self.send_message(request).await
    }

    /// Send a markdown message to a chat
    ///
    /// See: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/im-v1/message/create_json#45e0953e
    pub async fn send_markdown_message(
        &self,
        chat_id: &str,
        title: &str,
        content: &str,
    ) -> Result<Value> {
        // Create a simple markdown element with the content
        let elements = vec![vec![super::MarkdownElement {
            tag: "md".to_string(),
            text: content.to_string(),
            style: None,
        }]];

        let request = SendMessageRequest::markdown(chat_id, title, elements);
        self.send_message(request).await
    }

    /// Get a list of chats
    ///
    /// See: https://open.feishu.cn/document/server-docs/im-v1/chat/list
    pub async fn get_chat_group_list(&self) -> Result<Vec<ChatInfoItem>> {
        let token = self.token_manager.get_token().await?;

        // Using reqwest's built-in query parameter handling
        let resp = Client::new()
            .get("https://open.feishu.cn/open-apis/im/v1/chats")
            .query(&[("page_size", "20"), ("sort_type", "ByCreateTimeAsc")])
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json; charset=utf-8")
            .send()
            .await
            .map_err(|e| anyhow!(e).context("Failed to send request for getting chat list"))?
            .json::<LarkApiResponse<ChatListResponse>>()
            .await
            .map_err(|e| anyhow!(e).context("Failed to parse chat list response"))?;

        match resp.is_success() {
            true => Ok(resp.data.into()),
            false => Err(anyhow!(ChatApiError::ApiError {
                code: resp.code,
                message: resp.msg.clone(),
            })
            .context(format!("API returned error code: {}", resp.code))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_chat_group_list() {
        dotenvy::dotenv().ok();

        let client = ChatClient::new();
        let result = client.get_chat_group_list().await;

        println!("Result: {:?}", result);
    }

    #[tokio::test]
    async fn test_send_text_message() {
        dotenvy::dotenv().ok();

        let client = ChatClient::new();
        let chat_id = std::env::var("CHAT_ID").unwrap();
        let result = client
            .send_text_message(&chat_id, "Test message from Rust API")
            .await;

        println!("Send message result: {:?}", result);
    }

    #[tokio::test]
    async fn test_send_markdown_message() {
        dotenvy::dotenv().ok();

        let client = ChatClient::new();
        let chat_id = std::env::var("CHAT_ID").unwrap();

        let markdown_content = "# 股票市场实时数据\n\n**今日热门股票列表**\n\n- **阿里巴巴 (BABA)**: ¥78.45 📈 +2.3%\n- **腾讯控股 (0700.HK)**: ¥321.80 📉 -1.5%\n- **美团 (3690.HK)**: ¥125.60 📈 +3.7%\n- **京东 (JD)**: ¥142.30 📈 +0.8%\n- **百度 (BIDU)**: ¥112.75 📉 -2.1%\n- **小米集团 (1810.HK)**: ¥12.86 📈 +4.2%\n- **拼多多 (PDD)**: ¥89.35 📈 +5.6%\n\n> 数据更新时间: 2025-04-01 19:30:00";

        let result = client
            .send_markdown_message(&chat_id, "我是一个标题", markdown_content)
            .await;

        println!("Send markdown message result: {:?}", result);
    }
}
