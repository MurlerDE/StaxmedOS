// Simple test to verify .env loading
use dotenvy;
use std::env;

fn main() {
    println!("=================================");
    println!("  StaxmedOS .env Loading Test");
    println!("=================================\n");

    println!("Current directory: {:?}\n", env::current_dir().unwrap());

    // Try loading .env
    println!("Attempting to load .env...");
    let env_result = dotenvy::dotenv()
        .or_else(|_| dotenvy::from_path("backend/.env"))
        .or_else(|_| dotenvy::from_path(".env"));

    match env_result {
        Ok(path) => println!("✓ Loaded .env from: {:?}\n", path),
        Err(e) => println!("✗ Failed to load .env: {}\n", e),
    }

    // Check required variables
    println!("Checking required environment variables:\n");

    let vars_to_check = vec![
        "DATABASE_URL",
        "JWT_SECRET",
        "STAX_MODE",
    ];

    let mut all_ok = true;

    for var in vars_to_check {
        match env::var(var) {
            Ok(value) => {
                let display_value = if value.len() > 20 {
                    format!("{}... (length: {})", &value[..20], value.len())
                } else if var.contains("SECRET") || var.contains("PASSWORD") {
                    "***".to_string()
                } else {
                    value.clone()
                };
                println!("✓ {} = {}", var, display_value);
            }
            Err(_) => {
                println!("✗ {} = NOT SET", var);
                all_ok = false;
            }
        }
    }

    println!("\n=================================");
    if all_ok {
        println!("✓ All required variables are set!");
        println!("  Backend should start successfully.");
    } else {
        println!("✗ Some variables are missing!");
        println!("  Backend will fail to start.");
    }
    println!("=================================");
}
