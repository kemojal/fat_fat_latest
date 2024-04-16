// use std::ffi::CString;
// use std::sync::Arc;
// use axum::{Json, response::IntoResponse};
// use axum::extract::Path;
// use axum::http::StatusCode;
// use serde_json::json;
// use sqlx::{PgPool, query, query_as};



// use crate::models::workspace_models::{Workspace, NewWorkspace};




// pub async fn get_workspaces( pool: Arc<PgPool>) -> impl IntoResponse {
//     let workspaces: Vec<Workspace> = query_as!(
//         Workspace,
//         r#"
//         SELECT id, user_id, name, url_slug, created_at, updated_at FROM workspaces
//         "#
//     )
//         .fetch_all(&*pool)
//         .await
//         .expect("Failed to fetch workspaces");

//     Json(workspaces)
// }

// pub async fn get_workspaces_by_slug(
//     Path(slug): Path<String>,
//     pool: Arc<PgPool>) -> impl IntoResponse {
//     let workspaces: Vec<Workspace> = query_as!(
//         Workspace,
//         "
//         SELECT id, user_id, name, url_slug, created_at, updated_at FROM workspaces WHERE url_slug = $1
//         ", slug
//     )
//         .fetch_all(&*pool)
//         .await
//         .expect("Failed to fetch workspaces");

//     Json(workspaces)
// }

// pub async fn get_workspaces_by_user_id(
//     Path(user_id): Path<i32>,
//     pool: Arc<PgPool>) -> impl IntoResponse {

//     let workspaces: Vec<Workspace> = query_as!(
//         Workspace,
//         "
//         SELECT id, user_id, name, url_slug, created_at, updated_at FROM workspaces WHERE user_id = $1
//         ", user_id
//     )
//         .fetch_all(&*pool)
//         .await
//         .expect("Failed to fetch workspaces");

//     Json(workspaces)
// }

// pub async fn create_workspace(
//     Path(user_id): Path<i32>,
//     Json(new_workspace): Json<NewWorkspace>, pool: Arc<PgPool>) -> impl IntoResponse {
//     // let user_id = new_workspace.user_id;
//     let user_id = user_id;
//     let name = new_workspace.name;
//     let url_slug = new_workspace.url_slug;
//     // WHERE user_id = $1 AND name = $2 AND url_slug = $3
//     let workspace_exists = query!(
//         "SELECT EXISTS (
//             SELECT 1 FROM workspaces
//             WHERE user_id = $1 AND name = $2 AND url_slug = $3
//         )",
//         user_id,
//         name,
//         url_slug
//     )
//         .fetch_one(&*pool)
//         .await
//         .expect("Failed to check if workspace exists");

//     if let Some(true) = workspace_exists.exists {
//         // Workspace with the same name, url_slug, and user_id exists
//         return Json(json!({
//             "status": "already_exist",
//             "message": "Workspace already exists, please use a different workspace name"
//         }));
//     }


//     // let created_at = new_workspace.created_at;
//     // let updated_at = new_workspace.updated_at;
//     let query_result = query!(

//         "
//         INSERT INTO workspaces (user_id, name, url_slug)
//         VALUES ($1, $2, $3)
//         RETURNING *",
//         user_id,
//         name,
//         url_slug,
//         // created_at,
//         // updated_at

//     )
//         .fetch_one(&*pool)
//         .await;

//     match query_result {
//         Ok(row) => {
//             let new_id = row.id;
//             Json(json!({
//                 "status": "success",
//                 "message": "Workspace added successfully",
//                 "new_id": new_id
//             }))
//         }
//         Err(_) => {
//             // Handle error case
//             // You can return an error response or customize it as needed
//             // For now, let's return a generic error response
//             Json(json!({
//                 "status": "error",
//                 "message": "Failed to create a workspace"
//             }))
//         }
//     }
// }

// pub async fn delete_workspace(Path(id): Path<i32>,
//                                 pool: Arc<PgPool>) -> impl IntoResponse {
//     // Use the id to delete the item from the database
//     let delete_result = query!(
//     "DELETE FROM workspaces WHERE id = $1",
//     id
// )
//         .execute(&*pool)
//         .await;

//     if delete_result.is_ok() {
//         // Return a success response
//         Json(json!({
//         "status": "success",
//         "message": format!("workspace with ID {} deleted", id)
//     }))
//     } else {
//         // Return an error response
//         Json(json!({
//         "status": "error",
//         "message": format!("Failed to delete the workspace with ID {}", id)
//     }))
//     }
// }
