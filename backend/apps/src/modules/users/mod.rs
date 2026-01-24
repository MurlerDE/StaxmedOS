//! User Management Module
//!
//! Provides APIs for:
//! - User CRUD operations
//! - Permission management
//! - Role assignment
//! - User listing with filters

use axum::{
    Json,
    Router,
    Extension,
    routing::{get, post, put, delete},
    extract::Path,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2
};

use crate::modules::auth::{AdminUser, StaxAdminUser, CurrentUser};

// ============================================================================
// DTOs (Data Transfer Objects)
// ============================================================================

#[derive(Serialize, Deserialize)]
pub struct UserListItem {
    pub id: String,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
    pub tenant_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct UserDetail {
    pub id: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: String,
    pub is_active: bool,
    pub tenant_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub roles: Vec<UserRoleInfo>,
    pub permissions: Vec<PermissionInfo>,
}

#[derive(Serialize)]
pub struct UserRoleInfo {
    pub role_id: String,
    pub role_name: String,
    pub display_name: String,
    pub assigned_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct PermissionInfo {
    pub permission_name: String,
    pub source: String,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role_ids: Option<Vec<String>>, // Optional: Rollen-IDs
    pub tenant_id: Option<String>, // Nur für StaxAdmin
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Deserialize)]
pub struct AssignRoleRequest {
    pub role_id: String,
}

#[derive(Deserialize)]
pub struct AssignPermissionRequest {
    pub permission_name: String,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct UpdatePasswordRequest {
    pub new_password: String,
}

#[derive(Serialize)]
pub struct RoleListItem {
    pub id: String,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub is_system: Option<bool>,
    pub permission_count: i64,
}

#[derive(Serialize)]
pub struct PermissionListItem {
    pub id: String,
    pub name: Option<String>,
    pub resource: Option<String>,
    pub action: Option<String>,
    pub scope: Option<String>,
    pub description: Option<String>,
}

// ============================================================================
// Router
// ============================================================================

pub fn router() -> Router {
    Router::new()
        // User CRUD
        .route("/", get(list_users))
        .route("/", post(create_user))
        .route("/{user_id}", get(get_user))
        .route("/{user_id}", put(update_user))
        .route("/{user_id}", delete(delete_user))

        // Password Management
        .route("/{user_id}/password", put(update_password))

        // Role Assignment
        .route("/{user_id}/roles", get(get_user_roles))
        .route("/{user_id}/roles", post(assign_role))
        .route("/{user_id}/roles/{role_id}", delete(remove_role))

        // Permission Management
        .route("/{user_id}/permissions", get(get_user_permissions))
        .route("/{user_id}/permissions", post(assign_permission))
        .route("/{user_id}/permissions/{permission_id}", delete(remove_permission))

        // Available Roles & Permissions
        .route("/roles/available", get(list_available_roles))
        .route("/permissions/available", get(list_available_permissions))
}

// ============================================================================
// Handlers: User CRUD
// ============================================================================

/// List users (Tenant Admin: nur eigener Tenant, StaxAdmin: alle oder gefiltert)
async fn list_users(
    current_user: CurrentUser,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<UserListItem>>, (StatusCode, String)> {

    // Prüfe ob User Berechtigung hat
    let has_view_users = sqlx::query_scalar!(
        "SELECT user_has_permission($1, 'users.view')",
        current_user.user_id
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(Some(false))
    .unwrap_or(false);

    if !has_view_users {
        return Err((StatusCode::FORBIDDEN, "Keine Berechtigung zum Anzeigen von Benutzern".into()));
    }

    // StaxAdmin sieht alle, Tenant Admin nur eigenen Tenant
    let users = if current_user.role == "staxadmin" {
        sqlx::query_as!(
            UserListItem,
            r#"
            SELECT
                id::text as "id!",
                username as "username?",
                first_name,
                last_name,
                role as "role?",
                is_active as "is_active?",
                tenant_id::text as "tenant_id?",
                created_at as "created_at?"
            FROM users
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&pool)
        .await
    } else {
        sqlx::query_as!(
            UserListItem,
            r#"
            SELECT
                id::text as "id!",
                username as "username?",
                first_name,
                last_name,
                role as "role?",
                is_active as "is_active?",
                tenant_id::text as "tenant_id?",
                created_at as "created_at?"
            FROM users
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            "#,
            current_user.tenant_id
        )
        .fetch_all(&pool)
        .await
    };

    users
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Datenbankfehler: {}", e)))
}

/// Get single user with details
async fn get_user(
    current_user: CurrentUser,
    Path(user_id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<UserDetail>, (StatusCode, String)> {

    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Ungültige User-ID".into()))?;

    // Prüfe Berechtigung
    let has_view = sqlx::query_scalar!(
        "SELECT user_has_permission($1, 'users.view')",
        current_user.user_id
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(Some(false))
    .unwrap_or(false);

    if !has_view {
        return Err((StatusCode::FORBIDDEN, "Keine Berechtigung".into()));
    }

    // Hole User-Basis-Daten
    let user = sqlx::query!(
        r#"
        SELECT
            id,
            username,
            first_name,
            last_name,
            role,
            is_active,
            tenant_id,
            created_at
        FROM users
        WHERE id = $1
        "#,
        user_uuid
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        eprintln!("ERROR loading user {}: {}", user_uuid, e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Datenbankfehler: {}", e))
    })?
    .ok_or_else(|| {
        eprintln!("ERROR: User {} not found", user_uuid);
        (StatusCode::NOT_FOUND, "User nicht gefunden".into())
    })?;

    // Prüfe Tenant-Zugriff (außer StaxAdmin)
    if current_user.role != "staxadmin" {
        // user.tenant_id ist Uuid (NOT NULL in DB), current_user.tenant_id ist Option<Uuid>
        match current_user.tenant_id {
            Some(current_tid) if user.tenant_id == current_tid => {
                // OK, gleicher Tenant
            }
            _ => {
                return Err((StatusCode::FORBIDDEN, "Zugriff verweigert".into()));
            }
        }
    }

    // Hole Rollen
    let roles = sqlx::query_as!(
        UserRoleInfo,
        r#"
        SELECT
            r.id::text as "role_id!",
            r.name as "role_name!",
            r.display_name as "display_name!",
            ur.assigned_at as "assigned_at?"
        FROM user_roles ur
        JOIN roles r ON ur.role_id = r.id
        WHERE ur.user_id = $1
        "#,
        user_uuid
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("ERROR loading roles for user {}: {}", user_uuid, e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Fehler beim Laden der Rollen: {}", e))
    })?;

    // Hole Permissions (inline statt Function Call wegen sqlx Kompatibilität)
    let permissions_raw = sqlx::query!(
        r#"
        -- Permissions über Rollen
        SELECT DISTINCT
            p.name as permission_name,
            'role:' || r.name as source
        FROM user_roles ur
        JOIN roles r ON ur.role_id = r.id
        JOIN role_permissions rp ON r.id = rp.role_id
        JOIN permissions p ON rp.permission_id = p.id
        WHERE ur.user_id = $1

        UNION

        -- Direkte User-Permissions
        SELECT
            p.name as permission_name,
            'direct' as source
        FROM user_permissions up
        JOIN permissions p ON up.permission_id = p.id
        WHERE up.user_id = $1
          AND up.granted = true
          AND (up.expires_at IS NULL OR up.expires_at > now())
        "#,
        user_uuid
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        eprintln!("ERROR loading permissions for user {}: {}", user_uuid, e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Fehler beim Laden der Permissions: {}", e))
    })?;

    // Konvertiere zu PermissionInfo Struktur
    let permissions: Vec<PermissionInfo> = permissions_raw
        .into_iter()
        .map(|row| PermissionInfo {
            permission_name: row.permission_name.unwrap_or_default(),
            source: row.source.unwrap_or_default(),
        })
        .collect();

    Ok(Json(UserDetail {
        id: user.id.to_string(),
        username: user.username,
        first_name: user.first_name,
        last_name: user.last_name,
        role: user.role,
        is_active: user.is_active.unwrap_or(true),
        tenant_id: Some(user.tenant_id.to_string()),
        created_at: user.created_at.unwrap_or_else(Utc::now),
        roles,
        permissions,
    }))
}

/// Create new user
async fn create_user(
    current_user: CurrentUser,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserDetail>, (StatusCode, String)> {

    // Prüfe Berechtigung
    let has_create = sqlx::query_scalar!(
        "SELECT user_has_permission($1, 'users.create')",
        current_user.user_id
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(Some(false))
    .unwrap_or(false);

    if !has_create {
        return Err((StatusCode::FORBIDDEN, "Keine Berechtigung zum Erstellen von Benutzern".into()));
    }

    // Tenant-Admin kann nur User in eigenem Tenant erstellen
    let target_tenant_id = if current_user.role == "staxadmin" {
        payload.tenant_id.as_ref()
            .and_then(|id| Uuid::parse_str(id).ok())
    } else {
        current_user.tenant_id
    };

    if target_tenant_id.is_none() {
        return Err((StatusCode::BAD_REQUEST, "Tenant-ID erforderlich".into()));
    }

    // Passwort hashen
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Fehler beim Hashing".into()))?
        .to_string();

    // Transaktion starten
    let mut tx = pool.begin().await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Transaktionsfehler".into()))?;

    // User erstellen
    let new_user_id = sqlx::query_scalar!(
        r#"
        INSERT INTO users (username, password_hash, first_name, last_name, role, tenant_id, is_active)
        VALUES ($1, $2, $3, $4, 'paramedic', $5, true)
        RETURNING id
        "#,
        payload.username,
        password_hash,
        payload.first_name,
        payload.last_name,
        target_tenant_id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, format!("User existiert bereits oder Fehler: {}", e)))?;

    // Rollen zuweisen (falls angegeben)
    if let Some(role_ids) = payload.role_ids {
        for role_id_str in role_ids {
            if let Ok(role_id) = Uuid::parse_str(&role_id_str) {
                sqlx::query!(
                    "INSERT INTO user_roles (user_id, role_id, assigned_by) VALUES ($1, $2, $3)",
                    new_user_id,
                    role_id,
                    current_user.user_id
                )
                .execute(&mut *tx)
                .await
                .ok(); // Ignoriere Fehler wenn Rolle nicht existiert
            }
        }
    } else {
        // Default: Paramedic-Rolle zuweisen
        sqlx::query!(
            r#"
            INSERT INTO user_roles (user_id, role_id, assigned_by)
            SELECT $1, id, $2
            FROM roles
            WHERE name = 'paramedic' AND (tenant_id = $3 OR tenant_id IS NULL)
            LIMIT 1
            "#,
            new_user_id,
            current_user.user_id,
            target_tenant_id
        )
        .execute(&mut *tx)
        .await
        .ok();
    }

    tx.commit().await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Fehler beim Speichern".into()))?;

    // User-Details laden und zurückgeben
    get_user(current_user, Path(new_user_id.to_string()), Extension(pool)).await
}

/// Update user
async fn update_user(
    current_user: CurrentUser,
    Path(user_id): Path<String>,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserDetail>, (StatusCode, String)> {

    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Ungültige User-ID".into()))?;

    // Prüfe Berechtigung
    let has_update = sqlx::query_scalar!(
        "SELECT user_has_permission($1, 'users.update')",
        current_user.user_id
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(Some(false))
    .unwrap_or(false);

    if !has_update {
        return Err((StatusCode::FORBIDDEN, "Keine Berechtigung".into()));
    }

    // Update User
    sqlx::query!(
        r#"
        UPDATE users
        SET
            first_name = COALESCE($2, first_name),
            last_name = COALESCE($3, last_name),
            is_active = COALESCE($4, is_active)
        WHERE id = $1
        "#,
        user_uuid,
        payload.first_name,
        payload.last_name,
        payload.is_active
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Fehler beim Update: {}", e)))?;

    get_user(current_user, Path(user_id), Extension(pool)).await
}

/// Delete user
async fn delete_user(
    current_user: CurrentUser,
    Path(user_id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<StatusCode, (StatusCode, String)> {

    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Ungültige User-ID".into()))?;

    // Prüfe Berechtigung
    let has_delete = sqlx::query_scalar!(
        "SELECT user_has_permission($1, 'users.delete')",
        current_user.user_id
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(Some(false))
    .unwrap_or(false);

    if !has_delete {
        return Err((StatusCode::FORBIDDEN, "Keine Berechtigung".into()));
    }

    // Verhindere Selbstlöschung
    if user_uuid == current_user.user_id {
        return Err((StatusCode::BAD_REQUEST, "Selbstlöschung nicht erlaubt".into()));
    }

    sqlx::query!("DELETE FROM users WHERE id = $1", user_uuid)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Fehler beim Löschen: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// Handlers: Password Management
// ============================================================================

async fn update_password(
    current_user: CurrentUser,
    Path(user_id): Path<String>,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<UpdatePasswordRequest>,
) -> Result<StatusCode, (StatusCode, String)> {

    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Ungültige User-ID".into()))?;

    // User darf eigenes Passwort ändern, oder braucht users.update Permission
    if user_uuid != current_user.user_id {
        let has_update = sqlx::query_scalar!(
            "SELECT user_has_permission($1, 'users.update')",
            current_user.user_id
        )
        .fetch_one(&pool)
        .await
        .unwrap_or(Some(false))
        .unwrap_or(false);

        if !has_update {
            return Err((StatusCode::FORBIDDEN, "Keine Berechtigung".into()));
        }
    }

    // Passwort hashen
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.new_password.as_bytes(), &salt)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Fehler beim Hashing".into()))?
        .to_string();

    sqlx::query!(
        "UPDATE users SET password_hash = $1 WHERE id = $2",
        password_hash,
        user_uuid
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Fehler: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// Handlers: Role Assignment
// ============================================================================

async fn get_user_roles(
    _current_user: CurrentUser,
    Path(user_id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<UserRoleInfo>>, (StatusCode, String)> {

    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Ungültige User-ID".into()))?;

    let roles = sqlx::query_as!(
        UserRoleInfo,
        r#"
        SELECT
            r.id::text as "role_id!",
            r.name as "role_name!",
            r.display_name as "display_name!",
            ur.assigned_at as "assigned_at?"
        FROM user_roles ur
        JOIN roles r ON ur.role_id = r.id
        WHERE ur.user_id = $1
        "#,
        user_uuid
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Fehler: {}", e)))?;

    Ok(Json(roles))
}

async fn assign_role(
    current_user: CurrentUser,
    Path(user_id): Path<String>,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<AssignRoleRequest>,
) -> Result<StatusCode, (StatusCode, String)> {

    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Ungültige User-ID".into()))?;

    let role_uuid = Uuid::parse_str(&payload.role_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Ungültige Role-ID".into()))?;

    // Prüfe Berechtigung
    let has_manage = sqlx::query_scalar!(
        "SELECT user_has_permission($1, 'users.manage_permissions')",
        current_user.user_id
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(Some(false))
    .unwrap_or(false);

    if !has_manage {
        return Err((StatusCode::FORBIDDEN, "Keine Berechtigung".into()));
    }

    sqlx::query!(
        "INSERT INTO user_roles (user_id, role_id, assigned_by) VALUES ($1, $2, $3) ON CONFLICT (user_id, role_id) DO NOTHING",
        user_uuid,
        role_uuid,
        current_user.user_id
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Fehler: {}", e)))?;

    Ok(StatusCode::CREATED)
}

async fn remove_role(
    current_user: CurrentUser,
    Path((user_id, role_id)): Path<(String, String)>,
    Extension(pool): Extension<PgPool>,
) -> Result<StatusCode, (StatusCode, String)> {

    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Ungültige User-ID".into()))?;

    let role_uuid = Uuid::parse_str(&role_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Ungültige Role-ID".into()))?;

    // Prüfe Berechtigung
    let has_manage = sqlx::query_scalar!(
        "SELECT user_has_permission($1, 'users.manage_permissions')",
        current_user.user_id
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(Some(false))
    .unwrap_or(false);

    if !has_manage {
        return Err((StatusCode::FORBIDDEN, "Keine Berechtigung".into()));
    }

    sqlx::query!(
        "DELETE FROM user_roles WHERE user_id = $1 AND role_id = $2",
        user_uuid,
        role_uuid
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Fehler: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// Handlers: Permission Assignment
// ============================================================================

async fn get_user_permissions(
    Path(user_id): Path<String>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<PermissionInfo>>, (StatusCode, String)> {

    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Ungültige User-ID".into()))?;

    let permissions_raw = sqlx::query!(
        r#"
        -- Permissions über Rollen
        SELECT DISTINCT
            p.name as permission_name,
            'role:' || r.name as source
        FROM user_roles ur
        JOIN roles r ON ur.role_id = r.id
        JOIN role_permissions rp ON r.id = rp.role_id
        JOIN permissions p ON rp.permission_id = p.id
        WHERE ur.user_id = $1

        UNION

        -- Direkte User-Permissions
        SELECT
            p.name as permission_name,
            'direct' as source
        FROM user_permissions up
        JOIN permissions p ON up.permission_id = p.id
        WHERE up.user_id = $1
          AND up.granted = true
          AND (up.expires_at IS NULL OR up.expires_at > now())
        "#,
        user_uuid
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Fehler: {}", e)))?;

    // Konvertiere zu PermissionInfo Struktur
    let permissions: Vec<PermissionInfo> = permissions_raw
        .into_iter()
        .map(|row| PermissionInfo {
            permission_name: row.permission_name.unwrap_or_default(),
            source: row.source.unwrap_or_default(),
        })
        .collect();

    Ok(Json(permissions))
}

async fn assign_permission(
    current_user: CurrentUser,
    Path(user_id): Path<String>,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<AssignPermissionRequest>,
) -> Result<StatusCode, (StatusCode, String)> {

    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Ungültige User-ID".into()))?;

    // Prüfe Berechtigung
    let has_manage = sqlx::query_scalar!(
        "SELECT user_has_permission($1, 'users.manage_permissions')",
        current_user.user_id
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(Some(false))
    .unwrap_or(false);

    if !has_manage {
        return Err((StatusCode::FORBIDDEN, "Keine Berechtigung".into()));
    }

    // Hole Permission-ID
    let perm_id = sqlx::query_scalar!(
        "SELECT id FROM permissions WHERE name = $1",
        payload.permission_name
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Fehler: {}", e)))?
    .ok_or((StatusCode::NOT_FOUND, "Permission nicht gefunden".into()))?;

    sqlx::query!(
        r#"
        INSERT INTO user_permissions (user_id, permission_id, granted, granted_by, expires_at)
        VALUES ($1, $2, true, $3, $4)
        ON CONFLICT (user_id, permission_id) DO UPDATE
        SET granted = true, expires_at = $4
        "#,
        user_uuid,
        perm_id,
        current_user.user_id,
        payload.expires_at
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Fehler: {}", e)))?;

    Ok(StatusCode::CREATED)
}

async fn remove_permission(
    current_user: CurrentUser,
    Path((user_id, permission_id)): Path<(String, String)>,
    Extension(pool): Extension<PgPool>,
) -> Result<StatusCode, (StatusCode, String)> {

    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Ungültige User-ID".into()))?;

    let perm_uuid = Uuid::parse_str(&permission_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Ungültige Permission-ID".into()))?;

    // Prüfe Berechtigung
    let has_manage = sqlx::query_scalar!(
        "SELECT user_has_permission($1, 'users.manage_permissions')",
        current_user.user_id
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(Some(false))
    .unwrap_or(false);

    if !has_manage {
        return Err((StatusCode::FORBIDDEN, "Keine Berechtigung".into()));
    }

    sqlx::query!(
        "DELETE FROM user_permissions WHERE user_id = $1 AND permission_id = $2",
        user_uuid,
        perm_uuid
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Fehler: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// Handlers: Available Roles & Permissions
// ============================================================================

async fn list_available_roles(
    current_user: CurrentUser,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<RoleListItem>>, (StatusCode, String)> {

    // StaxAdmin sieht alle, Tenant Admin nur eigene + System-Rollen
    let roles = if current_user.role == "staxadmin" {
        sqlx::query_as!(
            RoleListItem,
            r#"
            SELECT
                r.id::text as "id!",
                r.name as "name?",
                r.display_name as "display_name?",
                r.description,
                r.is_system as "is_system?",
                COUNT(rp.permission_id) as "permission_count!"
            FROM roles r
            LEFT JOIN role_permissions rp ON r.id = rp.role_id
            GROUP BY r.id
            ORDER BY r.is_system DESC, r.name
            "#
        )
        .fetch_all(&pool)
        .await
    } else {
        sqlx::query_as!(
            RoleListItem,
            r#"
            SELECT
                r.id::text as "id!",
                r.name as "name?",
                r.display_name as "display_name?",
                r.description,
                r.is_system as "is_system?",
                COUNT(rp.permission_id) as "permission_count!"
            FROM roles r
            LEFT JOIN role_permissions rp ON r.id = rp.role_id
            WHERE r.tenant_id = $1 OR r.tenant_id IS NULL
            GROUP BY r.id
            ORDER BY r.is_system DESC, r.name
            "#,
            current_user.tenant_id
        )
        .fetch_all(&pool)
        .await
    };

    roles
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Fehler: {}", e)))
}

async fn list_available_permissions(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<PermissionListItem>>, (StatusCode, String)> {

    let permissions = sqlx::query_as!(
        PermissionListItem,
        r#"
        SELECT
            id::text as "id!",
            name as "name?",
            resource as "resource?",
            action as "action?",
            scope as "scope?",
            description
        FROM permissions
        ORDER BY resource, action
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Fehler: {}", e)))?;

    Ok(Json(permissions))
}
