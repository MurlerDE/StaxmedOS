use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};

// ============================================================================
// AUTH MODULE TYPES
// ============================================================================

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct AuthResponse {
    pub token: String,
    pub username: String,
    pub role: String,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct UserInfo {
    pub username: String,
    pub role: String,
    pub user_id: String,
    pub tenant_id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub tenant_id: Option<Uuid>,
}

// ============================================================================
// TENANT/ADMIN MODULE TYPES
// ============================================================================

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct TenantListItem {
    pub id: Uuid,
    pub customer_number: Option<String>,
    pub name: String,
    pub status: Option<String>,
    pub plan_id: Option<Uuid>,
    pub plan_name: Option<String>,
    #[ts(type = "string")]
    pub valid_until: Option<DateTime<Utc>>,
    #[ts(type = "string")]
    pub created_at: Option<DateTime<Utc>>,
    pub user_count: Option<i64>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct TenantDetail {
    pub id: Uuid,
    pub customer_number: Option<String>,
    pub name: String,
    pub license_key: Option<String>,
    #[ts(type = "string")]
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
    #[ts(type = "string")]
    pub created_at: Option<DateTime<Utc>>,
    #[ts(type = "string")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
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
    #[ts(type = "string", optional)]
    pub valid_until: Option<DateTime<Utc>>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
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
    #[ts(type = "string", optional)]
    pub valid_until: Option<DateTime<Utc>>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct TenantQueryParams {
    pub status: Option<String>,
    pub plan_id: Option<Uuid>,
    pub search: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

// ============================================================================
// ADDRESS TYPES
// ============================================================================

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
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
    #[ts(type = "string")]
    pub created_at: Option<DateTime<Utc>>,
    #[ts(type = "string")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
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

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
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
// CONTACT TYPES
// ============================================================================

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
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
    #[ts(type = "string")]
    pub created_at: Option<DateTime<Utc>>,
    #[ts(type = "string")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
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

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
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
// PAYMENT TYPES
// ============================================================================

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct TenantPayment {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub payment_method: String,
    pub payment_status: String,
    #[ts(type = "string")]
    pub amount: String,
    pub currency: Option<String>,
    #[ts(type = "string")]
    pub billing_period_start: Option<NaiveDate>,
    #[ts(type = "string")]
    pub billing_period_end: Option<NaiveDate>,
    #[ts(type = "string")]
    pub due_date: Option<NaiveDate>,
    #[ts(type = "string")]
    pub paid_at: Option<DateTime<Utc>>,
    pub invoice_number: Option<String>,
    pub notes: Option<String>,
    #[ts(type = "string")]
    pub created_at: Option<DateTime<Utc>>,
    #[ts(type = "string")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct CreatePaymentRequest {
    pub payment_method: String,
    #[ts(type = "string")]
    pub amount: String,
    pub currency: Option<String>,
    #[ts(type = "string")]
    pub billing_period_start: Option<NaiveDate>,
    #[ts(type = "string")]
    pub billing_period_end: Option<NaiveDate>,
    #[ts(type = "string")]
    pub due_date: Option<NaiveDate>,
    pub invoice_number: Option<String>,
    pub notes: Option<String>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct UpdatePaymentRequest {
    pub payment_status: Option<String>,
    #[ts(type = "string")]
    pub paid_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

// ============================================================================
// PLAN TYPES
// ============================================================================

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct Plan {
    pub id: Uuid,
    pub name: String,
    pub max_users: Option<i32>,
    #[ts(type = "string")]
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct SwitchPlanRequest {
    pub plan_id: Uuid,
}

// ============================================================================
// STATISTICS TYPES
// ============================================================================

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct TenantStatistics {
    pub total_tenants: i64,
    pub active_tenants: i64,
    pub trial_tenants: i64,
    pub suspended_tenants: i64,
    pub total_users: i64,
    pub tenants_by_plan: Vec<TenantsByPlan>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct TenantsByPlan {
    pub plan_name: Option<String>,
    pub count: Option<i64>,
}

// ============================================================================
// USER MODULE TYPES
// ============================================================================

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct UserListItem {
    pub id: String,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
    pub tenant_id: Option<String>,
    #[ts(type = "string")]
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct UserDetail {
    pub id: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: String,
    pub is_active: bool,
    pub tenant_id: Option<String>,
    #[ts(type = "string")]
    pub created_at: DateTime<Utc>,
    pub roles: Vec<UserRoleInfo>,
    pub permissions: Vec<PermissionInfo>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct UserRoleInfo {
    pub role_id: String,
    pub role_name: String,
    pub display_name: String,
    #[ts(type = "string")]
    pub assigned_at: Option<DateTime<Utc>>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct PermissionInfo {
    pub permission_name: String,
    pub source: String,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role_ids: Option<Vec<String>>,
    pub tenant_id: Option<String>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct AssignRoleRequest {
    pub role_id: String,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct AssignPermissionRequest {
    pub permission_name: String,
    #[ts(type = "string")]
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct UpdatePasswordRequest {
    pub new_password: String,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct RoleListItem {
    pub id: String,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub is_system: Option<bool>,
    pub permission_count: i64,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct PermissionListItem {
    pub id: String,
    pub name: Option<String>,
    pub resource: Option<String>,
    pub action: Option<String>,
    pub scope: Option<String>,
    pub description: Option<String>,
}

// ============================================================================
// INVENTORY MODULE TYPES
// ============================================================================

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct GlobalProduct {
    pub udi_di: String,
    pub brand_name_de: Option<String>,
    pub manufacturer: Option<String>,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct InventoryItem {
    pub id: Uuid,
    pub udi_di: String,
    pub brand_name: String,
    pub quantity: i32,
    pub batch_number: String,
}

// ============================================================================
// LEGACY TYPES (kept for backwards compatibility)
// ============================================================================

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub license_key: Option<String>,
    pub status: TenantStatus,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub enum TenantStatus {
    Active,
    Trial,
    Suspended,
    Canceled,
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_bindings() {
        // Auth types
        LoginRequest::export().expect("Failed to export LoginRequest");
        AuthResponse::export().expect("Failed to export AuthResponse");
        UserInfo::export().expect("Failed to export UserInfo");
        RegisterRequest::export().expect("Failed to export RegisterRequest");

        // Tenant types
        TenantListItem::export().expect("Failed to export TenantListItem");
        TenantDetail::export().expect("Failed to export TenantDetail");
        CreateTenantRequest::export().expect("Failed to export CreateTenantRequest");
        UpdateTenantRequest::export().expect("Failed to export UpdateTenantRequest");
        TenantQueryParams::export().expect("Failed to export TenantQueryParams");

        // Address types
        TenantAddress::export().expect("Failed to export TenantAddress");
        CreateAddressRequest::export().expect("Failed to export CreateAddressRequest");
        UpdateAddressRequest::export().expect("Failed to export UpdateAddressRequest");

        // Contact types
        TenantContact::export().expect("Failed to export TenantContact");
        CreateContactRequest::export().expect("Failed to export CreateContactRequest");
        UpdateContactRequest::export().expect("Failed to export UpdateContactRequest");

        // Payment types
        TenantPayment::export().expect("Failed to export TenantPayment");
        CreatePaymentRequest::export().expect("Failed to export CreatePaymentRequest");
        UpdatePaymentRequest::export().expect("Failed to export UpdatePaymentRequest");

        // Plan types
        Plan::export().expect("Failed to export Plan");
        SwitchPlanRequest::export().expect("Failed to export SwitchPlanRequest");

        // Statistics types
        TenantStatistics::export().expect("Failed to export TenantStatistics");
        TenantsByPlan::export().expect("Failed to export TenantsByPlan");

        // User types
        UserListItem::export().expect("Failed to export UserListItem");
        UserDetail::export().expect("Failed to export UserDetail");
        UserRoleInfo::export().expect("Failed to export UserRoleInfo");
        PermissionInfo::export().expect("Failed to export PermissionInfo");
        CreateUserRequest::export().expect("Failed to export CreateUserRequest");
        UpdateUserRequest::export().expect("Failed to export UpdateUserRequest");
        AssignRoleRequest::export().expect("Failed to export AssignRoleRequest");
        AssignPermissionRequest::export().expect("Failed to export AssignPermissionRequest");
        UpdatePasswordRequest::export().expect("Failed to export UpdatePasswordRequest");
        RoleListItem::export().expect("Failed to export RoleListItem");
        PermissionListItem::export().expect("Failed to export PermissionListItem");

        // Inventory types
        GlobalProduct::export().expect("Failed to export GlobalProduct");
        InventoryItem::export().expect("Failed to export InventoryItem");

        // Legacy types
        Tenant::export().expect("Failed to export Tenant");
        TenantStatus::export().expect("Failed to export TenantStatus");
    }
}
