use crate::model::{AppInfo, AppMetric, AppQuery, AppRating, RawJsonData, RawRatingData};

use anyhow::Result;
use chrono::{DateTime, Local};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use sqlx::{
    FromRow,
    postgres::{PgPool, PgPoolOptions},
};

pub mod insert;
pub mod query;
pub mod read_data;

/// 分页查询结果
#[derive(Debug, Deserialize, Serialize)]
pub struct PaginatedAppInfo<D> {
    pub data: Vec<D>,
    pub total_count: u32,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: PgPool,
}

#[derive(Debug, Clone)]
pub struct DbSearch {
    pub key: String,
    pub value: String,
    pub is_exact: bool,
}

#[derive(Debug, FromRow, serde::Deserialize, serde::Serialize)]
pub struct AppCounts {
    pub total: i64,
    pub apps: i64,
    pub atomic_services: i64,
}

impl DbSearch {
    pub fn new(key: String, value: String, is_exact: bool) -> Self {
        Self {
            key,
            value,
            is_exact,
        }
    }
    pub fn exact(key: String, value: String) -> Self {
        Self {
            key,
            value,
            is_exact: true,
        }
    }
    pub fn fuzzy(key: String, value: String) -> Self {
        Self {
            key,
            value,
            is_exact: false,
        }
    }
    /// exact: 不动
    /// not exact: %value%
    pub fn search_value(&self) -> String {
        if self.is_exact {
            self.value.clone()
        } else {
            format!("%{}%", self.value)
        }
    }
}

impl Database {
    /// 创建数据库连接池
    pub async fn new(database_url: &str, max_connect: u32) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connect)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    /// 保存应用数据到数据库
    /// 返回布尔值表示是否插入了新数据
    pub async fn save_app_data(
        &self,
        raw_data: &(RawJsonData, JsonValue),
        raw_rating: Option<&RawRatingData>,
        listed_at: Option<DateTime<Local>>,
        comment: Option<serde_json::Value>,
    ) -> Result<(bool, bool, bool)> {
        // 转换原始JSON数据用于比较
        let (raw_data, raw_value) = raw_data;
        let app_id = raw_data.app_id.clone();
        let query = AppQuery::app_id(&app_id);
        let exists = self.app_exists(&query).await;
        let insert_data = if exists && self.is_same_data(&query, raw_value).await {
            (false, false)
        } else {
            let mut app_info: AppInfo = raw_data.into();
            app_info.comment = comment;
            if let Some(listed_at) = listed_at {
                app_info.listed_at = listed_at;
            }

            // 转换并保存应用信息
            let info_new = if self.is_same_app_info(&query, &app_info).await {
                false
            } else {
                self.insert_app_info(&app_info).await?;
                println!(
                    "{}",
                    format!("插入新的 app info {} ({})", app_id, app_info.name).bright_green()
                );
                true
            };

            // 保存指标信息
            let app_metric = AppMetric::from_raw_data(raw_data);
            let metric_new = if self.is_same_app_metric(&query, &app_metric).await {
                false
            } else {
                self.insert_app_metric(&app_metric).await?;
                println!(
                    "{}",
                    format!(
                        "插入新的 app metric {} ({})",
                        app_metric.app_id, raw_data.name
                    )
                    .bright_green()
                );
                true
            };
            (info_new, metric_new)
        };
        // 保存评分信息（如果有）
        let insert_rate = if let Some(raw_star) = raw_rating {
            let value = serde_json::to_value(raw_star).unwrap();
            if self.is_same_rating(&query, &value).await {
                false
            } else {
                let app_rating = AppRating::from_raw_star(raw_data, raw_star);
                println!(
                    "{}",
                    format!("更新评分数据 {} ({})", app_id, raw_data.name).bright_green()
                );
                self.insert_app_rating(&app_rating).await?;
                true
            }
        } else {
            false
        };

        if insert_data.0 || insert_data.1 || insert_rate {
            // 保存原始JSON数据
            self.insert_raw_data(
                &app_id,
                raw_value,
                raw_rating.map(|r| serde_json::to_value(r).unwrap()),
            )
            .await?;
            println!(
                "{}",
                format!("应用数据保存成功: {} ({})", app_id, raw_data.name).green()
            );
        }
        Ok((insert_data.0, insert_data.1, insert_rate))
    }
}
