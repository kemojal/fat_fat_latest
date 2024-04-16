// // routes/appointment.rs

// use std::sync::Arc;

// use axum::{

//     routing::{get, post, put, delete},  Json, Router,  extract::Path,  http::Method

// };
// use sqlx::PgPool;

// use crate::handlers::workspace_handlers::{create_workspace, delete_workspace, get_workspaces, get_workspaces_by_slug, get_workspaces_by_user_id};
// use crate::models::workspace_models::{Workspace, NewWorkspace};

// pub fn workspace_routes(pool: Arc<PgPool>) -> Router {
//     let get_pool = Arc::clone(&pool);
//     let get_slug_pool = Arc::clone(&pool);
//     let get_user_pool = Arc::clone(&pool);
//     let create_pool = Arc::clone(&pool);
//     let delete_pool = Arc::clone(&pool);


//     Router::new()
//         .route("/", get( move || {get_workspaces(get_pool)  }))
//         .route("/user/:user_id", get( move |path: Path<i32>,| {
//             get_workspaces_by_user_id(path, get_user_pool)
//         }))
//         .route("/:slug", get( move |path: Path<String>,| {
//             get_workspaces_by_slug(path, get_slug_pool)
//         }))
//         .route("/:user_id/create", post(move | path: Path<i32>, Json(new_workspace): Json<NewWorkspace>| {
//             create_workspace(path, Json(new_workspace), create_pool)
//         }))
//         .route("/:id/delete", delete(move |path: Path<i32>| {
//             delete_workspace(path, delete_pool)
//         }))
// }

