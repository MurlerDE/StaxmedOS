use axum::{
    Json,
    Router,
    routing::post,
    Extension,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode}};

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub username: String,
    pub role: String,
}

pub struct AdminUser {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
}

// Hilfs-Struktur für die Datenbank-Abfrage
struct UserRecord {
    id: Uuid,
    password_hash: Option<String>,
    role: Option<String>,
    tenant_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    tenant_id: String,
    role: String,
    exp: usize,
}

impl<S> FromRequestParts<S> for AdminUser
where S: Send + Sync {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1. Token aus Header extrahieren
        let auth_header = parts.headers.get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Kein Token gefunden".into()))?;

        // 2. Token validieren
        let token = auth_header.strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Ungültiges Token-Format".into()))?;

        // 3. JWT dekodieren (Secret muss sicher gelagert werden!)
        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret("DEIN_GEHEIMES_JWT_SECRET".as_ref()),
            &jsonwebtoken::Validation::default(),
        ).map_err(|_| (StatusCode::UNAUTHORIZED, "Ungültiger Token".into()))?;

        // 4. Rolle prüfen
        if token_data.claims.role != "admin" {
            return Err((StatusCode::FORBIDDEN, "Admin-Rechte erforderlich".into()));
        }

        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Invalid User ID".into()))?;
        let tenant_id = Uuid::parse_str(&token_data.claims.tenant_id)
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Invalid Tenant ID".into()))?;

        Ok(AdminUser { user_id, tenant_id })
    }
}

pub fn router() -> Router {
    Router::new().route("/login", post(login_handler))
}

async fn login_handler(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (axum::http::StatusCode, String)> {

    // 1. User in DB suchen - query_as! nutzt die UserRecord Struktur
    let user = sqlx::query_as!(
        UserRecord,
        "SELECT id, password_hash, role, tenant_id FROM users WHERE username = $1 AND is_active = true",
        payload.username
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((axum::http::StatusCode::UNAUTHORIZED, "Ungültige Anmeldedaten".into()))?;

    // 2. Token erstellen (Passwort-Prüfung kommt im nächsten Schritt)
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        tenant_id: user.tenant_id.unwrap_or_default().to_string(),
        role: user.role.unwrap_or_else(|| "user".into()),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("DEIN_GEHEIMES_JWT_SECRET".as_ref()),
    ).map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Token-Fehler".into()))?;

    Ok(Json(AuthResponse {
        token,
        username: payload.username,
        role: claims.role,
    }))
}
