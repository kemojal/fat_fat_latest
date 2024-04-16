use chrono::{ NaiveDateTime};

use serde::{Serialize, Deserialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub id: i32,
    pub user_id: Option<i32>,
    pub name: Option<String>,
    pub url_slug: Option<String>,
    pub created_at: Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
    pub updated_at: Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewWorkspace {
    // pub user_id: i32,
    pub name: Option<String>,
    pub url_slug: Option<String>,
    // pub created_at: Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
    // pub updated_at: Option<NaiveDateTime>, // Change to DateTime type based on your needs (e.g., chrono)
}


#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceId {
    pub id: i32,
}