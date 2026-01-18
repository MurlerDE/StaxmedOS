use axum::{
    Json,
    Router,
    routing::{get, post},
    Extension,
    http::StatusCode // Diese Zeile war wichtig!
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::modules::auth::AdminUser;

#[derive(Serialize, sqlx::FromRow)]
pub struct TenantListItem {
    pub id: Uuid,
    pub name: String,
    pub status: Option<String>,
    pub plan_name: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateTenantRequest {
    pub name: String,
    pub plan_id: Uuid,
}

pub fn router() -> Router {
    Router::new()
        .route("/tenants", get(get_tenants))
        .route("/tenants", post(create_tenant))
}

async fn get_tenants(
    _admin: AdminUser,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<TenantListItem>>, (StatusCode, String)> {
    let tenants = sqlx::query_as!(
        TenantListItem,
        r#"SELECT t.id, t.name, t.status, p.name as plan_name
           FROM tenants t
           LEFT JOIN plans p ON t.plan_id = p.id"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(tenants))
}

async fn create_tenant(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateTenantRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query!(
        "INSERT INTO tenants (name, plan_id, status) VALUES ($1, $2, 'active')",
        payload.name,
        payload.plan_id
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::CREATED)
}
