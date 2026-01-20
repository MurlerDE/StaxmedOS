use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

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

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct InventoryItem {
    pub id: Uuid,
    pub udi_di: String,
    pub brand_name: String,
    pub quantity: i32,
    pub batch_number: String,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../../frontend/src/app/core/models/")]
pub struct UserInfo {
    pub username: String,
    pub role: String,
    pub user_id: String,
    pub tenant_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_bindings() {
        // This test triggers the TypeScript export generation
        Tenant::export().expect("Failed to export Tenant");
        TenantStatus::export().expect("Failed to export TenantStatus");
        InventoryItem::export().expect("Failed to export InventoryItem");
        UserInfo::export().expect("Failed to export UserInfo");
    }
}
