use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: Uuid,
    pub is_private: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub language: Option<String>,
    pub stars_count: i32,
    pub forks_count: i32,
    pub topics: Vec<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ListQuery {
    pub q: Option<String>,
    #[serde(rename = "type")]
    pub repo_type: Option<String>,
    pub language: Option<String>,
    pub sort: Option<String>,
    pub per_page: Option<i32>,
    pub page: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRepository {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub language: Option<String>,
    pub topics: Vec<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateRepository {
    pub description: Option<String>,
    pub is_private: Option<bool>,
    pub language: Option<String>,
    pub topics: Option<Vec<String>>,
}