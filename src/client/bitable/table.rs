#![allow(dead_code)]

use crate::client::LarkApiResponse;
use crate::client::auth::FeishuTokenManager;
use crate::client::bitable::{FieldsListResponse, SearchRecordsResponse};
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde_json::Value;
use thiserror::Error;

use super::BatchCreateRecordsRequest;

#[derive(Error, Debug)]
pub enum BitableApiError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("JSON serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("API error: {message} (code: {code})")]
    ApiError { code: i32, message: String },
}

pub struct BitableTableClient {
    token_manager: FeishuTokenManager,
}

impl BitableTableClient {
    pub fn new() -> Self {
        Self {
            token_manager: FeishuTokenManager::new(),
        }
    }

    /// Search records in a Bitable table
    ///
    /// See: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/search
    pub async fn get_records_list(
        &self,
        app_token: &str,
        table_id: &str,
    ) -> Result<SearchRecordsResponse> {
        let url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records/search",
            app_token, table_id
        );

        let token = self.token_manager.get_token().await?;
        let response = Client::new()
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json; charset=utf-8")
            .json(&serde_json::json!({}))
            .send()
            .await
            .map_err(|e| anyhow!(e).context("Failed to send request for searching records"))?
            .json::<LarkApiResponse<SearchRecordsResponse>>()
            .await
            .map_err(|e| anyhow!(e).context("Failed to parse search records response"))?;

        if !response.is_success() {
            let err = BitableApiError::ApiError {
                code: response.code,
                message: response.msg.clone(),
            };
            return Err(anyhow!(err).context(format!("API returned error code: {}", response.code)));
        }

        Ok(response.data)
    }

    /// Batch create multiple records in a Bitable table
    ///
    /// * [Feishu Bitable Batch Create API](https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/batch_create)
    pub async fn batch_create_records(
        &self,
        app_token: &str,
        table_id: &str,
        request: BatchCreateRecordsRequest,
    ) -> Result<()> {
        if app_token.is_empty() || table_id.is_empty() {
            return Err(anyhow!("app_token and table_id cannot be empty"));
        }
        if request.records.is_empty() {
            return Err(anyhow!("No records provided for batch creation"));
        }

        let url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_create",
            app_token, table_id
        );
        let token = self
            .token_manager
            .get_token()
            .await
            .map_err(|e| anyhow!(e).context("Failed to obtain authentication token"))?;

        let resp = Client::new()
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json; charset=utf-8")
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow!(e).context("Failed to send request for batch creating records"))?
            .json::<LarkApiResponse<Value>>()
            .await
            .map_err(|e| anyhow!(e).context("Failed to parse batch create records response"))?;

        match resp.is_success() {
            true => Ok(()),
            false => Err(anyhow!(BitableApiError::ApiError {
                code: resp.code,
                message: resp.msg.clone(),
            })
            .context(format!(
                "API returned error code: {} - {}",
                resp.code, resp.msg
            ))),
        }
    }

    pub async fn batch_create_records_json(
        &self,
        app_token: &str,
        table_id: &str,
        records_json: &str,
    ) -> Result<()> {
        if app_token.is_empty() || table_id.is_empty() {
            return Err(anyhow!("app_token and table_id cannot be empty"));
        }

        // 先尝试解析JSON字符串
        let value: Value = serde_json::from_str(records_json)
            .map_err(|e| anyhow!("Failed to parse JSON string: {}", e))?;

        // 使用From trait将Value转换为BatchCreateRecordsRequest
        let request = BatchCreateRecordsRequest::from(value);

        if request.records.is_empty() {
            return Err(anyhow!("No valid records found in the provided JSON"));
        }

        self.batch_create_records(app_token, table_id, request)
            .await
    }

    pub async fn get_fields_list(
        &self,
        app_token: &str,
        table_id: &str,
    ) -> Result<FieldsListResponse> {
        if app_token.is_empty() || table_id.is_empty() {
            return Err(anyhow!("app_token and table_id cannot be empty"));
        }

        let token = self.token_manager.get_token().await?;

        let url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}/tables/{}/fields",
            app_token, table_id
        );

        let response = Client::new()
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json; charset=utf-8")
            .send()
            .await
            .map_err(|e| anyhow!(e).context("Failed to send request for getting fields list"))?
            .json::<LarkApiResponse<FieldsListResponse>>()
            .await
            .map_err(|e| anyhow!(e).context("Failed to parse fields list response"))?;

        match response.is_success() {
            true => Ok(response.data),
            false => Err(anyhow!(BitableApiError::ApiError {
                code: response.code,
                message: response.msg.clone(),
            })
            .context(format!(
                "API returned error code: {} - {}",
                response.code, response.msg
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_records_list() {
        dotenvy::dotenv().ok();

        let client = BitableTableClient::new();

        let app_token = "xxxx";
        let table_id = "xxxx";
        let result = client.get_records_list(app_token, table_id).await;
        assert!(result.is_ok());

        println!("Result: {:#?}", result.unwrap());
    }

    #[tokio::test]
    async fn test_batch_create_records() {
        dotenvy::dotenv().ok();

        let client = BitableTableClient::new();

        let app_token = "xxxx";
        let table_id = "xxxx";

        let records_json = r#"[
            {
                "股票名称": "xxxx",
                "题材概念": "xxxx",
                "日期": 1743129600000,
                "梯队": ["xxxx"]
            }
        ]"#;
        let result = client
            .batch_create_records_json(app_token, table_id, records_json)
            .await;
        assert!(result.is_ok());

        println!("Result: {:#?}", result.unwrap());
    }

    #[tokio::test]
    async fn test_get_fields_list() {
        dotenvy::dotenv().ok();

        let client = BitableTableClient::new();

        let app_token = "xxxx";
        let table_id = "xxxx";

        let result = client.get_fields_list(app_token, table_id).await;
        assert!(result.is_ok());

        let fields: Vec<crate::client::bitable::FieldInfo> = result.unwrap().into();
        println!("Simplified Field Infos: {:#?}", fields);
    }
}
