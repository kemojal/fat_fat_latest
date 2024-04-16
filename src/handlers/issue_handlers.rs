// use std::sync::Arc;
// use axum::extract::Path;
// use axum::response::{Json, IntoResponse, Response};
// use chrono::expect;
// use serde_json::json;
// use sqlx::{pool, query, query_as, PgPool};
// use crate::models::issue_models::{Issue, NewIssue};
// use crate::models::user_models::UserId;
// use crate::models::workspace_models::{NewWorkspace, Workspace, WorkspaceId};

// pub async fn get_issues(pool: Arc<PgPool>) -> impl IntoResponse {
//     let issues: Vec<Issue> = query_as!(
//         Issue,
//         r#"
//         SELECT id, workspace_id, name, description, status, priority, assignee_id, team_id, parent_id, url_slug, created_by, created_at, updated_at
//         FROM issues
//         "#
//     )
//         .fetch_all(&*pool)
//         .await
//         .expect("Failed to fetch issues");

//     Json(issues)
// }



// // https://linear.app/kemo/my-issues/assigned
// //https://linear.app/kemo/my-issues/created
// //https://linear.app/kemo/my-issues/subscribed
// //https://linear.app/kemo/my-issues/activity

// //https://linear.app/bantaba/team/RAI/all

// pub async fn get_issues_by_workspace_id(
//     Path(workspace_id): Path<i32>,
//     pool: Arc<PgPool>) -> impl IntoResponse {
//     let issues: Vec<Issue> = query_as!(
//         Issue,
//         "
//         SELECT id, workspace_id, name, description, status, priority, assignee_id, team_id,  parent_id, url_slug, created_by, created_at, updated_at
//         FROM issues
//         WHERE workspace_id = $1
//         ",
//         workspace_id
//     )
//         .fetch_all(&*pool)
//         .await
//         .expect("Failed to fetch workspaces");

//     Json(issues)
// }


// pub async fn get_my_issues_all(
//     Path(username): Path<String>,
//     pool: Arc<PgPool>) -> impl IntoResponse {

//     let user_id: Vec<UserId> = query_as!(
//         UserId,
//         "
//         SELECT id
//         FROM users
//         WHERE username = $1
//         ",
//         username
//     )
//         .fetch_all(&*pool)
//         .await
//         .expect("Failed to fetch user");

//     if let Some(first_user_id) = user_id.get(0){

//         let issues: Vec<Issue> = query_as!(
//         Issue,
//         "
//         SELECT id, workspace_id, name, description, status, priority, assignee_id, team_id, parent_id, url_slug, created_by, created_at, updated_at
//         FROM issues
//         WHERE assignee_id = $1
//         ",
//          first_user_id.id
//     )
//             .fetch_all(&*pool)
//             .await
//             .expect("Failed to fetch workspaces");

//         return Json(issues);

//     }

//     Json(Vec::<Issue>::new())


// }


// pub async fn get_my_issues_created(
//     Path(username): Path<String>,
//     pool: Arc<PgPool>) -> impl IntoResponse {

//     let user_id: Vec<UserId> = query_as!(
//         UserId,
//         "
//         SELECT id
//         FROM users
//         WHERE username = $1
//         ",
//         username
//     )
//         .fetch_all(&*pool)
//         .await
//         .expect("Failed to fetch user");

//     if let Some(first_user_id) = user_id.get(0){

//         let issues: Vec<Issue> = query_as!(
//         Issue,
//         "
//         SELECT id, workspace_id, name, description, status, priority, assignee_id, team_id,  parent_id, url_slug, created_by, created_at, updated_at
//         FROM issues
//         WHERE assignee_id = $1 AND created_by = $2
//         ",
//          first_user_id.id,
//         first_user_id.id
//     )
//             .fetch_all(&*pool)
//             .await
//             .expect("Failed to fetch workspaces");

//         return Json(issues);

//     }

//     Json(Vec::<Issue>::new())


// }

// pub async fn get_issues_by_workspace_slug(Path(workspace_slug): Path<String>, pool: Arc<PgPool>) -> impl IntoResponse {
//     // get the workspace id from the workspaces 
//     // if workspace does not exist, return empty
//     // else get all the issues with that workspace_id

//     if let Some(workspace) = sqlx::query_as!(
//         WorkspaceId,
//         "SELECT id FROM workspaces WHERE url_slug = $1",
//         workspace_slug
//     )
//     .fetch_optional(&*pool)
//     .await
//     .expect("Failed to fetch workspace")
//     {
//         let issues = query_as!(
//             Issue,
//             "SELECT * FROM issues WHERE workspace_id = $1",
//             workspace.id
//         )
//         .fetch_all(&*pool)
//         .await
//         .expect("Failed to fetch issues");

//         // Return the issues along with the success message
//         return Json(json!({
//             "status": "success",
//             "message": "This workspace slug exists",
//             "issues": issues,
//         }));
//     } else {
//         // Workspace does not exist
//         return Json(json!({
//             "status": "error",
//             "message": "This workspace slug does not exist"
//         }));
//     }
// }


// pub async fn create_issue(
//     Path(username): Path<String>,
//     Json(new_issue): Json<NewIssue>,
//     pool: Arc<PgPool>) -> impl IntoResponse {



//     let user_id: Vec<UserId> = query_as!(
//         UserId,
//         "
//         SELECT id
//         FROM users
//         WHERE username = $1
//         ",
//         username
//     )
//         .fetch_all(&*pool)
//         .await
//         .expect("Failed to fetch user");

//     if let Some(first_user_id) = user_id.get(0){

//         let result = query!(
//         "
//         INSERT INTO issues (workspace_id, name, description, status, priority, assignee_id,created_by, team_id,  parent_id, url_slug)
//         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
//         RETURNING *
//         ",
//         new_issue.workspace_id,
//         new_issue.name,
//         new_issue.description,
//         new_issue.status,
//         new_issue.priority,
//         new_issue.assignee_id,
//        first_user_id.id,
//         new_issue.team_id,
//         new_issue.parent_id,
//         new_issue.url_slug
//     )
//             .fetch_one(&*pool)
//             .await;

//         match result {
//             Ok(row) => {
//                 let new_id = row.id;
//                return  Json(json!({
//                 "status": "success",
//                 "message": "Workspace added successfully",
//                 "new_id": new_id
//             }))
//             }
//             Err(e) => {
//                 println!("Error inserting into database: {:?}", e);
//                 // Handle error case
//                 // You can return an error response or customize it as needed
//                 // For now, let's return a generic error response
//                 return Json(json!({
//                 "status": "error",
//                 "message": format!("Failed to create a workspace: {:?}", e)
//             }))
//             }
//         }

//     }
//     Json(json!([]))
// }
// //
// // pub async fn delete_issue(issue_id: i32, pool: &PgPool) -> impl IntoResponse {
// //     let result = query!(
// //         "DELETE FROM issues WHERE id = $1",
// //         issue_id
// //     )
// //         .execute(pool)
// //         .await;
// //
// //     if result.is_ok() {
// //         let success_json = json!({
// //             "status": "success",
// //             "message": format!("Issue with ID {} deleted", issue_id),
// //         });
// //         Ok(Response::json(&success_json))
// //     } else {
// //         let error_json = json!({
// //             "status": "error",
// //             "message": format!("Failed to delete the issue with ID {}", issue_id),
// //         });
// //         Ok(Response::json(&error_json).status(StatusCode::InternalServerError))
// //     }
// // }