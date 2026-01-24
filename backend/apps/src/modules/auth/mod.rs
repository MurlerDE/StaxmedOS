//! # Authentication Module
//!
//! ## Rollen-Hierarchie:
//!
//! - **staxadmin**: Super Admin (StaxMed Mitarbeiter)
//!   - Zugriff auf ALLE Tenants
//!   - Kann System-weite Einstellungen vornehmen
//!   - Tenant: '00000000-0000-0000-0000-000000000000'
//!
//! - **admin**: Tenant Admin (Kunden-Admin)
//!   - Zugriff nur auf eigenen Tenant
//!   - Kann User innerhalb des Tenants verwalten
//!   - Kann Locations/Settings des Tenants verwalten
//!
//! - **logistic_manager**: Logistik-Manager
//!   - Kann Inventory, Shipments, Stock verwalten
//!   - Kein User-Management
//!
//! - **paramedic**: Standard-User (Sanitäter)
//!   - Kann Verbrauch melden
//!   - Kann Inventory anzeigen

use axum::{
    Json,
    Router,
    routing::{post, get},
    Extension,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode}};

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use chrono::{Utc, Duration};
use uuid::Uuid;
use std::sync::Arc;
use crate::config::AppConfig;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString, PasswordHash},
    Argon2
};

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

#[derive(Serialize)]
pub struct UserInfo {
    pub username: String,
    pub role: String,
    pub user_id: String,
    pub tenant_id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

/// Extractor für StaxMed Super-Admins (Zugriff auf alle Tenants)
pub struct StaxAdminUser {
    pub user_id: Uuid,
}

/// Extractor für Tenant-Admins (Zugriff nur auf eigenen Tenant)
pub struct AdminUser {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub tenant_id: Option<Uuid>,
}

// Extractor für jeden authentifizierten User (nicht nur Admins)
pub struct CurrentUser {
    pub user_id: Uuid,
    pub username: String,
    pub role: String,
    pub tenant_id: Option<Uuid>,
}

// Hilfs-Struktur für die Datenbank-Abfrage
struct UserRecord {
    id: Uuid,
    password_hash: Option<String>,  // sqlx gibt Option zurück
    role: Option<String>,           // sqlx gibt Option zurück
    tenant_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,       // User ID
    tenant_id: String, // Kann leer sein für System-User
    role: String,
    exp: usize,
    #[serde(default)]
    username: String,  // Username im Token speichern
}

impl<S> FromRequestParts<S> for StaxAdminUser
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

        // 3. JWT Secret aus Extensions holen
        let config = parts.extensions.get::<Arc<AppConfig>>()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Config not found".into()))?;

        // 4. JWT dekodieren mit strikter Validation
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        validation.leeway = 0; // Keine Zeittoleranz

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt_secret.as_ref()),
            &validation,
        ).map_err(|_| (StatusCode::UNAUTHORIZED, "Ungültiger Token".into()))?;

        // 5. Rolle prüfen - nur staxadmin erlaubt
        if token_data.claims.role != "staxadmin" {
            return Err((StatusCode::FORBIDDEN, "StaxAdmin-Rechte erforderlich".into()));
        }

        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Ungültiger Token".into()))?;

        Ok(StaxAdminUser { user_id })
    }
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

        // 3. JWT Secret aus Extensions holen
        let config = parts.extensions.get::<Arc<AppConfig>>()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Config not found".into()))?;

        // 4. JWT dekodieren mit strikter Validation
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        validation.leeway = 0; // Keine Zeittoleranz

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt_secret.as_ref()),
            &validation,
        ).map_err(|_| (StatusCode::UNAUTHORIZED, "Ungültiger Token".into()))?;

        // 5. Rolle prüfen - admin oder staxadmin erlaubt
        if token_data.claims.role != "admin" && token_data.claims.role != "staxadmin" {
            return Err((StatusCode::FORBIDDEN, "Admin-Rechte erforderlich".into()));
        }

        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Ungültiger Token".into()))?;

        // tenant_id kann leer sein - dann FORBIDDEN
        let tenant_id = Uuid::parse_str(&token_data.claims.tenant_id)
            .map_err(|_| (StatusCode::FORBIDDEN, "Kein Tenant zugeordnet".into()))?;

        Ok(AdminUser { user_id, tenant_id })
    }
}

impl<S> FromRequestParts<S> for CurrentUser
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

        // 3. JWT Secret aus Extensions holen
        let config = parts.extensions.get::<Arc<AppConfig>>()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "Config not found".into()))?;

        // 4. JWT dekodieren mit strikter Validation
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        validation.leeway = 0; // Keine Zeittoleranz

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt_secret.as_ref()),
            &validation,
        ).map_err(|_| (StatusCode::UNAUTHORIZED, "Ungültiger Token".into()))?;

        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Ungültiger Token".into()))?;
        let tenant_id = Uuid::parse_str(&token_data.claims.tenant_id).ok();

        // Username ist bereits im Token - keine DB-Abfrage nötig
        Ok(CurrentUser {
            user_id,
            username: token_data.claims.username,
            role: token_data.claims.role,
            tenant_id,
        })
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .route("/register", post(register_handler))
        .route("/me", get(me_handler))
        .route("/dev/hash", post(generate_hash_handler)) // Nur für Development!
}

async fn me_handler(
    current_user: CurrentUser,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<UserInfo>, (StatusCode, String)> {
    // Hole vollständige User-Daten aus DB
    let user_data = sqlx::query!(
        "SELECT first_name, last_name FROM users WHERE id = $1",
        current_user.user_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Datenbankfehler".into()))?;

    Ok(Json(UserInfo {
        username: current_user.username,
        role: current_user.role,
        user_id: current_user.user_id.to_string(),
        tenant_id: current_user.tenant_id.map(|id| id.to_string()),
        first_name: user_data.first_name,
        last_name: user_data.last_name,
    }))
}

async fn login_handler(
    Extension(pool): Extension<PgPool>,
    Extension(config): Extension<Arc<AppConfig>>,
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
    .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Datenbankfehler".into()))?
    .ok_or((axum::http::StatusCode::UNAUTHORIZED, "Ungültige Anmeldedaten".into()))?;

    // 2. Passwort prüfen (Argon2 Verifizierung)
    let hash = user.password_hash
        .ok_or((axum::http::StatusCode::UNAUTHORIZED, "Ungültige Anmeldedaten".into()))?;
    let parsed_hash = PasswordHash::new(&hash)
        .map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Ungültiges Hash-Format".into()))?;

    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| (axum::http::StatusCode::UNAUTHORIZED, "Ungültige Anmeldedaten".into()))?;

    // 3. Token erstellen (1 Stunde Gültigkeit für bessere Sicherheit)
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        tenant_id: user.tenant_id.map(|id| id.to_string()).unwrap_or_default(),
        role: user.role.unwrap_or_else(|| "user".into()),
        username: payload.username.clone(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    ).map_err(|_| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Token-Fehler".into()))?;

    Ok(Json(AuthResponse {
        token,
        username: payload.username,
        role: claims.role,
    }))
}

async fn register_handler(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Transaktion starten: Entweder alles oder nichts!
    let mut tx = pool.begin().await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Transaktionsfehler".into()))?;

    // 1. Tenant validieren oder erstellen (wenn tenant_id nicht angegeben)
    let tenant_id = if let Some(tid) = payload.tenant_id {
        // Prüfen ob Tenant existiert
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM tenants WHERE id = $1)",
            tid
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Datenbankfehler".into()))?
        .unwrap_or(false);

        if !exists {
            return Err((StatusCode::BAD_REQUEST, "Tenant existiert nicht".into()));
        }
        tid
    } else {
        // Erstelle neuen Tenant für Self-Service Registration
        sqlx::query_scalar!(
            "INSERT INTO tenants (name, status) VALUES ($1, 'trial') RETURNING id",
            format!("{}'s Organization", payload.username)
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| (StatusCode::BAD_REQUEST, "Fehler beim Anlegen der Organisation".into()))?
    };

    // 2. Passwort hashen
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Fehler beim Hashing".into()))?
        .to_string();

    // 3. Bestimme Rolle: Wenn kein tenant_id angegeben war, wird User Admin des neuen Tenants
    // Niemals 'staxadmin' über API vergeben - nur durch manuelle DB-Manipulation
    let role = if payload.tenant_id.is_none() { "admin" } else { "paramedic" };

    // 4. User in DB speichern
    sqlx::query!(
        "INSERT INTO users (id, username, password_hash, role, tenant_id, is_active, first_name, last_name)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        Uuid::new_v4(),
        payload.username,
        password_hash,
        role,
        tenant_id,
        true,
        payload.first_name,
        payload.last_name
    )
    .execute(&mut *tx)
    .await
    .map_err(|_| (StatusCode::BAD_REQUEST, "Benutzername bereits vergeben".into()))?;

    // Alles speichern
    tx.commit().await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Fehler beim Abschluss der Registrierung".into()))?;

    Ok(StatusCode::CREATED)
}

// ENTWICKLUNGS-TOOL: Generiert Argon2 Hash für ein Passwort
// TODO: Entfernen oder nur in Development-Mode aktivieren!
#[derive(Deserialize)]
struct HashRequest {
    password: String,
}

#[derive(Serialize)]
struct HashResponse {
    hash: String,
}

async fn generate_hash_handler(
    Json(payload): Json<HashRequest>,
) -> Result<Json<HashResponse>, (StatusCode, String)> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Fehler beim Hashing".into()))?
        .to_string();

    Ok(Json(HashResponse {
        hash: password_hash,
    }))
}
