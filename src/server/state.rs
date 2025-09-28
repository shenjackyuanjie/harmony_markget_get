use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{config::Config, db::Database};

/// 应用状态，包含数据库连接、HTTP客户端和配置
#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub client: Client,
    pub cfg: Config,
}

impl AppState {
    /// 创建新的应用状态
    pub fn new(db: Database, client: Client, cfg: Config) -> Self {
        Self { db, client, cfg }
    }
}

/// 用于API响应的统一格式
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiResponse {
    pub success: bool,
    pub data: serde_json::Value,
    pub total: Option<u32>,
    pub limit: Option<u32>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ApiResponse {
    /// 创建成功的API响应
    pub fn success<T: serde::Serialize>(data: T, total: Option<u32>, limit: Option<u32>) -> Self {
        Self {
            success: true,
            data: serde_json::to_value(data).unwrap_or_default(),
            total,
            limit,
            timestamp: chrono::Utc::now(),
        }
    }

    /// 创建失败的API响应
    pub fn error(error_msg: impl ToString) -> Self {
        Self {
            success: false,
            data: serde_json::json!({"error": error_msg.to_string()}),
            total: None,
            limit: None,
            timestamp: chrono::Utc::now(),
        }
    }
    /// 创建失败的API响应
    pub fn error_with_value(error: serde_json::Value) -> Self {
        Self {
            success: false,
            data: error,
            total: None,
            limit: None,
            timestamp: chrono::Utc::now(),
        }
    }
}

/// 用于排行API的查询参数
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RankingQuery {
    pub limit: Option<u32>,
    pub exclude_pattern: Option<String>,
    pub time_range: Option<String>,
}

impl Default for RankingQuery {
    fn default() -> Self {
        Self {
            limit: Some(10),
            exclude_pattern: None,
            time_range: None,
        }
    }
}

/// 用于查询应用列表的查询参数
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct AppListQuery {
    pub sort: Option<String>,
    pub desc: Option<bool>,
    pub search: Option<String>,
    pub page_size: Option<u32>,
    pub detail: Option<bool>,
}

impl AppListQuery {
    pub fn is_valid_sort(&self) -> bool {
        if let Some(sort_field) = &self.sort {
            matches!(
                sort_field.as_str(),
                "download_count"
                    | "ratings"
                    | "price"
                    | "size"
                    | "rating_count"
                    | "last_updated"
                    | "created_at"
            )
        } else {
            false
        }
    }

    pub fn sort_key(&self) -> String {
        if self.is_valid_sort() {
            self.sort.as_deref().unwrap_or("created_at").to_string()
        } else {
            "created_at".to_string()
        }
    }

    pub fn page_size(&self) -> u32 {
        self.page_size.unwrap_or(100)
    }

    pub fn detail(&self) -> bool {
        self.detail.unwrap_or(true)
    }
}
