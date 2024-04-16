// use std::sync::Arc;
// use axum::extract::Path;
// use axum::{Json, Router};
// use axum::routing::{delete, get, post};
// use sqlx::PgPool;
// use crate::handlers::issue_handlers::{create_issue, get_issues, get_issues_by_workspace_id, get_my_issues_all, get_my_issues_created, get_issues_by_workspace_slug};
// use crate::handlers::workspace_handlers::create_workspace;
// use crate::models::issue_models::NewIssue;
// use crate::models::workspace_models::NewWorkspace;


// pub fn issue_routes(pool: Arc<PgPool>) -> Router {
//     let get_pool = Arc::clone(&pool);
//     let get_slug_pool = Arc::clone(&pool);
//     let get_user_pool = Arc::clone(&pool);
//     let get_my_issues_pool = Arc::clone(&pool);
//     let create_pool = Arc::clone(&pool);
//     let get_my_issues_created_pool = Arc::clone(&pool);
//     let delete_pool = Arc::clone(&pool);
//     let get_issues_by_workspace_id_pool = Arc::clone(&pool);




//     Router::new()
//         .route("/", get( move || {get_issues(get_pool)  }))
//         .route("/:user_id", get( move |path: Path<i32>,| {
//             get_issues_by_workspace_id(path, get_user_pool)
//         }))
//         .route("/:username/my_issues/all", get( move |path: Path<String>,| {
//             get_my_issues_all(path, get_my_issues_pool)
//         }))
//         .route("/:username/my_issues/created", get( move |path: Path<String>| {
//             get_my_issues_created(path, get_my_issues_created_pool)
//         }))
//         .route("/:username/create", post(move | path: Path<String>, Json(new_issue): Json<NewIssue>| {
//             create_issue(path, Json(new_issue), create_pool)
//         }))
//         .route("/workspaces/:workspace_slug", get( move |path: Path<String>| {
//             get_issues_by_workspace_slug(path, get_issues_by_workspace_id_pool)
//         }))

//         // .route("/:slug", get( move |path: Path<String>,| {
//         //     get_workspaces_by_slug(path, get_slug_pool)
//         // }))
//         // .route("/:user_id/create", post(move | path: Path<i32>, Json(new_workspace): Json<NewWorkspace>| {
//         //     create_workspace(path, Json(new_workspace), create_pool)
//         // }))
//         // .route("/:id/delete", delete(move |path: Path<i32>| {
//         //     delete_workspace(path, delete_pool)
//         // }))
// }