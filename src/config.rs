use serde::Deserialize;
use std::{fs, sync::OnceLock};
use tracing::{Level, event};

use crate::sync::identy_id::GLOBAL_IDENTITY_ID;

pub static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    GLOBAL_CONFIG.get().unwrap()
}

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
    pub info_url: String,
    /// 应用市场的 detail api
    pub detail_url: String,
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
    pub fn load() -> anyhow::Result<&'static Self> {
        let config_content = fs::read_to_string("config.toml")?;
        event!(Level::INFO, "config.toml loaded");
        let config: Config = toml::from_str(&config_content)?;
        event!(Level::INFO, "config.toml parsed");
        GLOBAL_IDENTITY_ID.get_identity_id();
        std::thread::sleep(std::time::Duration::from_secs(1));
        Ok(GLOBAL_CONFIG.get_or_init(|| config))
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

    pub fn api_info_url(&self) -> &str {
        &self.api.info_url
    }

    pub fn api_detail_url(&self) -> &str {
        &self.api.detail_url
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
