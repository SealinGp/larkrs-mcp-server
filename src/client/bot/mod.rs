use serde::{Deserialize, Serialize};

pub mod chat;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ChatListResponse {
    pub items: Vec<ChatInfo>,
    #[serde(default)]
    pub page_token: Option<String>,
    #[serde(default)]
    pub has_more: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatInfo {
    pub chat_id: String,
    pub name: String,
    pub avatar: Option<String>,
    pub description: Option<String>,
    pub owner_id: Option<String>,
    pub owner_id_type: Option<String>,
    #[serde(default)]
    pub chat_mode: Option<String>,
    #[serde(default)]
    pub chat_type: Option<String>,
    #[serde(default)]
    pub external: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatInfoItem {
    pub chat_id: String,
    pub name: String,
}

impl From<ChatListResponse> for Vec<ChatInfoItem> {
    fn from(response: ChatListResponse) -> Self {
        response.items
            .into_iter()
            .map(|chat| ChatInfoItem {
                chat_id: chat.chat_id,
                name: chat.name,
            })
            .collect()
    }
}
