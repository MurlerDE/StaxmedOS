use axum::{routing::get, Router, Extension};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

mod modules;
mod config; // Wir lagern die Config-Logik aus
mod license; // Hier kommt der Heartbeat-Worker rein

use crate::config::{AppConfig, StaxMode};

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    // 1. Zentrale Konfiguration laden
    let config = Arc::new(AppConfig::from_env());
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 2. Datenbank-Pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // 3. Heartbeat starten (nur wenn im STAXBOX Modus)
    if let StaxMode::StaxBox = config.mode {
        let heartbeat_config = config.clone();
        tokio::spawn(async move {
            license::worker::run_heartbeat_loop(heartbeat_config).await;
        });
    }

    // 4. CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 5. API-Routing mit v1 Versionierung
    let api_routes = Router::new()
        .route("/healthy", get(health_check))
        .nest("/inventory", modules::inventory::router())
        .nest("/auth", modules::auth::router());
        .nest("/system", modules::system::router());

    // Den gesamten Router unter /api/v1 hängen
    let app = Router::new()
        .nest("/api/v1", api_routes)
        .layer(cors)
        .layer(Extension(pool))
        .layer(Extension(config)); // Config für alle Handler verfügbar machen

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("StaxmedOS Backend ({:?}) listening on http://{}", config.mode, addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "StaxmedOS API v1 - Online"
}
