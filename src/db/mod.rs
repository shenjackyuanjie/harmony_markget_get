use crate::model::{AppInfo, AppMetric, AppRating, AppRaw, RawJsonData, RawRatingData};

use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};

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
        raw_data: &RawJsonData,
        raw_star: Option<&RawRatingData>,
    ) -> Result<(bool, bool)> {
        // 转换原始JSON数据用于比较
        let raw_json = AppRaw::from_raw_datas(raw_data, raw_star);
        let insert_data = if self
            .is_same_data(&raw_data.app_id, &raw_json.raw_json_data)
            .await?
        {
            println!(
                "{}",
                format!(
                    "基本数据与最后一条记录相同，跳过插入: {} ({})",
                    raw_data.app_id, raw_data.name
                )
                .bright_black()
            );
            false
        } else {
            // 检查应用是否已存在，如果存在则保留原有的 created_at 时间
            let existing_created_at = self.get_app_created_at(&raw_data.app_id).await?;
            let mut app_info: AppInfo = raw_data.into();

            // 如果应用已存在，使用原有的 created_at 时间
            if let Some(created_at) = existing_created_at {
                app_info.created_at = created_at;
            }

            // 转换并保存应用信息
            self.insert_app_info(&app_info).await?;

            // 保存指标信息
            let app_metric = AppMetric::from_raw_data(raw_data);
            self.insert_app_metric(&app_metric).await?;
            true
        };
        // 保存评分信息（如果有）
        let insert_rate = if let Some(raw_star) = raw_star {
            let value = serde_json::to_value(raw_star).unwrap();
            if self.is_same_rating(&raw_data.app_id, &value).await? {
                println!(
                    "{}",
                    format!(
                        "评分数据与最后一条记录相同，跳过评分信息保存: {} ({})",
                        raw_data.app_id, raw_data.name
                    )
                    .bright_black()
                );
                false
            } else {
                let app_rating = AppRating::from_raw_star(raw_data, raw_star);
                self.insert_app_rating(&app_rating).await?;
                true
            }
        } else {
            println!(
                "{}",
                format!(
                    "评分数据缺失，跳过评分信息保存: {} ({})",
                    raw_data.app_id, raw_data.name
                )
                .yellow()
            );
            false
        };

        if insert_data || insert_rate {
            // 保存原始JSON数据
            self.insert_raw_data(&raw_json).await?;
            println!(
                "{}",
                format!("应用数据保存成功: {} ({})", raw_data.app_id, raw_data.name).green()
            );
        }
        Ok((insert_data, insert_rate))
    }
}
