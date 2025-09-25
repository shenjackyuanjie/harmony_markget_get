use crate::model::{AppInfo, AppMetric, ShortAppInfo};
use anyhow::Result;
use chrono::{DateTime, Local};
use serde_json::Value;
use sqlx::Row;
use std::ops::Range;

use crate::db::Database;

use crate::db::PaginatedAppInfo;

impl Database {
    /// 检查应用是否已存在
    pub async fn app_exists(&self, app_id: &str) -> Result<bool> {
        const QUERY: &str = "SELECT COUNT(*) FROM app_info WHERE app_id = $1";
        let count: i64 = sqlx::query(QUERY)
            .bind(app_id)
            .fetch_one(&self.pool)
            .await?
            .get(0);

        Ok(count > 0)
    }

    /// 获取指定应用的最后一条原始JSON数据
    pub async fn get_last_raw_json_data(&self, app_id: &str) -> Result<Option<Value>> {
        const QUERY: &str = r#"
            SELECT raw_json_data
            FROM app_raw
            WHERE app_id = $1
            ORDER BY created_at DESC
            LIMIT 1
        "#;

        let result = sqlx::query(QUERY)
            .bind(app_id)
            .fetch_optional(&self.pool)
            .await?;

        match result {
            Some(row) => {
                let raw_json: Value = row.get("raw_json_data");
                Ok(Some(raw_json))
            }
            None => Ok(None),
        }
    }

    /// 获取指定应用的 created_at 时间
    pub async fn get_app_created_at(&self, app_id: &str) -> Result<Option<DateTime<Local>>> {
        const QUERY: &str = r#"
            SELECT created_at
            FROM app_info
            WHERE app_id = $1
        "#;

        let result = sqlx::query(QUERY)
            .bind(app_id)
            .fetch_optional(&self.pool)
            .await?;

        match result {
            Some(row) => {
                let created_at: DateTime<Local> = row.get("created_at");
                Ok(Some(created_at))
            }
            None => Ok(None),
        }
    }

    /// 获取指定应用的最后一条原始JSON数据
    pub async fn get_last_raw_json_star(&self, app_id: &str) -> Result<Option<Value>> {
        const QUERY: &str = r#"
            SELECT raw_json_star
            FROM app_raw
            WHERE app_id = $1
            ORDER BY created_at DESC
            LIMIT 1
        "#;

        let result = sqlx::query(QUERY)
            .bind(app_id)
            .fetch_optional(&self.pool)
            .await?;

        match result {
            Some(row) => {
                let raw_json: Value = row.get("raw_json_star");
                Ok(Some(raw_json))
            }
            None => Ok(None),
        }
    }

    /// 检查新数据是否与最后一条数据相同
    pub async fn is_same_data(&self, app_id: &str, new_data: &Value) -> Result<bool> {
        if let Some(last_data) = self.get_last_raw_json_data(app_id).await? {
            Ok(last_data == *new_data)
        } else {
            Ok(false)
        }
    }

    /// 检查评分是否相同
    pub async fn is_same_rating(&self, app_id: &str, new_rating: &Value) -> Result<bool> {
        if let Some(last_rating) = self.get_last_raw_json_star(app_id).await? {
            Ok(last_rating == *new_rating)
        } else {
            Ok(false)
        }
    }

    /// 获取数据库中所有的 pkg_name
    pub async fn get_all_pkg_names(&self) -> Result<Vec<String>> {
        const QUERY: &str = "SELECT DISTINCT pkg_name FROM app_info WHERE pkg_name IS NOT NULL";

        let rows = sqlx::query(QUERY).fetch_all(&self.pool).await?;

        let pkg_names = rows
            .into_iter()
            .map(|row| row.get::<String, _>("pkg_name"))
            .collect();

        Ok(pkg_names)
    }

    /// 获取数据库中所有的 app_id
    pub async fn get_all_app_ids(&self) -> Result<Vec<String>> {
        const QUERY: &str = "SELECT DISTINCT app_id FROM app_info WHERE app_id IS NOT NULL";

        let rows = sqlx::query(QUERY).fetch_all(&self.pool).await?;

        let app_ids = rows
            .into_iter()
            .map(|row| row.get::<String, _>("app_id"))
            .collect();

        Ok(app_ids)
    }

    /// 分页查询 app_info 数据，按照创建时间排序
    ///
    /// # 参数
    /// - `range`: 范围参数，例如 0..10 表示获取前10条记录
    ///
    /// # 示例
    /// ```rust
    /// let db = Database::new("postgres://...", 5).await?;
    /// let apps = db.get_app_info_paginated(0..10).await?;
    /// println!("获取到 {} 条应用信息", apps.len());
    /// ```
    pub async fn get_app_info_paginated(&self, range: Range<u32>) -> Result<Vec<AppInfo>> {
        const QUERY: &str = r#"
        SELECT ai.*
        FROM app_info ai
        ORDER BY ai.created_at DESC
        LIMIT $1 OFFSET $2
        "#;

        let limit = (range.end - range.start) as i64;
        let offset = range.start as i64;

        let rows = sqlx::query(QUERY)
            .bind(limit) // LIMIT
            .bind(offset) // OFFSET - 修正了参数顺序
            .fetch_all(&self.pool)
            .await?;

        let mut app_infos = Vec::new();
        for row in rows {
            let app_info = AppInfo {
                app_id: row.get("app_id"),
                alliance_app_id: row.get("alliance_app_id"),
                name: row.get("name"),
                pkg_name: row.get("pkg_name"),
                dev_id: row.get("dev_id"),
                developer_name: row.get("developer_name"),
                dev_en_name: row.get("dev_en_name"),
                supplier: row.get("supplier"),
                kind_id: row.get("kind_id"),
                kind_name: row.get("kind_name"),
                tag_name: row.get("tag_name"),
                kind_type_id: row.get("kind_type_id"),
                kind_type_name: row.get("kind_type_name"),
                icon_url: row.get("icon_url"),
                brief_desc: row.get("brief_desc"),
                description: row.get("description"),
                privacy_url: row.get("privacy_url"),
                ctype: row.get("ctype"),
                detail_id: row.get("detail_id"),
                app_level: row.get("app_level"),
                jocat_id: row.get("jocat_id"),
                iap: row.get("iap"),
                hms: row.get("hms"),
                tariff_type: row.get("tariff_type"),
                packing_type: row.get("packing_type"),
                order_app: row.get("order_app"),
                denpend_gms: row.get("denpend_gms"),
                denpend_hms: row.get("denpend_hms"),
                force_update: row.get("force_update"),
                img_tag: row.get("img_tag"),
                is_pay: row.get("is_pay"),
                is_disciplined: row.get("is_disciplined"),
                is_shelves: row.get("is_shelves"),
                submit_type: row.get("submit_type"),
                delete_archive: row.get("delete_archive"),
                charging: row.get("charging"),
                button_grey: row.get("button_grey"),
                app_gift: row.get("app_gift"),
                free_days: row.get("free_days"),
                pay_install_type: row.get("pay_install_type"),
                created_at: row.get("created_at"),
            };
            app_infos.push(app_info);
        }

        Ok(app_infos)
    }

    /// 分页查询 app_info 数据（增强版），返回分页信息和数据
    ///
    /// # 参数
    /// - `page`: 页码（从1开始）
    /// - `page_size`: 每页大小
    ///
    /// # 示例
    /// ```rust
    /// let db = Database::new("postgres://...", 5).await?;
    /// let result = db.get_app_info_paginated_enhanced(1, 10).await?;
    /// println!("第 {} 页，共 {} 页，总计 {} 条记录",
    ///     result.page, result.total_pages, result.total_count);
    /// ```
    pub async fn get_app_info_paginated_enhanced(
        &self,
        page: u32,
        page_size: u32,
    ) -> Result<PaginatedAppInfo<AppInfo>> {
        let total_count = self.get_app_info_count().await?;
        let total_pages = if page_size == 0 {
            0
        } else {
            ((total_count as f32 / page_size as f32).ceil()) as u32
        };
        let offset = (page.saturating_sub(1)) * page_size;

        let data = self
            .get_app_info_paginated(offset..(offset + page_size))
            .await?;

        Ok(PaginatedAppInfo {
            data,
            total_count,
            page,
            page_size,
            total_pages,
        })
    }

    /// 同 get_app_info_paginated_enhanced
    ///
    /// 但是数据简洁一些
    pub async fn get_app_info_paginated_short(
        &self,
        page: u32,
        page_size: u32,
    ) -> Result<PaginatedAppInfo<ShortAppInfo>> {
        let detail_info = self
            .get_app_info_paginated_enhanced(page, page_size)
            .await?;
        let data = detail_info
            .data
            .into_iter()
            .map(|app| ShortAppInfo::from(&app))
            .collect();

        Ok(PaginatedAppInfo {
            data,
            total_count: detail_info.total_count,
            page,
            page_size,
            total_pages: detail_info.total_pages,
        })
    }

    /// 获取数据库内所有应用数量
    pub async fn count_all_apps(&self) -> Result<u32> {
        const QUERY: &str = "SELECT COUNT(*) FROM app_info";

        let count: i64 = sqlx::query(QUERY).fetch_one(&self.pool).await?.get(0);

        Ok(count as u32)
    }

    /// 获取数据库内应用数量
    pub async fn count_apps(&self) -> Result<u32> {
        const QUERY: &str = "SELECT COUNT(*) FROM app_info
        WHERE pkg_name NOT LIKE 'com.atomicservice.%';";

        let count: i64 = sqlx::query(QUERY).fetch_one(&self.pool).await?.get(0);

        Ok(count as u32)
    }

    /// 获取数据库内元应用数量
    pub async fn count_atomic_services(&self) -> Result<u32> {
        const QUERY: &str = "SELECT count(*) FROM app_info
        WHERE pkg_name LIKE 'com.atomicservice.%';";

        let count: i64 = sqlx::query(QUERY).fetch_one(&self.pool).await?.get(0);

        Ok(count as u32)
    }

    /// 获取 app_info 表中的总记录数
    ///
    /// # 示例
    /// ```rust
    /// let db = Database::new("postgres://...", 5).await?;
    /// let count = db.get_app_info_count().await?;
    /// println!("总共有 {} 条应用记录", count);
    /// ```
    pub async fn get_app_info_count(&self) -> Result<u32> {
        const QUERY: &str = "SELECT COUNT(*) FROM app_info";

        let count: i64 = sqlx::query(QUERY).fetch_one(&self.pool).await?.get(0);

        Ok(count as u32)
    }

    /// 获取指定 pkg_id 的所有 app_metric 信息
    pub async fn get_app_metrics_by_pkg_id(&self, pkg_id: &str) -> Result<Vec<AppMetric>> {
        const QUERY: &str = r#"
            SELECT
                am.id,
                am.app_id,
                am.version,
                am.version_code,
                am.size_bytes,
                am.sha256,
                am.info_score,
                am.info_rate_count,
                am.download_count,
                am.price,
                am.release_date,
                am.new_features,
                am.upgrade_msg,
                am.target_sdk,
                am.minsdk,
                am.compile_sdk_version,
                am.min_hmos_api_level,
                am.api_release_type,
                am.created_at
            FROM app_metrics am
            JOIN app_info ai ON am.app_id = ai.app_id
            WHERE ai.pkg_name = $1
            ORDER BY am.created_at DESC
        "#;

        let rows = sqlx::query(QUERY)
            .bind(pkg_id)
            .fetch_all(&self.pool)
            .await?;

        let mut app_metrics = Vec::new();
        for row in rows {
            let app_metric = AppMetric {
                id: row.get("id"),
                app_id: row.get("app_id"),
                version: row.get("version"),
                version_code: row.get("version_code"),
                size_bytes: row.get("size_bytes"),
                sha256: row.get("sha256"),
                info_score: row.get("info_score"),
                info_rate_count: row.get("info_rate_count"),
                download_count: row.get("download_count"),
                price: row.get("price"),
                release_date: row.get("release_date"),
                new_features: row.get("new_features"),
                upgrade_msg: row.get("upgrade_msg"),
                target_sdk: row.get("target_sdk"),
                minsdk: row.get("minsdk"),
                compile_sdk_version: row.get("compile_sdk_version"),
                min_hmos_api_level: row.get("min_hmos_api_level"),
                api_release_type: row.get("api_release_type"),
                created_at: row.get("created_at"),
            };
            app_metrics.push(app_metric);
        }

        Ok(app_metrics)
    }
}
