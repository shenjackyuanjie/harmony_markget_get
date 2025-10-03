use anyhow::Context;
use serde::Deserialize;
use std::{fs, sync::OnceLock};
use tracing::{Level, event};

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
    pub api_url: String,
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
        let config_content =
            fs::read_to_string("config.toml").with_context(|| "无法读取 config.toml 配置文件")?;
        event!(Level::INFO, "config.toml loaded");
        let config: Config =
            toml::from_str(&config_content).with_context(|| "无法解析 config.toml 配置文件")?;
        event!(Level::INFO, "config.toml parsed");
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

    pub fn api_url(&self) -> &str {
        &self.api.api_url
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
