use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../frontend/src/app/generated/")]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub license_key: Option<String>,
    pub status: TenantStatus,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../frontend/src/app/generated/")]
pub enum TenantStatus {
    Active,
    Trial,
    Suspended,
    Canceled,
}

#[derive(TS, Serialize, Deserialize, Debug, Clone)]
#[ts(export, export_to = "../../../frontend/src/app/generated/")]
pub struct InventoryItem {
    pub id: Uuid,
    pub udi_di: String,
    pub brand_name: String,
    pub quantity: i32,
    pub batch_number: String,
}
