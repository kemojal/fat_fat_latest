use std::sync::Arc;
use axum::extract::{Path};
use axum::response::{Json, IntoResponse};


use serde_json::json;
use sqlx::{query, query_as, PgPool};

use crate::models::user_models::UserId;
use crate::models::wallet_models::{NewWallet, Wallet};


pub async fn get_wallets(pool: Arc<PgPool>) -> impl IntoResponse {
    let wallets: Vec<Wallet> = query_as!(
        Wallet,
        r#"
        SELECT id, user_id, balance, currency, created_at, updated_at
        FROM wallets
        "#
    )
        .fetch_all(&*pool)
        .await
        .expect("Failed to fetch issues");

    Json(wallets                                                                                          )
}



pub async fn get_wallets_by_user_id(
    Path(user_id): Path<i32>,
    pool: Arc<PgPool>) -> impl IntoResponse {
    let issues: Vec<Wallet> = query_as!(
        Wallet,
        "
        SELECT id, user_id, balance, currency, created_at, updated_at
        FROM wallets
        WHERE user_id = $1
        ",
        user_id
    )
        .fetch_all(&*pool)
        .await
        .expect("Failed to fetch workspaces");

    Json(issues)
}


// pub async fn get_my_wallets_all(
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


// pub async fn get_my_wallet_created(
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

// pub async fn get_wallets_by_workspace_slug(Path(workspace_slug): Path<String>, pool: Arc<PgPool>) -> impl IntoResponse {
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


pub async fn create_wallet(
    Path(username): Path<String>,
    Json(new_wallet): Json<NewWallet>,
    pool: Arc<PgPool>) -> impl IntoResponse {


        let balance = new_wallet.balance;



    let user_id: Vec<UserId> = query_as!(
        UserId,
        "
        SELECT id
        FROM users
        WHERE username = $1
        ",
        username
    )
        .fetch_all(&*pool)
        .await
        .expect("Failed to fetch user");

    if let Some(first_user_id) = user_id.get(0){
        let user_id_value: i32 = first_user_id.id; 

        let result = query!(
        "
        INSERT INTO wallets (user_id, balance)
        VALUES ($1, $2)
        RETURNING *",
        user_id_value,
        balance
    )
            .fetch_one(&*pool)
            .await;

        match result {
            Ok(row) => {
                let new_id = row.id;
               return  Json(json!({
                "status": "success",
                "message": "Workspace added successfully",
                "new_id": new_id
            }))
            }
            Err(e) => {
                println!("Error inserting into database: {:?}", e);
                // Handle error case
                // You can return an error response or customize it as needed
                // For now, let's return a generic error response
                return Json(json!({
                "status": "error",
                "message": format!("Failed to create a workspace: {:?}", e)
            }))
            }
        }

    }
    Json(json!([]))
}


// pub async fn deposit(
//     Path(id): Path<i32>,
//     edit_user_data: Json<DepositAmount>,
//     pool: Arc<PgPool>
// ) -> impl IntoResponse {

//     // let first_name = edit_user_data.first_name.clone();
//     // let last_name = edit_user_data.last_name.clone();
//     let email = edit_user_data.email.clone();

//     // let completed = edit_todo_data.completed;

//     // UPDATE users
//     // SET first_name = coalesce($2, first_name),
//     // last_name = coalesce($3, last_name),
//     // email = coalesce($4, email)
//     // WHERE id = $1
//     // RETURNING *


//     let update_result = query!(
//         "
//         UPDATE users
//         SET email = coalesce($2, email)
//         WHERE id = $1
//         RETURNING *",
//         id,
//     email)
//         .fetch_one(&*pool)
//         .await;

//     if update_result.is_ok() {
//         // Return a success response
//         Json(json!({
//             "status": "success",
//             "message": format!("User with ID {} edited", id)
//         }))
//     } else {
//         // Return an error response
//         Json(json!({
//             "status": "error",
//             "message": "Failed to update user"
//         }))
//     }


// }

// pub async fn withdraw(
//     Path(user_id): Path<i32>,
//     // Json(new_wallet): Json<NewWallet>,
//     pool: Arc<PgPool>) -> impl IntoResponse {


//         // let balance = new_wallet.balance;



//     let user_id: Vec<UserId> = query_as!(
//         UserId,
//         "
//         SELECT id
//         FROM users
//         WHERE id = $1
//         ",
//         user_id
//     )
//         .fetch_all(&*pool)
//         .await
//         .expect("Failed to fetch user");

//     if let Some(first_user_id) = user_id.get(0){
//         let user_id_value: i32 = first_user_id.id; 

//         let result = query!(
//         "
//         INSERT INTO wallets (user_id)
//         VALUES ($1)
//         RETURNING *",
//         user_id_value
        
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
//
pub async fn delete_wallet(
    Path(wallet_id): Path<i32>, 
    pool: Arc<PgPool>,
    
) -> impl IntoResponse {
    // Use the id to delete the item from the database
    let delete_result = query!(
        "DELETE FROM users WHERE id = $1",
        wallet_id
    )
        .execute(&*pool)
        .await;

    if delete_result.is_ok() {
        // Return a success response
        Json(json!({
            "status": "success",
            "message": format!("User with ID {} deleted", wallet_id)
        }))
    } else {
        // Return an error response
        Json(json!({
            "status": "error",
            "message": format!("Failed to delete user with ID {}", wallet_id)
        }))
    }
}



//helpers
// async fn check_user_authorization(
//     wallet_id: i32,
//     user_id: UserId,
//     pool: &PgPool,
// ) -> bool {
//     let query_result = query!(
//         "SELECT COUNT(*) FROM wallets WHERE id = $1 AND user_id = $2",
//         wallet_id,
//         user_id.
//     )
//     .fetch_one(pool)
//     .await;

//     match query_result {
//         Ok(row) => row.get(0),
//         Err(_) => false,
//     }
// }