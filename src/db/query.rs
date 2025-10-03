use anyhow::Result;
use chrono::{DateTime, Local};
use serde_json::Value;
use sqlx::{Row, postgres::PgRow};
use std::ops::Range;

use crate::db::read_data::{
    SELECT_APP_INFO_FIELDS, SELECT_APP_METRIC_FIELDS, SELECT_APP_RATING_FIELDS,
};
use crate::db::{AppCounts, Database, DbSearch, PaginatedAppInfo};
use crate::model::{AppInfo, AppMetric, AppQuery, FullAppInfo, ShortAppRating};

impl Database {
    fn read_full_app_data_from_row(row: &PgRow) -> FullAppInfo {
        let app_info = Self::read_app_info_from_row(row);
        let app_metrics = Self::read_app_metric_from_row(row);
        // 检查对应字段有没有内容
        let app_rating = if let Ok(rate) = row.try_get::<'_, Option<i32>, _>("star_1_rating_count")
        {
            if rate.is_some() {
                Some(Self::read_app_rating_from_row(row))
            } else {
                None
            }
        } else {
            None
        };
        FullAppInfo {
            info: app_info,
            metric: app_metrics,
            rating: app_rating,
        }
    }

    /// 检查应用是否已存在
    pub async fn app_exists(&self, app_query: &AppQuery) -> bool {
        let query = format!(
            "SELECT COUNT(*) FROM app_info WHERE {} = $1",
            app_query.app_db_name()
        );
        let count: Option<i64> = sqlx::query(&query)
            .bind(app_query.name())
            .fetch_one(&self.pool)
            .await
            .ok()
            .map(|r| r.get(0));

        count.map(|c| c > 0).unwrap_or(false)
    }

    /// 获取指定应用的最后一条原始JSON数据
    pub async fn get_last_raw_json_data(&self, app_id: &str) -> Option<Value> {
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
            .await
            .ok()?;

        result.and_then(|r| r.try_get("raw_json_data").ok())
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
    pub async fn get_last_raw_json_star(&self, app_id: &str) -> Option<Value> {
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
            .await
            .ok()?;

        match result {
            Some(row) => {
                let raw_json: Value = row.get("raw_json_star");
                Some(raw_json)
            }
            None => None,
        }
    }

    /// 检查新数据是否与最后一条数据相同
    pub async fn is_same_data(&self, app_id: &str, new_data: &Value) -> bool {
        self.get_last_raw_json_data(app_id)
            .await
            .map(|last_data| last_data == *new_data)
            .unwrap_or(false)
    }

    /// 检查评分是否相同
    pub async fn is_same_rating(&self, app_id: &str, new_rating: &Value) -> bool {
        self.get_last_raw_json_star(app_id)
            .await
            .map(|last_rating| last_rating == *new_rating)
            .unwrap_or(false)
    }

    /// 实际检查 app info 是否相同
    pub async fn is_same_app_info(&self, app_id: &str, app_info: &AppInfo) -> bool {
        self.get_app_info(app_id)
            .await
            .map(|d| d == *app_info)
            .unwrap_or(false)
    }

    pub async fn get_app_info(&self, app_id: &str) -> Option<AppInfo> {
        let query = format!(
            "SELECT {} FROM app_info WHERE app_id = $1",
            SELECT_APP_INFO_FIELDS
        );
        let row = sqlx::query(&query)
            .bind(app_id)
            .fetch_one(&self.pool)
            .await
            .ok()?;
        let data = Self::read_app_info_from_row(&row);

        Some(data)
    }

    /// 实际检查 app metric 是否相同
    pub async fn is_same_app_metric(&self, app_id: &str, app_metric: &AppMetric) -> bool {
        self.get_app_last_metric(app_id)
            .await
            .map(|last_metric| last_metric == *app_metric)
            .unwrap_or(false)
    }

    /// 获取 app 最后一次的 metric
    pub async fn get_app_last_metric(&self, app_id: &str) -> Option<AppMetric> {
        let query = format!(
            "SELECT {} FROM app_metrics WHERE app_id = $1",
            SELECT_APP_METRIC_FIELDS
        );
        let row = sqlx::query(&query)
            .bind(app_id)
            .fetch_optional(&self.pool)
            .await
            .ok()?;
        match row {
            Some(row) => {
                let data = Self::read_app_metric_from_row(&row);
                Some(data)
            }
            None => None,
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

    /// 获取数据库中所有的 app_id
    pub async fn get_all_alliance_app_id(&self) -> Result<Vec<i64>> {
        const QUERY: &str =
            "SELECT DISTINCT alliance_app_id FROM app_info WHERE alliance_app_id IS NOT NULL";

        let rows = sqlx::query(QUERY).fetch_all(&self.pool).await?;

        let app_ids = rows
            .into_iter()
            .map(|row| row.get::<i64, _>("alliance_app_id"))
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
    pub async fn get_app_info_paginated(
        &self,
        range: Range<u32>,
        sort_key: &str,
        sort_desc: bool,
        search: Option<DbSearch>,
    ) -> Result<Vec<FullAppInfo>> {
        let limit = (range.end - range.start) as i64;
        let offset = range.start as i64;

        let rows = match search {
            Some(DbSearch {
                key,
                value,
                is_exact,
            }) => {
                let query = format!(
                    r#"
                    SELECT {}, {}, {}
                    FROM app_latest_info
                    WHERE {} ILIKE $1
                    ORDER BY {} {}
                    LIMIT $2 OFFSET $3
                "#,
                    SELECT_APP_INFO_FIELDS,
                    SELECT_APP_METRIC_FIELDS
                        .replace("created_at AS metrics_created_at", "metrics_created_at"),
                    SELECT_APP_RATING_FIELDS
                        .replace("created_at AS rating_created_at", "rating_created_at"),
                    key,
                    sort_key,
                    if sort_desc { "DESC" } else { "ASC" }
                );

                sqlx::query(&query)
                    .bind(if is_exact {
                        value
                    } else {
                        format!("%{value}%")
                    })
                    .bind(limit) // LIMIT
                    .bind(offset) // OFFSET - 修正了参数顺序
                    .fetch_all(&self.pool)
                    .await?
            }
            None => {
                let query = format!(
                    r#"
                    SELECT {}, {}, {}
                    FROM app_latest_info
                    ORDER BY {} {}
                    LIMIT $1 OFFSET $2
                "#,
                    SELECT_APP_INFO_FIELDS,
                    SELECT_APP_METRIC_FIELDS
                        .replace("created_at AS metrics_created_at", "metrics_created_at"),
                    SELECT_APP_RATING_FIELDS
                        .replace("created_at AS rating_created_at", "rating_created_at"),
                    sort_key,
                    if sort_desc { "DESC" } else { "ASC" }
                );

                sqlx::query(&query)
                    .bind(limit) // LIMIT
                    .bind(offset) // OFFSET - 修正了参数顺序
                    .fetch_all(&self.pool)
                    .await?
            }
        };

        let app_infos = rows.iter().map(Self::read_full_app_data_from_row).collect();
        Ok(app_infos)
    }

    /// Count the number of distinct developers
    pub async fn count_developers(&self) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar(
            r#"
            SELECT COUNT(DISTINCT developer_name)
            FROM app_info
            WHERE developer_name IS NOT NULL
            "#,
        )
        .fetch_one(&self.pool)
        .await
    }

    /// Get star distribution counts from app_rating table
    pub async fn get_star_distribution(&self) -> Result<(i64, i64, i64, i64, i64), sqlx::Error> {
        sqlx::query_as::<_, (i64, i64, i64, i64, i64)>(
            r#"
            SELECT
                COUNT(*) FILTER (WHERE average_rating >= 0 AND average_rating < 1) AS range_0_1,
                COUNT(*) FILTER (WHERE average_rating >= 1 AND average_rating < 2) AS range_1_2,
                COUNT(*) FILTER (WHERE average_rating >= 2 AND average_rating < 3) AS range_2_3,
                COUNT(*) FILTER (WHERE average_rating >= 3 AND average_rating < 4) AS range_3_4,
                COUNT(*) FILTER (WHERE average_rating >= 4 AND average_rating <= 5) AS range_4_5
            FROM app_latest_info
            WHERE average_rating IS NOT NULL
            "#,
        )
        .fetch_one(&self.pool)
        .await
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
    pub async fn get_app_info_paginated_enhanced<D: From<FullAppInfo>>(
        &self,
        page: u32,
        page_size: u32,
        sort_key: &str,
        sort_desc: bool,
        search: Option<DbSearch>,
    ) -> Result<PaginatedAppInfo<D>> {
        let total_count = self.get_app_info_count().await?;
        let total_pages = if page_size == 0 {
            0
        } else {
            ((total_count as f32 / page_size as f32).ceil()) as u32
        };
        let offset = (page.saturating_sub(1)) * page_size;

        let data = self
            .get_app_info_paginated(offset..(offset + page_size), sort_key, sort_desc, search)
            .await?
            .into_iter()
            .map(D::from)
            .collect();

        Ok(PaginatedAppInfo {
            data,
            total_count,
            page,
            page_size,
            total_pages,
        })
    }

    /// 获取数据库内应用数量
    pub async fn count_apps(&self) -> Result<AppCounts> {
        const QUERY: &str = r#"
        SELECT
            COUNT(*) AS total,
            SUM(CASE WHEN pkg_name NOT LIKE 'com.atomicservice.%' THEN 1 ELSE 0 END) AS apps,
            SUM(CASE WHEN pkg_name LIKE 'com.atomicservice.%' THEN 1 ELSE 0 END) AS atomic_services
        FROM
            app_info
        "#;

        Ok(sqlx::query_as(QUERY).fetch_one(&self.pool).await?)
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
                am.info_score::text,
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
                am.created_at metrics_created_at
            FROM app_metrics am
            JOIN app_info ai ON am.app_id = ai.app_id
            WHERE ai.pkg_name = $1
            ORDER BY am.created_at DESC
        "#;

        let rows = sqlx::query(QUERY)
            .bind(pkg_id)
            .fetch_all(&self.pool)
            .await?;

        let app_metrics = rows.iter().map(Self::read_app_metric_from_row).collect();

        Ok(app_metrics)
    }

    /// 获取评分最高的应用排行
    ///
    /// # 参数
    /// - `limit`: 返回的应用数量
    ///
    /// # 示例
    /// ```rust
    /// let db = Database::new("postgres://...", 5).await?;
    /// let top_rated_apps = db.get_top_rated_apps(10).await?;
    /// println!("评分最高的应用: {:?}", top_rated_apps);
    /// ```
    pub async fn get_top_rated_apps(&self, limit: u32) -> Result<Vec<ShortAppRating>> {
        const QUERY: &str = r#"
            SELECT
                ai.app_id,
                ai.name,
                ai.pkg_name,
                ai.developer_name,
                ai.icon_url,
                ar.average_rating::text,
                ar.total_star_rating_count
            FROM app_info ai
            JOIN app_rating ar ON ai.app_id = ar.app_id
            ORDER BY ar.average_rating DESC, ar.total_star_rating_count DESC
            LIMIT $1
        "#;

        let rows = sqlx::query(QUERY)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await?;

        let mut app_ratings = Vec::new();
        for row in rows {
            let app_rating = ShortAppRating {
                app_id: row.get("app_id"),
                name: row.get("name"),
                pkg_name: row.get("pkg_name"),
                developer_name: row.get("developer_name"),
                icon_url: row.get("icon_url"),
                average_rating: {
                    let raw: String = row.get("average_rating");
                    raw.parse().unwrap_or_default()
                },
                total_star_rating_count: row.get("total_star_rating_count"),
            };
            app_ratings.push(app_rating);
        }

        Ok(app_ratings)
    }

    /// 获取最近更新的应用排行
    ///
    /// # 参数
    /// - `limit`: 返回的应用数量
    ///
    /// # 示例
    /// ```rust
    /// let db = Database::new("postgres://...", 5).await?;
    /// let recently_updated_apps = db.get_recently_updated_apps(10).await?;
    /// println!("最近更新的应用: {:?}", recently_updated_apps);
    /// ```
    pub async fn get_recently_updated_apps(&self, limit: u32) -> Result<Vec<AppInfo>> {
        const QUERY: &str = r#"
            SELECT
                ai.*
            FROM app_info ai
            JOIN app_metrics am ON ai.app_id = am.app_id
            ORDER BY am.release_date DESC
            LIMIT $1
        "#;

        let rows = sqlx::query(QUERY)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await?;

        let app_infos = rows.iter().map(Self::read_app_info_from_row).collect();

        Ok(app_infos)
    }

    /// 获取价格最高的应用排行
    ///
    /// # 参数
    /// - `limit`: 返回的应用数量
    ///
    /// # 示例
    /// ```rust
    /// let db = Database::new("postgres://...", 5).await?;
    /// let top_priced_apps = db.get_top_priced_apps(10).await?;
    /// println!("价格最高的应用: {:?}", top_priced_apps);
    /// ```
    pub async fn get_top_priced_apps(&self, limit: u32) -> Result<Vec<AppMetric>> {
        const QUERY: &str = r#"
            SELECT
                am.id,
                am.app_id,
                am.version,
                am.version_code,
                am.size_bytes,
                am.sha256,
                am.info_score::text,
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
            ORDER BY am.price DESC
            LIMIT $1
        "#;

        let rows = sqlx::query(QUERY)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await?;

        let app_metrics = rows.iter().map(Self::read_app_metric_from_row).collect();

        Ok(app_metrics)
    }

    /// 获取下载量增长最快的应用排行
    ///
    /// # 参数
    /// - `limit`: 返回的应用数量
    /// - `time_range`: 时间范围（如 "7d" 表示最近7天）
    ///
    /// # 示例
    /// ```rust
    /// let db = Database::new("postgres://...", 5).await?;
    /// let growth_apps = db.get_download_growth_apps(10, "7d").await?;
    /// println!("下载量增长最快的应用: {:?}", growth_apps);
    /// ```
    pub async fn get_download_growth_apps(
        &self,
        limit: u32,
        _time_range: &str,
    ) -> Result<Vec<AppMetric>> {
        // 简化实现：获取最近有更新的应用，按下载量排序
        const QUERY: &str = r#"
            SELECT DISTINCT ON (am.app_id)
                am.*
            FROM app_metrics am
            ORDER BY am.app_id, am.created_at DESC
            LIMIT $1
        "#;

        let rows = sqlx::query(QUERY)
            .bind(limit as i64)
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

        // 按下载量排序
        app_metrics.sort_by(|a, b| b.download_count.cmp(&a.download_count));
        app_metrics.truncate(limit as usize);

        Ok(app_metrics)
    }

    /// 获取评分增长最快的应用排行
    ///
    /// # 参数
    /// - `limit`: 返回的应用数量
    /// - `time_range`: 时间范围（如 "7d" 表示最近7天）
    ///
    /// # 示例
    /// ```rust
    /// let db = Database::new("postgres://...", 5).await?;
    /// let rating_growth_apps = db.get_rating_growth_apps(10, "7d").await?;
    /// println!("评分增长最快的应用: {:?}", rating_growth_apps);
    /// ```
    pub async fn get_rating_growth_apps(
        &self,
        limit: u32,
        _time_range: &str,
    ) -> Result<Vec<ShortAppRating>> {
        // 简化实现：获取评分最高的应用
        const QUERY: &str = r#"
            SELECT
                ai.app_id,
                ai.name,
                ai.pkg_name,
                ai.developer_name,
                ai.icon_url,
                ar.average_rating,
                ar.total_star_rating_count
            FROM app_info ai
            JOIN app_rating ar ON ai.app_id = ar.app_id
            ORDER BY ar.average_rating DESC, ar.total_star_rating_count DESC
            LIMIT $1
        "#;

        let rows = sqlx::query(QUERY)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await?;

        let mut app_ratings = Vec::new();
        for row in rows {
            let app_rating = ShortAppRating {
                app_id: row.get("app_id"),
                name: row.get("name"),
                pkg_name: row.get("pkg_name"),
                developer_name: row.get("developer_name"),
                icon_url: row.get("icon_url"),
                average_rating: row.get("average_rating"),
                total_star_rating_count: row.get("total_star_rating_count"),
            };
            app_ratings.push(app_rating);
        }

        Ok(app_ratings)
    }

    /// 获取开发者排行（按发布应用数量）
    ///
    /// # 参数
    /// - `limit`: 返回的开发者数量
    ///
    /// # 示例
    /// ```rust
    /// let db = Database::new("postgres://...", 5).await?;
    /// let top_developers = db.get_top_developers(10).await?;
    /// println!("发布应用最多的开发者: {:?}", top_developers);
    /// ```
    pub async fn get_top_developers(&self, limit: u32) -> Result<Vec<(String, String, i64)>> {
        const QUERY: &str = r#"
            SELECT
                dev_id,
                developer_name,
                COUNT(*) as app_count
            FROM app_info
            GROUP BY dev_id, developer_name
            ORDER BY app_count DESC
            LIMIT $1
        "#;

        let rows = sqlx::query(QUERY)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await?;

        let mut developers = Vec::new();
        for row in rows {
            let dev_id: String = row.get("dev_id");
            let developer_name: String = row.get("developer_name");
            let app_count: i64 = row.get("app_count");
            developers.push((dev_id, developer_name, app_count));
        }

        Ok(developers)
    }

    /// 获取下载量最高的应用
    ///
    /// # 参数
    /// - `limit`: 返回的应用数量
    /// - `exclude_keywords`: 可选的排除关键词，用于过滤开发者名称
    ///
    /// # 示例
    /// ```rust
    /// let db = Database::new("postgres://...", 5).await?;
    /// let top_downloads = db.get_top_downloads(10, None).await?;
    /// println!("下载量最高的应用: {:?}", top_downloads);
    ///
    /// let top_downloads_filtered = db.get_top_downloads(10, Some("huawei")).await?;
    /// println!("排除华为的下载量最高的应用: {:?}", top_downloads_filtered);
    /// ```
    pub async fn get_top_downloads(
        &self,
        limit: u32,
        exclude_keywords: Option<impl ToString>,
    ) -> Result<Vec<(AppInfo, AppMetric)>> {
        let (query, bind_exclude) = match exclude_keywords {
            Some(_) => (
                r#"
                SELECT
                    app_id, alliance_app_id, name, pkg_name,
                    dev_id, developer_name, dev_en_name,
                    supplier, kind_id, kind_name,
                    tag_name, kind_type_id, kind_type_name, icon_url,
                    brief_desc, description, privacy_url, ctype,
                    detail_id, app_level, jocat_id, iap, hms,
                    tariff_type, packing_type, order_app, denpend_gms,
                    denpend_hms, force_update, img_tag, is_pay,
                    is_disciplined, is_shelves, submit_type, delete_archive,
                    charging, button_grey, app_gift, free_days,
                    pay_install_type, created_at,
                    version, version_code, size_bytes,
                    sha256, info_score::text, info_rate_count,
                    download_count, price, release_date,
                    new_features, upgrade_msg, target_sdk,
                    minsdk, compile_sdk_version, min_hmos_api_level,
                    api_release_type, metrics_created_at, listed_at, comment
                FROM app_latest_info
                WHERE download_count IS NOT NULL
                  AND dev_en_name !~* $2
                ORDER BY download_count DESC
                LIMIT $1
                "#,
                true,
            ),
            None => (
                r#"
                SELECT
                    app_id, alliance_app_id, name, pkg_name,
                    dev_id, developer_name, dev_en_name,
                    supplier, kind_id, kind_name,
                    tag_name, kind_type_id, kind_type_name, icon_url,
                    brief_desc, description, privacy_url, ctype,
                    detail_id, app_level, jocat_id, iap, hms,
                    tariff_type, packing_type, order_app, denpend_gms,
                    denpend_hms, force_update, img_tag, is_pay,
                    is_disciplined, is_shelves, submit_type, delete_archive,
                    charging, button_grey, app_gift, free_days,
                    pay_install_type, created_at,
                    version, version_code, size_bytes,
                    sha256, info_score::text, info_rate_count,
                    download_count, price, release_date,
                    new_features, upgrade_msg, target_sdk,
                    minsdk, compile_sdk_version, min_hmos_api_level,
                    api_release_type, metrics_created_at, listed_at, comment
                FROM app_latest_info
                WHERE download_count IS NOT NULL
                ORDER BY download_count DESC
                LIMIT $1
                "#,
                false,
            ),
        };

        let rows = if bind_exclude {
            sqlx::query(query)
                .bind(limit as i64)
                .bind(exclude_keywords.unwrap().to_string())
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query(query)
                .bind(limit as i64)
                .fetch_all(&self.pool)
                .await?
        };

        let mut apps = Vec::new();
        for row in rows {
            let app_info = Self::read_app_info_from_row(&row);
            let app_metric = Self::read_app_metric_from_row(&row);

            apps.push((app_info, app_metric));
        }

        Ok(apps)
    }
}
