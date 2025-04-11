//! # larkrs-client
//!
//! A Rust client library for the Lark (Feishu) API.
//!
//! This library provides a convenient way to interact with Lark (Feishu) APIs,
//! including authentication, Bitable operations, and bot messaging.
//!
//! ## Features
//!
//! - Authentication: Tenant access token management with automatic refresh
//! - Bitable: Read and write operations for Feishu Bitable
//! - Bot: Send messages and interact with chats
//!
//! ## Example
//!
//! ```rust,no_run
//! use larkrs_client::auth::FeishuTokenManager;
//! use larkrs_client::bot::chat::ChatClient;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Get a token
//!     let token_manager = FeishuTokenManager::new();
//!     let token = token_manager.get_token().await?;
//!
//!     // Send a message
//!     let client = ChatClient::new();
//!     client.send_text_message("chat_id", "Hello from Rust!").await?;
//!
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};

pub mod auth;
pub mod bitable;
pub mod bot;

/// Response structure for Lark API calls.
///
/// All Lark API responses follow this common structure with a code, message, and data payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct LarkApiResponse<T> {
    /// Status code. 0 indicates success, other values indicate failure.
    pub code: i32,
    /// Status message. Empty when successful, error description when failed.
    pub msg: String,
    /// The actual response data. This is generic and depends on the specific API call.
    #[serde(default)]
    pub data: T,
}

impl<T> LarkApiResponse<T> {
    /// Checks if the API call was successful.
    ///
    /// Returns `true` if the status code is 0, which indicates success in Lark API.
    pub fn is_success(&self) -> bool {
        self.code == 0
    }
}
