mod db;

use axum::http::{self, HeaderValue, Method};
use axum::response::IntoResponse;
use axum::routing::get;
use axum_prometheus::PrometheusMetricLayer;
use middlewares::auth_middleware::auth_middleware;
use middlewares::logger_middleware::logger_middleware;
// use middlewares::rate_limit_middleware::RateLimitStore;
// use axum::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::System;
use tokio::net::TcpListener;

use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::{middleware, Router};

use sqlx::{Pool, Postgres};

use tower_http::cors::CorsLayer;

mod configs;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod services;
mod utils;

// use routes::barbershop_routes;
use routes::{
    auth_routes::auth_routes, merchant_routes::merchant_routes, payment_routes::payment_routes,
    product_routes::product_routes, register_routes::register_routes,
    transaction_routes::transaction_routes, user_routes::user_routes, wallet_routes::wallet_routes,
};

pub struct AppState {
    pub pool: Arc<Pool<Postgres>>,
}

fn app_routes(pool: Arc<Pool<Postgres>>) -> Router {
    // !TODO: use app state to get rid of cloning

    // let rate_limit_store = RateLimitStore::new();

    let _cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_credentials(true)
        // allow requests from any origin
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        // .allow_origin(Any)
        .allow_origin("http://localhost:3003".parse::<HeaderValue>().unwrap());

    //prometheus metrics and monitoring
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    Router::new()
        .nest("/api/user", user_routes(pool.clone()))
        .nest("/api/wallet", wallet_routes(pool.clone()))
        .nest("/api/transactions", transaction_routes(pool.clone()))
        .nest("/api/merchant", merchant_routes(pool.clone()))
        .nest("/api/product", product_routes(pool.clone()))
        .nest("/api/payment", payment_routes(pool.clone()))
        .route_layer(middleware::from_fn(auth_middleware))
        .nest("/api/auth", auth_routes(pool.clone()))
        .nest("/api/register", register_routes(pool.clone()))
        .route_layer(middleware::from_fn(logger_middleware))
        .fallback(axum::routing::get(|| async {
            (http::StatusCode::NOT_FOUND, "Not Found")
        }))
        .layer(CorsLayer::permissive())
        // .layer(BufferLayer::new(1024))
        .route("/fast", get(|| async {}))
        .route(
            "/slow",
            get(|| async {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }),
        )
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .route("/system_metrics", get(collect_system_metrics))
        .layer(prometheus_layer)
        .with_state(pool)
}

#[tokio::main]
async fn main() {
    // let pool = create_db_pool().await;

    dotenvy::dotenv().expect("Unable to access .env file");

    let _server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env file");

    let pool = PgPoolOptions::new()
        .max_connections(64)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("can't connect to database");

    let cloned_db_pool = pool.clone();
    let app = app_routes(cloned_db_pool.into());

    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("0.0.0.0:3090".to_owned());
    let listener = TcpListener::bind(server_address)
        .await
        .expect("Could not create tcp listener");

    println!("🚀 Server started successfully");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    axum::serve(listener, app).await.unwrap();
}

// Function to collect system metrics
async fn collect_system_metrics() -> impl IntoResponse {
    // Create a new system object
    let system = System::new();

    // Collect system metrics
    let cpu_usage = system.global_cpu_info().cpu_usage();
    let memory_usage = system.used_memory();
    let swap_usage = system.used_swap();

    // Format metrics in Prometheus exposition format
    let prometheus_metrics = format!(
        "system_cpu_usage {}\nsystem_memory_usage {}\nsystem_swap_usage {}",
        cpu_usage, memory_usage, swap_usage
    );

    // Return metrics as a HTTP response
    http::Response::builder()
        .status(http::StatusCode::OK)
        .header("Content-Type", "text/plain")
        .body(prometheus_metrics)
        .unwrap()
}
