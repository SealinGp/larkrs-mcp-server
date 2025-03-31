#![allow(dead_code)]

use crate::client::LarkApiResponse;
use crate::client::auth::FeishuTokenManager;
use anyhow::{Result, anyhow};
use reqwest::Client;
use thiserror::Error;

use super::{ChatListResponse, ChatInfoItem};

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
}
