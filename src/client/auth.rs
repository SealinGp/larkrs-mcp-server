#![allow(dead_code)]

use anyhow::{Result, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FeishuApiError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("JSON serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("API error: {message} (code: {code})")]
    ApiError { code: i32, message: String },

    #[error("Token refresh error: {0}")]
    TokenRefreshError(String),
}

#[derive(Debug, Serialize)]
pub struct TenantAccessTokenRequest {
    pub app_id: String,
    pub app_secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TenantAccessTokenResponse {
    pub code: i32,
    pub msg: String,
    #[serde(default)]
    pub tenant_access_token: String,
    #[serde(default)]
    pub expire: i32,
}

impl TenantAccessTokenResponse {
    pub fn is_success(&self) -> bool {
        self.code == 0
    }
}

#[derive(Debug, Clone)]
struct TokenCache {
    token: String,
    expiry: Instant,
}

/// Feishu token manager that manages token refresh
pub struct FeishuTokenManager {
    token_cache: Arc<Mutex<Option<TokenCache>>>,
    /// Token refresh buffer time in seconds (default: 60 seconds)
    refresh_buffer: u64,
}

impl FeishuTokenManager {
    pub fn new() -> Self {
        Self {
            token_cache: Arc::new(Mutex::new(None)),
            refresh_buffer: 60, // 1 minute buffer before expiry
        }
    }

    pub fn with_refresh_buffer(mut self, seconds: u64) -> Self {
        self.refresh_buffer = seconds;
        self
    }

    pub async fn get_token(&self) -> Result<String> {
        if let Some(token) = self.get_cached_token() {
            return Ok(token);
        }

        self.refresh_token().await
    }

    fn get_cached_token(&self) -> Option<String> {
        let cache = self.token_cache.lock().unwrap();

        if let Some(cached) = &*cache {
            if Instant::now() < cached.expiry {
                return Some(cached.token.clone());
            }
        }

        None
    }

    async fn refresh_token(&self) -> Result<String> {
        let token_response = self.fetch_tenant_access_token().await?;

        if !token_response.is_success() {
            let err = FeishuApiError::ApiError {
                code: token_response.code,
                message: token_response.msg.clone(),
            };
            return Err(anyhow::anyhow!(err)
                .context(format!("API returned error code: {}", token_response.code)));
        }

        let token = token_response.tenant_access_token.clone();

        let expire_secs = token_response.expire as u64;
        let buffer_secs = self.refresh_buffer;
        let actual_expire_secs = expire_secs.saturating_sub(buffer_secs);

        let cache = TokenCache {
            token: token.clone(),
            expiry: Instant::now() + Duration::from_secs(actual_expire_secs),
        };

        let mut cache_lock = self.token_cache.lock().unwrap();
        *cache_lock = Some(cache);

        Ok(token)
    }

    /// Fetch a new tenant access token from the API
    async fn fetch_tenant_access_token(&self) -> Result<TenantAccessTokenResponse> {
        let app_id = env::var("FEISHU_APP_ID").map_err(|_| {
            FeishuApiError::TokenRefreshError("FEISHU_APP_ID not set in environment".to_string())
        })?;

        let app_secret = env::var("FEISHU_APP_SECRET").map_err(|_| {
            FeishuApiError::TokenRefreshError(
                "FEISHU_APP_SECRET not set in environment".to_string(),
            )
        })?;

        let request_body = TenantAccessTokenRequest { app_id, app_secret };

        let client = Client::new();
        let token_response = client
            .post("https://open.feishu.cn/open-apis/auth/v3/tenant_access_token/internal")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| anyhow!(e).context("Failed to send request for tenant access token"))?
            .json::<TenantAccessTokenResponse>()
            .await
            .map_err(|e| anyhow!(e).context("Failed to parse tenant access token response"))?;

        Ok(token_response)
    }

    pub async fn force_refresh(&self) -> Result<String> {
        self.refresh_token().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_feishu_auth_client() {
        dotenv().ok();

        let auth_client = FeishuTokenManager::new();

        // First token fetch
        let token1 = auth_client.get_token().await.expect("Failed to get token");
        assert!(!token1.is_empty());
        println!("First token: {}", token1);

        // Second token fetch should use cache
        let token2 = auth_client.get_token().await.expect("Failed to get token");
        assert_eq!(token1, token2, "Cached token should be the same");
        println!("Second token (from cache): {}", token2);
    }
}
