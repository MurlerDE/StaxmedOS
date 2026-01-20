use axum::{Extension, Json, http::StatusCode};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Deserialize)]
pub struct HeartbeatRequest {
    pub box_id: String,      // Eindeutige ID der StaxBox
    pub license_key: String, // Optionaler Sicherheits-Key
}

#[derive(Serialize)]
pub struct HeartbeatResponse {
    pub status: String,
    pub valid_until: Option<chrono::DateTime<Utc>>,
}

pub async fn receive_heartbeat(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<HeartbeatRequest>,
) -> Result<Json<HeartbeatResponse>, StatusCode> {
    
    // 1. Suche die Box in der Cloud-Datenbank
    // Wir schauen, ob die Box existiert und aktiv ist
    let box_record = sqlx::query!(
        r#"
        UPDATE staxbox_licenses 
        SET last_heartbeat = NOW() 
        WHERE hardware_id = $1 AND is_active = true
        RETURNING valid_until
        "#,
        payload.box_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 2. Falls die Box nicht existiert oder deaktiviert wurde -> 403 Forbidden
    let valid_until = match box_record {
        Some(record) => record.valid_until,
        None => return Err(StatusCode::FORBIDDEN),
    };

    // 3. Erfolg melden
    Ok(Json(HeartbeatResponse {
        status: "verified".to_string(),
        valid_until,
    }))
}
