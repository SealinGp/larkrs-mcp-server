use crate::auth::FeishuTokenManager;
use crate::LarkApiResponse;
use super::{WikiContentResponse, WikiNodeResponse, WikiListResponse};
use anyhow::Result;
use reqwest::Client;
use log::{info, warn, error, debug};

pub struct WikiClient {
    token_manager: FeishuTokenManager,
    client: Client,
}

impl WikiClient {
    pub fn new() -> Self {
        Self {
            token_manager: FeishuTokenManager::new(),
            client: Client::new(),
        }
    }

    /// Get wiki node information by node token
    pub async fn get_wiki_node(&self, space_id: &str, node_token: &str) -> Result<WikiNodeResponse> {
        let token = self.token_manager.get_token().await?;
        
        let url = format!(
            "https://open.feishu.cn/open-apis/wiki/v2/spaces/{}/nodes/{}",
            space_id, node_token
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        let api_response: LarkApiResponse<WikiNodeResponse> = response.json().await?;
        
        if api_response.is_success() {
            Ok(api_response.data)
        } else {
            Ok(WikiNodeResponse::default())
        }
    }

    /// Get wiki content by node token using raw_content API
    pub async fn get_wiki_content(&self, node_token: &str) -> Result<WikiContentResponse> {
        debug!("Getting wiki content for node_token: {}", node_token);
        
        let token = self.token_manager.get_token().await?;
        
        let url = format!(
            "https://open.feishu.cn/open-apis/docx/v1/documents/{}/raw_content",
            node_token
        );

        debug!("Requesting URL: {}", url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        let status = response.status();
        debug!("HTTP Status: {}", status);

        let response_text = response.text().await?;
        debug!("Response body length: {} chars", response_text.len());

        // Try to parse as JSON
        match serde_json::from_str::<LarkApiResponse<WikiContentResponse>>(&response_text) {
            Ok(api_response) => {
                debug!("API Response code: {}, msg: {}", api_response.code, api_response.msg);
                if api_response.is_success() {
                    info!("Successfully retrieved wiki content");
                    Ok(api_response.data)
                } else {
                    warn!("API call failed with code: {}, message: {}", api_response.code, api_response.msg);
                    Ok(WikiContentResponse::default())
                }
            }
            Err(e) => {
                error!("Failed to parse JSON response: {}. Response text: {}", e, response_text);
                Ok(WikiContentResponse::default())
            }
        }
    }

    /// List wiki nodes in a space
    pub async fn list_wiki_nodes(&self, space_id: &str) -> Result<WikiListResponse> {
        let token = self.token_manager.get_token().await?;
        
        let url = format!(
            "https://open.feishu.cn/open-apis/wiki/v2/spaces/{}/nodes",
            space_id
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;

        let api_response: LarkApiResponse<WikiListResponse> = response.json().await?;
        
        if api_response.is_success() {
            Ok(api_response.data)
        } else {
            Ok(WikiListResponse::default())
        }
    }

    /// Extract wiki information from a Feishu wiki URL
    /// URL format: https://xxx.feishu.cn/wiki/{node_token}?fromScene=spaceOverview
    pub fn parse_wiki_url(&self, url: &str) -> Result<(String, String)> {
        debug!("Parsing wiki URL: {}", url);
        
        // Extract node_token from URL
        let url_parts: Vec<&str> = url.split('/').collect();
        debug!("URL parts: {:?}", url_parts);
        
        if let Some(wiki_part) = url_parts.iter().find(|&&part| part.contains("wiki")) {
            if let Some(index) = url_parts.iter().position(|&x| x == *wiki_part) {
                if index + 1 < url_parts.len() {
                    let node_token = url_parts[index + 1].split('?').next().unwrap_or("");
                    debug!("Extracted node_token: {}", node_token);
                    // For now, we'll use a default space_id or extract from URL if available
                    let space_id = "default"; // This would need to be extracted or provided
                    return Ok((space_id.to_string(), node_token.to_string()));
                }
            }
        }
        error!("Failed to parse wiki URL: {}", url);
        Err(anyhow::anyhow!("Invalid wiki URL format"))
    }
}

impl Default for WikiClient {
    fn default() -> Self {
        Self::new()
    }
}