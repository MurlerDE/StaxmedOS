use axum::{routing::get, Router, Extension};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use dotenvy::dotenv;
use std::env;

mod modules; // Wir erstellen gleich den Ordner 'modules'

#[tokio::main]
async fn main() {
    // 1. .env Datei laden
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 2. Datenbank-Pool erstellen
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // 3. CORS Konfiguration (wie gehabt)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 4. Router zusammenbauen
    let app = Router::new()
        .route("/api/healthy", get(health_check))
            .nest("/api/inventory", modules::inventory::router())
            .nest("/api/auth", modules::auth::router()) // Diese Zeile hinzufügen!
            .layer(cors)
            .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("StaxmedOS Backend listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "Backend & Database connected"
}
