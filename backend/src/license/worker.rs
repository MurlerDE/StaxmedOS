use crate::config::AppConfig;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub async fn run_heartbeat_loop(config: Arc<AppConfig>) {
    let client = reqwest::Client::new();

    loop {
        if let Some(url) = &config.cloud_master_url {
            let endpoint = format!("{}/api/v1/system/heartbeat", url);
            
            // Hier senden wir die Box-Daten zum Mutterschiff
            let res = client.post(&endpoint)
                .json(&serde_json::json!({ "box_id": config.box_id }))
                .send()
                .await;

            match res {
                Ok(_) => println!("Heartbeat: Check-in bei Cloud erfolgreich."),
                Err(e) => println!("Heartbeat: Cloud-Master nicht erreichbar ({})", e),
            }
        }

        // Warte 24 Stunden bis zum nächsten Check (oder 1 Stunde zum Testen)
        sleep(Duration::from_secs(3600)).await;
    }
}
