pub env;

#[derive(Debug, Clone)]
pub enum StaxMode {
    SaaS,
    StaxBox,
}

pub struct AppConfig {
    pub mode: StaxMode,
    pub cloud_master_url: Option<String>,
    pub box_id: Option<String>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let mode_str = env::var("STAX_MODE").unwrap_or_else(|_| "SAAS".into());
        Self {
            mode: if mode_str == "STAXBOX" { StaxMode::StaxBox } else { StaxMode::SaaS },
            cloud_master_url: env::var("CLOUD_MASTER_URL").ok(),
            box_id: env::var("STAXBOX_ID").ok(),
        }
    }
}
