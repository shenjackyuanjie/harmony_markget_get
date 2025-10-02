use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    config::{Config, get_config},
    db::{Database, DbSearch},
};

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
    pub search_key: Option<String>,
    pub search_value: Option<String>,
    pub search_exact: Option<bool>,
    pub page_size: Option<u32>,
    pub detail: Option<bool>,
}

impl AppListQuery {
    pub fn is_valid_sort(&self) -> bool {
        if let Some(sort_field) = &self.sort {
            matches!(
                sort_field.as_str(),
                "download_count"
                    | "average_rating"
                    | "price"
                    | "size_bytes"
                    | "rating_count"
                    | "metrics_created_at"
                    | "created_at"
                    | "total_star_rating_count"
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
        let page = self.page_size.unwrap_or(100);
        let limit = get_config().api.page_limit.unwrap_or(page);
        if page > limit {
            limit
        } else {
            page
        }
    }

    pub fn detail(&self) -> bool {
        self.detail.unwrap_or(true)
    }

    fn is_valid_search(&self) -> bool {
        if let Some(key) = &self.search_key {
            matches!(
                key.as_str(),
                "name"
                    | "pkg_name"
                    | "app_id"
                    | "developer_name"
                    | "developer_en_name"
            )
        } else {
            false
        }
    }

    pub fn search_option(&self) -> Option<DbSearch> {
        if !self.is_valid_search() {
            return None;
        }
        match (&self.search_key, &self.search_value) {
            (Some(key), Some(value)) => Some(DbSearch::new(
                key.clone(),
                value.clone(),
                self.search_exact.unwrap_or(false),
            )),
            _ => None,
        }
    }
}
