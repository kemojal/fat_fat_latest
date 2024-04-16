use sqlx::PgPool;
use std::env;

pub async fn create_db_pool() -> PgPool {
    dotenv::dotenv().ok();

    let db_host = env::var("DB_HOST").expect("DB_HOST not set");
    let db_port = env::var("DB_PORT").expect("DB_PORT not set");
    let db_name = env::var("DB_NAME").expect("DB_NAME not set");
    let db_user = env::var("DB_USER").expect("DB_USER not set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD not set");

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );

    PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool")
}
