use std::collections::HashMap;

pub struct Config {
    pub db_path: String,
    pub sandbox_dir: String,
    pub claims_dir: String,
    pub server_addr: String,
    pub log_level: String,
    pub github_token: Option<String>,
    pub temp_dir: String,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            db_path: env_or_default("HIREBRIDGE_DB_PATH", "hirebridge.db"),
            sandbox_dir: env_or_default("HIREBRIDGE_SANDBOX_DIR", "./sandbox"),
            claims_dir: env_or_default("HIREBRIDGE_CLAIMS_DIR", "./claims"),
            server_addr: env_or_default("HIREBRIDGE_SERVER_ADDR", "127.0.0.1:8080"),
            log_level: env_or_default("HIREBRIDGE_LOG_LEVEL", "info"),
            github_token: std::env::var("GITHUB_TOKEN").ok(),
            temp_dir: env_or_default("HIREBRIDGE_TEMP_DIR", "/tmp/hirebridge"),
        }
    }

    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("db_path".to_string(), self.db_path.clone());
        map.insert("sandbox_dir".to_string(), self.sandbox_dir.clone());
        map.insert("claims_dir".to_string(), self.claims_dir.clone());
        map.insert("server_addr".to_string(), self.server_addr.clone());
        map.insert("log_level".to_string(), self.log_level.clone());
        map.insert("temp_dir".to_string(), self.temp_dir.clone());
        if let Some(ref token) = self.github_token {
            map.insert("github_token".to_string(), "***".to_string());
        }
        map
    }
}

fn env_or_default(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}