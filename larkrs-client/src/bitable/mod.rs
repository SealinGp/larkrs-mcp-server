pub mod table;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SearchRecordsResponse {
    pub items: Vec<Record>,
    pub page_token: Option<String>,
    pub has_more: bool,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub record_id: String,
    pub fields: HashMap<String, Value>,
    #[serde(default)]
    pub created_by: Option<UserId>,
    #[serde(default)]
    pub created_time: Option<i64>,
    #[serde(default)]
    pub last_modified_by: Option<UserId>,
    #[serde(default)]
    pub last_modified_time: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId {
    pub user_id: Option<String>,
    pub open_id: Option<String>,
    pub union_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SearchRecordsCond {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<Sort>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_names: Option<Vec<String>>,
    pub view_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_fields: Option<bool>,
}

/// Enum for filter conjunction types
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FilterConjunction {
    And,
    Or,
}

impl Default for FilterConjunction {
    fn default() -> Self {
        FilterConjunction::And
    }
}

/// Enum for filter operator types
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum FilterOperator {
    Is,
    IsNot,
    Contains,
    DoesNotContain,
    IsEmpty,
    IsNotEmpty,
    IsGreater,
    IsGreaterEqual,
    IsLess,
    IsLessEqual,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    pub conditions: Vec<FilterCondition>,
    #[serde(default)]
    pub conjunction: FilterConjunction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterCondition {
    pub field_name: String,
    pub operator: FilterOperator,
    pub value: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sort {
    pub field_name: String,
    #[serde(default)]
    pub desc: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SortKey {
    pub field_name: String,
    #[serde(rename = "order")]
    pub sort_order: SortOrder,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchCreateRecordsRequest {
    pub records: Vec<RecordCreate>,
}

impl From<Value> for BatchCreateRecordsRequest {
    fn from(value: Value) -> Self {
        match value {
            Value::Array(array) => {
                let records = array
                    .iter()
                    .filter_map(|item| {
                        item.as_object().map(|obj| RecordCreate {
                            fields: obj.clone().into_iter().collect(),
                        })
                    })
                    .collect();
                Self { records }
            }
            _ => Self { records: vec![] },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordCreate {
    pub fields: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FieldsListResponse {
    pub items: Vec<Field>,
    #[serde(default)]
    pub page_token: String,
    #[serde(default)]
    pub has_more: bool,
    #[serde(default)]
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub field_name: String,
    pub field_id: String,
    #[serde(rename = "type")]
    pub field_type: i32,
    #[serde(default)]
    pub property: Value,
    #[serde(rename = "ui_type", default)]
    pub ui_type: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub is_primary: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldInfo {
    pub field_name: String,          // 字段名称
    pub description: Option<String>, // 字段描述
    pub is_primary: Option<bool>,    // 是否为主键
    pub ui_type: Option<String>,     // UI类型
    pub write_type: Option<String>,  // 写入时传递的类型
}

impl From<FieldsListResponse> for Vec<FieldInfo> {
    fn from(response: FieldsListResponse) -> Self {
        response
            .items
            .into_iter()
            .map(|field| {
                let write_type = match field.ui_type.as_deref() {
                    Some("Text") => Some("String".to_string()),
                    Some("SingleSelect") => Some("String".to_string()),
                    Some("DateTime") => Some("Timestamp".to_string()),
                    Some("MultiSelect") => Some("Array<String>".to_string()),
                    _ => Some("String".to_string()),
                };

                FieldInfo {
                    field_name: field.field_name,
                    description: field.description,
                    is_primary: field.is_primary,
                    ui_type: field.ui_type,
                    write_type,
                }
            })
            .collect()
    }
}
