use poem_mcpserver::{McpServer, Tools, stdio::stdio, tool::Json};

use client::{
    bitable::{SearchRecordsResponse, table::BitableTableClient},
    bot::{ChatInfoItem, chat::ChatClient},
};

mod client;

struct LarkServer {}

#[Tools]
impl LarkServer {
    /// Get records list from a Bitable table
    ///
    /// input feishu url like: https://xxx.feishu.cn/base/{app_token}?table={table_id}&view={view_id}
    /// Args:
    ///     app_token: The app token of the Bitable app
    ///     table_id: The ID of the table
    ///
    /// Returns:
    ///     A JSON response containing the list of records
    async fn table_records_list(
        &self,
        app_token: String,
        table_id: String,
    ) -> Json<SearchRecordsResponse> {
        Json(
            BitableTableClient::new()
                .get_records_list(app_token.as_str(), table_id.as_str())
                .await
                .unwrap_or_default(),
        )
    }

    /// Batch create multiple records in a Bitable table
    ///
    /// input feishu url like: https://xxx.feishu.cn/base/{app_token}?table={table_id}&view={view_id}
    /// Args:
    ///     app_token: The app token of the Bitable app
    ///     table_id: The ID of the table
    ///     records_json: A JSON string containing an array of records to create
    ///
    /// records_json like:
    /// 如果字段类型是时间类型，需要传入时间戳，单位是毫秒; 日期得是当前上下文中的日期时间戳
    /// ```
    /// [
    ///     {"股票名称": "太阳电缆", "题材概念": "海洋经济", "日期": 1742956800000, "梯队": ["四板"]},
    ///     {"股票名称": "太阳电缆", "题材概念": "海洋经济", "日期": 1742956800000, "梯队": ["四板"]}
    /// ]
    /// ```
    ///
    /// Returns:
    ///     A JSON response containing the result of the batch create operation
    async fn create_table_records_json(
        &self,
        app_token: String,
        table_id: String,
        records_json: String,
    ) -> Json<()> {
        Json(
            BitableTableClient::new()
                .batch_create_records_json(
                    app_token.as_str(),
                    table_id.as_str(),
                    records_json.as_str(),
                )
                .await
                .unwrap_or_default(),
        )
    }

    /// Get simplified fields info from a Bitable table
    ///
    /// input feishu url like: https://xxx.feishu.cn/base/{app_token}?table={table_id}&view={view_id}
    /// Args:
    ///     app_token: The app token of the Bitable app
    ///     table_id: The ID of the table
    ///
    /// Returns:
    ///     A JSON array of simplified field information (field_name, description, is_primary, ui_type, write_type)
    async fn table_fields_info(
        &self,
        app_token: String,
        table_id: String,
    ) -> Json<Vec<client::bitable::FieldInfo>> {
        let fields_response = BitableTableClient::new()
            .get_fields_list(app_token.as_str(), table_id.as_str())
            .await
            .unwrap_or_default();

        Json(fields_response.into())
    }

    /// Get a list of chat groups
    ///
    /// Returns:
    ///     A JSON array of chat groups with chat_id and name
    async fn chat_group_list(&self) -> Json<Vec<ChatInfoItem>> {
        Json(
            ChatClient::new()
                .get_chat_group_list()
                .await
                .unwrap_or_default(),
        )
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    stdio(McpServer::new().tools(LarkServer {})).await
}
