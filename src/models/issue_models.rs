use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub id: i32,
    pub workspace_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<i32>, // Use an enum or specific type for status
    pub priority: Option<i32>, // Use an enum or specific type for priority
    pub assignee_id: Option<i32>,
    pub created_by: Option<i32>,
    pub team_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub url_slug: Option<String>,
    pub created_at: Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
    pub updated_at:Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewIssue {
    pub workspace_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<i32>, // Use an enum or specific type for status
    pub priority: Option<i32>, // Use an enum or specific type for priority
    pub assignee_id: Option<i32>,
    // pub created_by: Option<i32>,
    pub team_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub url_slug: Option<String>,
    // pub created_at: Option<NaiveDateTime>,
    // pub updated_at: Option<NaiveDateTime>,
}