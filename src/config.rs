use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub packages: Vec<String>,
    pub locale: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub base_url: String,
    pub timeout_seconds: u64,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub app: AppConfig,
    pub api: ApiConfig,
    pub logging: LoggingConfig,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config_content = fs::read_to_string("config.toml")?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }

    pub fn database_url(&self) -> &str {
        &self.database.url
    }

    pub fn packages(&self) -> &[String] {
        &self.app.packages
    }

    pub fn locale(&self) -> &str {
        &self.app.locale
    }

    pub fn api_base_url(&self) -> &str {
        &self.api.base_url
    }

    pub fn api_timeout_seconds(&self) -> u64 {
        self.api.timeout_seconds
    }

    pub fn log_level(&self) -> &str {
        &self.logging.level
    }
}
