use axum::{
    Json,
    Router,
    routing::{get, post, put, delete, patch},
    Extension,
    extract::{Path, Query},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use bigdecimal::BigDecimal;


// ============================================================================
// TENANT STRUCTS
// ============================================================================

#[derive(Serialize, sqlx::FromRow)]
pub struct TenantListItem {
    pub id: Uuid,
    pub customer_number: Option<String>,
    pub name: String,
    pub status: Option<String>,
    pub plan_id: Option<Uuid>,
    pub plan_name: Option<String>,
    pub valid_until: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub user_count: Option<i64>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct TenantDetail {
    pub id: Uuid,
    pub customer_number: Option<String>,
    pub name: String,
    pub license_key: Option<String>,
    pub valid_until: Option<DateTime<Utc>>,
    pub is_on_premise: Option<bool>,
    pub hardware_hash: Option<String>,
    pub plan_id: Option<Uuid>,
    pub status: Option<String>,
    pub vat_number: Option<String>,
    pub tax_id: Option<String>,
    pub website: Option<String>,
    pub industry: Option<String>,
    pub company_size: Option<String>,
    pub preferred_language: Option<String>,
    pub notes: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct CreateTenantRequest {
    pub name: String,
    pub plan_id: Uuid,
    pub status: Option<String>,
    pub vat_number: Option<String>,
    pub tax_id: Option<String>,
    pub website: Option<String>,
    pub industry: Option<String>,
    pub company_size: Option<String>,
    pub preferred_language: Option<String>,
    pub notes: Option<String>,
    pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct UpdateTenantRequest {
    pub name: Option<String>,
    pub status: Option<String>,
    pub vat_number: Option<String>,
    pub tax_id: Option<String>,
    pub website: Option<String>,
    pub industry: Option<String>,
    pub company_size: Option<String>,
    pub preferred_language: Option<String>,
    pub notes: Option<String>,
    pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct TenantQueryParams {
    pub status: Option<String>,
    pub plan_id: Option<Uuid>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

// ============================================================================
// ADDRESS STRUCTS
// ============================================================================

#[derive(Serialize, sqlx::FromRow)]
pub struct TenantAddress {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub address_type: String,
    pub company_name: Option<String>,
    pub street: String,
    pub street_additional: Option<String>,
    pub postal_code: String,
    pub city: String,
    pub state: Option<String>,
    pub country: String,
    pub is_primary: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct CreateAddressRequest {
    pub address_type: String,
    pub company_name: Option<String>,
    pub street: String,
    pub street_additional: Option<String>,
    pub postal_code: String,
    pub city: String,
    pub state: Option<String>,
    pub country: Option<String>,
    pub is_primary: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdateAddressRequest {
    pub address_type: Option<String>,
    pub company_name: Option<String>,
    pub street: Option<String>,
    pub street_additional: Option<String>,
    pub postal_code: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub is_primary: Option<bool>,
}

// ============================================================================
// CONTACT STRUCTS
// ============================================================================

#[derive(Serialize, sqlx::FromRow)]
pub struct TenantContact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub contact_type: String,
    pub salutation: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub position: Option<String>,
    pub email: String,
    pub phone: Option<String>,
    pub mobile: Option<String>,
    pub fax: Option<String>,
    pub is_primary: Option<bool>,
    pub notes: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct CreateContactRequest {
    pub contact_type: String,
    pub salutation: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub position: Option<String>,
    pub email: String,
    pub phone: Option<String>,
    pub mobile: Option<String>,
    pub fax: Option<String>,
    pub is_primary: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateContactRequest {
    pub contact_type: Option<String>,
    pub salutation: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub position: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub mobile: Option<String>,
    pub fax: Option<String>,
    pub is_primary: Option<bool>,
    pub notes: Option<String>,
}

// ============================================================================
// PAYMENT STRUCTS
// ============================================================================

#[derive(Serialize, sqlx::FromRow)]
pub struct TenantPayment {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub payment_method: String,
    pub payment_status: String,
    pub amount: BigDecimal,
    pub currency: Option<String>,
    pub billing_period_start: Option<NaiveDate>,
    pub billing_period_end: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
    pub paid_at: Option<DateTime<Utc>>,
    pub invoice_number: Option<String>,
    pub notes: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct CreatePaymentRequest {
    pub payment_method: String,
    pub amount: BigDecimal,
    pub currency: Option<String>,
    pub billing_period_start: Option<NaiveDate>,
    pub billing_period_end: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
    pub invoice_number: Option<String>,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdatePaymentRequest {
    pub payment_status: Option<String>,
    pub paid_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

// ============================================================================
// PLAN STRUCTS
// ============================================================================

#[derive(Serialize, sqlx::FromRow)]
pub struct Plan {
    pub id: Uuid,
    pub name: String,
    pub max_users: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct SwitchPlanRequest {
    pub plan_id: Uuid,
}

// ============================================================================
// STATISTICS STRUCTS
// ============================================================================

#[derive(Serialize)]
pub struct TenantStatistics {
    pub total_tenants: i64,
    pub active_tenants: i64,
    pub trial_tenants: i64,
    pub suspended_tenants: i64,
    pub total_users: i64,
    pub tenants_by_plan: Vec<TenantsByPlan>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct TenantsByPlan {
    pub plan_name: Option<String>,
    pub count: Option<i64>,
}

// ============================================================================
// ROUTER
// ============================================================================

pub fn router() -> Router {
    Router::new()
        // Tenant Management
        .route("/tenants", get(list_tenants))
        .route("/tenants", post(create_tenant))
        .route("/tenants/{id}", get(get_tenant))
        .route("/tenants/{id}", put(update_tenant))
        .route("/tenants/{id}", delete(delete_tenant))
        .route("/tenants/{id}/status", patch(update_tenant_status))

        // Plan Management
        .route("/tenants/{id}/plan", put(switch_tenant_plan))
        .route("/plans", get(list_plans))

        // Address Management
        .route("/tenants/{id}/addresses", get(list_tenant_addresses))
        .route("/tenants/{id}/addresses", post(create_tenant_address))
        .route("/tenants/{tenant_id}/addresses/{address_id}", put(update_tenant_address))
        .route("/tenants/{tenant_id}/addresses/{address_id}", delete(delete_tenant_address))

        // Contact Management
        .route("/tenants/{id}/contacts", get(list_tenant_contacts))
        .route("/tenants/{id}/contacts", post(create_tenant_contact))
        .route("/tenants/{tenant_id}/contacts/{contact_id}", put(update_tenant_contact))
        .route("/tenants/{tenant_id}/contacts/{contact_id}", delete(delete_tenant_contact))

        // Payment Management
        .route("/tenants/{id}/payments", get(list_tenant_payments))
        .route("/tenants/{id}/payments", post(create_tenant_payment))
        .route("/tenants/{tenant_id}/payments/{payment_id}", put(update_tenant_payment))
        .route("/tenants/{tenant_id}/payments/{payment_id}", delete(delete_tenant_payment))

        // Statistics
        .route("/statistics", get(get_statistics))
}

// ============================================================================
// TENANT HANDLERS
// ============================================================================

async fn list_tenants(
    Extension(pool): Extension<PgPool>,
    Query(params): Query<TenantQueryParams>,
) -> Result<Json<Vec<TenantListItem>>, (StatusCode, String)> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let mut query = String::from(
        r#"SELECT
            t.id,
            t.customer_number,
            t.name,
            t.status,
            t.plan_id,
            p.name as plan_name,
            t.valid_until,
            t.created_at,
            COUNT(u.id) as user_count
        FROM tenants t
        LEFT JOIN plans p ON t.plan_id = p.id
        LEFT JOIN users u ON t.id = u.tenant_id
        WHERE 1=1"#
    );

    if let Some(status) = &params.status {
        query.push_str(&format!(" AND t.status = '{}'", status));
    }

    if let Some(plan_id) = &params.plan_id {
        query.push_str(&format!(" AND t.plan_id = '{}'", plan_id));
    }

    if let Some(search) = &params.search {
        query.push_str(&format!(
            " AND (t.name ILIKE '%{}%' OR t.customer_number ILIKE '%{}%')",
            search, search
        ));
    }

    query.push_str(" GROUP BY t.id, p.name ORDER BY t.created_at DESC");
    query.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let tenants = sqlx::query_as::<_, TenantListItem>(&query)
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(tenants))
}

async fn get_tenant(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<TenantDetail>, (StatusCode, String)> {
    let tenant = sqlx::query_as!(
        TenantDetail,
        r#"SELECT
            id, customer_number, name, license_key, valid_until,
            is_on_premise, hardware_hash, plan_id, status,
            vat_number, tax_id, website, industry, company_size,
            preferred_language, notes, created_at, updated_at
        FROM tenants
        WHERE id = $1"#,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::NOT_FOUND, "Tenant not found".to_string()))?;

    Ok(Json(tenant))
}

async fn create_tenant(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateTenantRequest>,
) -> Result<(StatusCode, Json<TenantDetail>), (StatusCode, String)> {
    let status = payload.status.unwrap_or_else(|| "trial".to_string());

    let tenant = sqlx::query_as!(
        TenantDetail,
        r#"INSERT INTO tenants (
            name, plan_id, status, vat_number, tax_id, website,
            industry, company_size, preferred_language, notes, valid_until
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING
            id, customer_number, name, license_key, valid_until,
            is_on_premise, hardware_hash, plan_id, status,
            vat_number, tax_id, website, industry, company_size,
            preferred_language, notes, created_at, updated_at"#,
        payload.name,
        payload.plan_id,
        status,
        payload.vat_number,
        payload.tax_id,
        payload.website,
        payload.industry,
        payload.company_size,
        payload.preferred_language,
        payload.notes,
        payload.valid_until
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(tenant)))
}

async fn update_tenant(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTenantRequest>,
) -> Result<Json<TenantDetail>, (StatusCode, String)> {
    let tenant = sqlx::query_as!(
        TenantDetail,
        r#"UPDATE tenants SET
            name = COALESCE($2, name),
            status = COALESCE($3, status),
            vat_number = COALESCE($4, vat_number),
            tax_id = COALESCE($5, tax_id),
            website = COALESCE($6, website),
            industry = COALESCE($7, industry),
            company_size = COALESCE($8, company_size),
            preferred_language = COALESCE($9, preferred_language),
            notes = COALESCE($10, notes),
            valid_until = COALESCE($11, valid_until)
        WHERE id = $1
        RETURNING
            id, customer_number, name, license_key, valid_until,
            is_on_premise, hardware_hash, plan_id, status,
            vat_number, tax_id, website, industry, company_size,
            preferred_language, notes, created_at, updated_at"#,
        id,
        payload.name,
        payload.status,
        payload.vat_number,
        payload.tax_id,
        payload.website,
        payload.industry,
        payload.company_size,
        payload.preferred_language,
        payload.notes,
        payload.valid_until
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(tenant))
}

async fn delete_tenant(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query!("DELETE FROM tenants WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Tenant not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn update_tenant_status(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<TenantDetail>, (StatusCode, String)> {
    let status = payload.get("status")
        .and_then(|v| v.as_str())
        .ok_or((StatusCode::BAD_REQUEST, "Status is required".to_string()))?;

    let tenant = sqlx::query_as!(
        TenantDetail,
        r#"UPDATE tenants SET status = $2 WHERE id = $1
        RETURNING
            id, customer_number, name, license_key, valid_until,
            is_on_premise, hardware_hash, plan_id, status,
            vat_number, tax_id, website, industry, company_size,
            preferred_language, notes, created_at, updated_at"#,
        id,
        status
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(tenant))
}

// ============================================================================
// PLAN HANDLERS
// ============================================================================

async fn switch_tenant_plan(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<SwitchPlanRequest>,
) -> Result<Json<TenantDetail>, (StatusCode, String)> {
    let tenant = sqlx::query_as!(
        TenantDetail,
        r#"UPDATE tenants SET plan_id = $2 WHERE id = $1
        RETURNING
            id, customer_number, name, license_key, valid_until,
            is_on_premise, hardware_hash, plan_id, status,
            vat_number, tax_id, website, industry, company_size,
            preferred_language, notes, created_at, updated_at"#,
        id,
        payload.plan_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(tenant))
}

async fn list_plans(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Plan>>, (StatusCode, String)> {
    let plans = sqlx::query_as!(
        Plan,
        "SELECT id, name, max_users, created_at FROM plans ORDER BY name"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(plans))
}

// ============================================================================
// ADDRESS HANDLERS
// ============================================================================

async fn list_tenant_addresses(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<TenantAddress>>, (StatusCode, String)> {
    let addresses = sqlx::query_as!(
        TenantAddress,
        r#"SELECT
            id, tenant_id, address_type, company_name, street,
            street_additional, postal_code, city, state, country,
            is_primary, created_at, updated_at
        FROM tenant_addresses
        WHERE tenant_id = $1
        ORDER BY is_primary DESC, created_at DESC"#,
        id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(addresses))
}

async fn create_tenant_address(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateAddressRequest>,
) -> Result<(StatusCode, Json<TenantAddress>), (StatusCode, String)> {
    let country = payload.country.unwrap_or_else(|| "DE".to_string());

    let address = sqlx::query_as!(
        TenantAddress,
        r#"INSERT INTO tenant_addresses (
            tenant_id, address_type, company_name, street, street_additional,
            postal_code, city, state, country, is_primary
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING
            id, tenant_id, address_type, company_name, street,
            street_additional, postal_code, city, state, country,
            is_primary, created_at, updated_at"#,
        id,
        payload.address_type,
        payload.company_name,
        payload.street,
        payload.street_additional,
        payload.postal_code,
        payload.city,
        payload.state,
        country,
        payload.is_primary
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(address)))
}

async fn update_tenant_address(
    Extension(pool): Extension<PgPool>,
    Path((tenant_id, address_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateAddressRequest>,
) -> Result<Json<TenantAddress>, (StatusCode, String)> {
    let address = sqlx::query_as!(
        TenantAddress,
        r#"UPDATE tenant_addresses SET
            address_type = COALESCE($3, address_type),
            company_name = COALESCE($4, company_name),
            street = COALESCE($5, street),
            street_additional = COALESCE($6, street_additional),
            postal_code = COALESCE($7, postal_code),
            city = COALESCE($8, city),
            state = COALESCE($9, state),
            country = COALESCE($10, country),
            is_primary = COALESCE($11, is_primary)
        WHERE id = $1 AND tenant_id = $2
        RETURNING
            id, tenant_id, address_type, company_name, street,
            street_additional, postal_code, city, state, country,
            is_primary, created_at, updated_at"#,
        address_id,
        tenant_id,
        payload.address_type,
        payload.company_name,
        payload.street,
        payload.street_additional,
        payload.postal_code,
        payload.city,
        payload.state,
        payload.country,
        payload.is_primary
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(address))
}

async fn delete_tenant_address(
    Extension(pool): Extension<PgPool>,
    Path((tenant_id, address_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query!(
        "DELETE FROM tenant_addresses WHERE id = $1 AND tenant_id = $2",
        address_id,
        tenant_id
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Address not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// CONTACT HANDLERS
// ============================================================================

async fn list_tenant_contacts(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<TenantContact>>, (StatusCode, String)> {
    let contacts = sqlx::query_as!(
        TenantContact,
        r#"SELECT
            id, tenant_id, contact_type, salutation, first_name, last_name,
            position, email, phone, mobile, fax, is_primary, notes,
            created_at, updated_at
        FROM tenant_contacts
        WHERE tenant_id = $1
        ORDER BY is_primary DESC, created_at DESC"#,
        id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(contacts))
}

async fn create_tenant_contact(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateContactRequest>,
) -> Result<(StatusCode, Json<TenantContact>), (StatusCode, String)> {
    let contact = sqlx::query_as!(
        TenantContact,
        r#"INSERT INTO tenant_contacts (
            tenant_id, contact_type, salutation, first_name, last_name,
            position, email, phone, mobile, fax, is_primary, notes
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING
            id, tenant_id, contact_type, salutation, first_name, last_name,
            position, email, phone, mobile, fax, is_primary, notes,
            created_at, updated_at"#,
        id,
        payload.contact_type,
        payload.salutation,
        payload.first_name,
        payload.last_name,
        payload.position,
        payload.email,
        payload.phone,
        payload.mobile,
        payload.fax,
        payload.is_primary,
        payload.notes
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(contact)))
}

async fn update_tenant_contact(
    Extension(pool): Extension<PgPool>,
    Path((tenant_id, contact_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdateContactRequest>,
) -> Result<Json<TenantContact>, (StatusCode, String)> {
    let contact = sqlx::query_as!(
        TenantContact,
        r#"UPDATE tenant_contacts SET
            contact_type = COALESCE($3, contact_type),
            salutation = COALESCE($4, salutation),
            first_name = COALESCE($5, first_name),
            last_name = COALESCE($6, last_name),
            position = COALESCE($7, position),
            email = COALESCE($8, email),
            phone = COALESCE($9, phone),
            mobile = COALESCE($10, mobile),
            fax = COALESCE($11, fax),
            is_primary = COALESCE($12, is_primary),
            notes = COALESCE($13, notes)
        WHERE id = $1 AND tenant_id = $2
        RETURNING
            id, tenant_id, contact_type, salutation, first_name, last_name,
            position, email, phone, mobile, fax, is_primary, notes,
            created_at, updated_at"#,
        contact_id,
        tenant_id,
        payload.contact_type,
        payload.salutation,
        payload.first_name,
        payload.last_name,
        payload.position,
        payload.email,
        payload.phone,
        payload.mobile,
        payload.fax,
        payload.is_primary,
        payload.notes
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(contact))
}

async fn delete_tenant_contact(
    Extension(pool): Extension<PgPool>,
    Path((tenant_id, contact_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query!(
        "DELETE FROM tenant_contacts WHERE id = $1 AND tenant_id = $2",
        contact_id,
        tenant_id
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Contact not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// PAYMENT HANDLERS
// ============================================================================

async fn list_tenant_payments(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<TenantPayment>>, (StatusCode, String)> {
    let payments = sqlx::query_as!(
        TenantPayment,
        r#"SELECT
            id, tenant_id, payment_method, payment_status, amount, currency,
            billing_period_start, billing_period_end, due_date, paid_at,
            invoice_number, notes, created_at, updated_at
        FROM tenant_payments
        WHERE tenant_id = $1
        ORDER BY created_at DESC"#,
        id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(payments))
}

async fn create_tenant_payment(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreatePaymentRequest>,
) -> Result<(StatusCode, Json<TenantPayment>), (StatusCode, String)> {
    let currency = payload.currency.unwrap_or_else(|| "EUR".to_string());

    let payment = sqlx::query_as!(
        TenantPayment,
        r#"INSERT INTO tenant_payments (
            tenant_id, payment_method, payment_status, amount, currency,
            billing_period_start, billing_period_end, due_date,
            invoice_number, notes
        ) VALUES ($1, $2, 'pending', $3, $4, $5, $6, $7, $8, $9)
        RETURNING
            id, tenant_id, payment_method, payment_status, amount, currency,
            billing_period_start, billing_period_end, due_date, paid_at,
            invoice_number, notes, created_at, updated_at"#,
        id,
        payload.payment_method,
        payload.amount,
        currency,
        payload.billing_period_start,
        payload.billing_period_end,
        payload.due_date,
        payload.invoice_number,
        payload.notes
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(payment)))
}

async fn update_tenant_payment(
    Extension(pool): Extension<PgPool>,
    Path((tenant_id, payment_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<UpdatePaymentRequest>,
) -> Result<Json<TenantPayment>, (StatusCode, String)> {
    let payment = sqlx::query_as!(
        TenantPayment,
        r#"UPDATE tenant_payments SET
            payment_status = COALESCE($3, payment_status),
            paid_at = COALESCE($4, paid_at),
            notes = COALESCE($5, notes)
        WHERE id = $1 AND tenant_id = $2
        RETURNING
            id, tenant_id, payment_method, payment_status, amount, currency,
            billing_period_start, billing_period_end, due_date, paid_at,
            invoice_number, notes, created_at, updated_at"#,
        payment_id,
        tenant_id,
        payload.payment_status,
        payload.paid_at,
        payload.notes
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(payment))
}

async fn delete_tenant_payment(
    Extension(pool): Extension<PgPool>,
    Path((tenant_id, payment_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query!(
        "DELETE FROM tenant_payments WHERE id = $1 AND tenant_id = $2",
        payment_id,
        tenant_id
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Payment not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// STATISTICS HANDLERS
// ============================================================================

async fn get_statistics(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<TenantStatistics>, (StatusCode, String)> {
    // Total tenants
    let total_tenants: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tenants")
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Active tenants
    let active_tenants: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tenants WHERE status = 'active'")
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Trial tenants
    let trial_tenants: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tenants WHERE status = 'trial'")
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Suspended tenants
    let suspended_tenants: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tenants WHERE status = 'suspended'")
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Total users
    let total_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Tenants by plan
    let tenants_by_plan = sqlx::query_as!(
        TenantsByPlan,
        r#"SELECT p.name as plan_name, COUNT(t.id) as count
        FROM plans p
        LEFT JOIN tenants t ON p.id = t.plan_id
        GROUP BY p.name
        ORDER BY count DESC"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(TenantStatistics {
        total_tenants,
        active_tenants,
        trial_tenants,
        suspended_tenants,
        total_users,
        tenants_by_plan,
    }))
}
