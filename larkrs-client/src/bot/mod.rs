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
        response
            .items
            .into_iter()
            .map(|chat| ChatInfoItem {
                chat_id: chat.chat_id,
                name: chat.name,
            })
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
    pub msg_type: String,
    pub receive_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextContent {
    pub text: String,
}

impl TextContent {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

/// Markdown content structure for Feishu messages
#[derive(Debug, Serialize, Deserialize)]
pub struct MarkdownContent {
    pub zh_cn: MarkdownLanguageContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkdownLanguageContent {
    pub title: String,
    pub content: Vec<Vec<MarkdownElement>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkdownElement {
    pub tag: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<Vec<String>>,
}

impl SendMessageRequest {
    pub fn text(receive_id: &str, content: &str) -> Self {
        let text_content = TextContent::new(content);
        Self {
            content: serde_json::to_string(&text_content).unwrap_or_default(),
            msg_type: "text".to_string(),
            receive_id: receive_id.to_string(),
        }
    }

    /// Create a markdown message request
    pub fn markdown(receive_id: &str, title: &str, elements: Vec<Vec<MarkdownElement>>) -> Self {
        let markdown_content = MarkdownContent {
            zh_cn: MarkdownLanguageContent {
                title: title.to_string(),
                content: elements,
            },
        };

        Self {
            content: serde_json::to_string(&markdown_content).unwrap_or_default(),
            msg_type: "post".to_string(),
            receive_id: receive_id.to_string(),
        }
    }
}
