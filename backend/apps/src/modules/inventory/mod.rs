use axum::{extract::Path, routing::get, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct GlobalProduct {
    pub udi_di: String,
    pub brand_name_de: Option<String>,
    pub manufacturer: Option<String>,
}

pub fn router() -> Router {
    Router::new().route("/product/{udi}", get(get_product_by_udi))
}

async fn get_product_by_udi(
    Path(udi): Path<String>, // Der Name hier muss mit dem Namen in {udi} übereinstimmen
    Extension(pool): Extension<PgPool>,
) -> Result<Json<GlobalProduct>, (axum::http::StatusCode, String)> {
    let product = sqlx::query_as!(
        GlobalProduct,
        "SELECT udi_di, brand_name_de, manufacturer FROM global_products WHERE udi_di = $1",
        udi
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match product {
        Some(p) => Ok(Json(p)),
        None => Err((axum::http::StatusCode::NOT_FOUND, "Produkt nicht gefunden".into())),
    }
}
