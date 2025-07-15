use serde::{Deserialize, Serialize};

pub mod client;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WikiNodeResponse {
    pub node_token: String,
    pub node_type: String,
    pub parent_node_token: String,
    pub space_id: String,
    pub title: String,
    pub obj_token: String,
    pub obj_type: String,
    pub has_child: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WikiContentResponse {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WikiSpaceResponse {
    pub space_id: String,
    pub name: String,
    pub description: Option<String>,
    pub space_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct WikiListResponse {
    pub items: Vec<WikiNodeResponse>,
    pub page_token: Option<String>,
    pub has_more: bool,
}