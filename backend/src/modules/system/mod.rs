use axum::{routing::post, Router};
mod handlers;

pub fn router() -> Router {
    Router::new()
        .route("/heartbeat", post(handlers::receive_heartbeat))
}
