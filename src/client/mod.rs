use serde::{Deserialize, Serialize};

pub mod auth;
pub mod bitable;


#[derive(Debug, Serialize, Deserialize)]
pub struct LarkApiResponse<T> {
    pub code: i32,
    pub msg: String,
    #[serde(default)]
    pub data: T,
}

impl<T> LarkApiResponse<T> {
    pub fn is_success(&self) -> bool {
        self.code == 0
    }
}