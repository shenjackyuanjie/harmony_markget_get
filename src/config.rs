use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connect: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub packages: Vec<String>,
    pub locale: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApiConfig {
    /// 华为应用市场 API 基础 URL
    pub base_url: String,
    /// API 请求超时时间（秒）
    pub timeout_seconds: u64,
    /// 数据更新间隔 (秒)
    pub interval_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServeConfig {
    pub url: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub app: AppConfig,
    pub api: ApiConfig,
    pub serve: ServeConfig,
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

    pub fn db_max_connect(&self) -> u32 {
        self.database.max_connect
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

    pub fn api_interval(&self) -> u64 {
        self.api.interval_seconds
    }

    pub fn serve_url(&self) -> &str {
        &self.serve.url
    }

    pub fn serve_port(&self) -> u16 {
        self.serve.port
    }
}
