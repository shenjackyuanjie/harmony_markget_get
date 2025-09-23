use crate::datas::{AppInfo, AppMetric, AppRating, AppRaw, RawJsonData, RawRatingData};
use chrono::{DateTime, Local};

use std::ops::Range;

use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Row;
use sqlx::postgres::{PgPool, PgPoolOptions};

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

impl Database {
    /// 创建数据库连接池
    pub async fn new(database_url: &str, max_connect: u32) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connect)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    /// 插入应用信息到 app_info 表
    pub async fn insert_app_info(&self, app_info: &AppInfo) -> Result<()> {
        const QUERY: &str = r#"
            INSERT INTO app_info (
                app_id, alliance_app_id, name, pkg_name, dev_id, developer_name,
                dev_en_name, supplier, kind_id, kind_name, tag_name,
                kind_type_id, kind_type_name, icon_url, brief_desc, description,
                privacy_url, ctype, detail_id, app_level, jocat_id, iap, hms,
                tariff_type, packing_type, order_app, denpend_gms, denpend_hms,
                force_update, img_tag, is_pay, is_disciplined, is_shelves,
                submit_type, delete_archive, charging, button_grey, app_gift,
                free_days, pay_install_type, created_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15,
                $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28,
                $29, $30, $31, $32, $33, $34, $35, $36, $37, $38, $39, $40, $41
            )
            ON CONFLICT (app_id) DO UPDATE SET
                alliance_app_id = EXCLUDED.alliance_app_id,
                name = EXCLUDED.name,
                pkg_name = EXCLUDED.pkg_name,
                dev_id = EXCLUDED.dev_id,
                developer_name = EXCLUDED.developer_name,
                dev_en_name = EXCLUDED.dev_en_name,
                supplier = EXCLUDED.supplier,
                kind_id = EXCLUDED.kind_id,
                kind_name = EXCLUDED.kind_name,
                tag_name = EXCLUDED.tag_name,
                kind_type_id = EXCLUDED.kind_type_id,
                kind_type_name = EXCLUDED.kind_type_name,
                icon_url = EXCLUDED.icon_url,
                brief_desc = EXCLUDED.brief_desc,
                description = EXCLUDED.description,
                privacy_url = EXCLUDED.privacy_url,
                ctype = EXCLUDED.ctype,
                detail_id = EXCLUDED.detail_id,
                app_level = EXCLUDED.app_level,
                jocat_id = EXCLUDED.jocat_id,
                iap = EXCLUDED.iap,
                hms = EXCLUDED.hms,
                tariff_type = EXCLUDED.tariff_type,
                packing_type = EXCLUDED.packing_type,
                order_app = EXCLUDED.order_app,
                denpend_gms = EXCLUDED.denpend_gms,
                denpend_hms = EXCLUDED.denpend_hms,
                force_update = EXCLUDED.force_update,
                img_tag = EXCLUDED.img_tag,
                is_pay = EXCLUDED.is_pay,
                is_disciplined = EXCLUDED.is_disciplined,
                is_shelves = EXCLUDED.is_shelves,
                submit_type = EXCLUDED.submit_type,
                delete_archive = EXCLUDED.delete_archive,
                charging = EXCLUDED.charging,
                button_grey = EXCLUDED.button_grey,
                app_gift = EXCLUDED.app_gift,
                free_days = EXCLUDED.free_days,
                pay_install_type = EXCLUDED.pay_install_type
        "#;

        sqlx::query(QUERY)
            .bind(&app_info.app_id)
            .bind(&app_info.alliance_app_id)
            .bind(&app_info.name)
            .bind(&app_info.pkg_name)
            .bind(&app_info.dev_id)
            .bind(&app_info.developer_name)
            .bind(&app_info.dev_en_name)
            .bind(&app_info.supplier)
            .bind(app_info.kind_id)
            .bind(&app_info.kind_name)
            .bind(&app_info.tag_name)
            .bind(app_info.kind_type_id)
            .bind(&app_info.kind_type_name)
            .bind(&app_info.icon_url)
            .bind(&app_info.brief_desc)
            .bind(&app_info.description)
            .bind(&app_info.privacy_url)
            .bind(app_info.ctype)
            .bind(&app_info.detail_id)
            .bind(app_info.app_level)
            .bind(app_info.jocat_id)
            .bind(app_info.iap)
            .bind(app_info.hms)
            .bind(&app_info.tariff_type)
            .bind(app_info.packing_type)
            .bind(app_info.order_app)
            .bind(app_info.denpend_gms)
            .bind(app_info.denpend_hms)
            .bind(app_info.force_update)
            .bind(&app_info.img_tag)
            .bind(app_info.is_pay)
            .bind(app_info.is_disciplined)
            .bind(app_info.is_shelves)
            .bind(app_info.submit_type)
            .bind(app_info.delete_archive)
            .bind(app_info.charging)
            .bind(app_info.button_grey)
            .bind(app_info.app_gift)
            .bind(app_info.free_days)
            .bind(app_info.pay_install_type)
            .bind(app_info.created_at)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// 插入应用指标到 app_metrics 表
    pub async fn insert_app_metric(&self, app_metric: &AppMetric) -> Result<()> {
        const QUERY: &str = r#"
            INSERT INTO app_metrics (
                app_id, version, version_code, size_bytes, sha256, info_score,
                info_rate_count, download_count, price, release_date, new_features,
                upgrade_msg, target_sdk, minsdk, compile_sdk_version,
                min_hmos_api_level, api_release_type
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17
            )
        "#;

        sqlx::query(QUERY)
            .bind(&app_metric.app_id)
            .bind(&app_metric.version)
            .bind(app_metric.version_code)
            .bind(app_metric.size_bytes)
            .bind(&app_metric.sha256)
            .bind(app_metric.info_score)
            .bind(app_metric.info_rate_count)
            .bind(app_metric.download_count)
            .bind(&app_metric.price)
            .bind(app_metric.release_date)
            .bind(&app_metric.new_features)
            .bind(&app_metric.upgrade_msg)
            .bind(app_metric.target_sdk)
            .bind(app_metric.minsdk)
            .bind(app_metric.compile_sdk_version)
            .bind(app_metric.min_hmos_api_level)
            .bind(&app_metric.api_release_type)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// 插入应用评分到 app_rating 表
    pub async fn insert_app_rating(&self, app_rating: &AppRating) -> Result<()> {
        const QUERY: &str = r#"
            INSERT INTO app_rating (
                app_id, average_rating,
                star_1_rating_count, star_2_rating_count, star_3_rating_count,
                star_4_rating_count, star_5_rating_count, my_star_rating,
                total_star_rating_count, only_star_count, full_average_rating,
                source_type
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
            )
        "#;

        sqlx::query(QUERY)
            .bind(&app_rating.app_id)
            .bind(app_rating.average_rating)
            .bind(app_rating.star_1_rating_count)
            .bind(app_rating.star_2_rating_count)
            .bind(app_rating.star_3_rating_count)
            .bind(app_rating.star_4_rating_count)
            .bind(app_rating.star_5_rating_count)
            .bind(app_rating.my_star_rating)
            .bind(app_rating.total_star_rating_count)
            .bind(app_rating.only_star_count)
            .bind(app_rating.full_average_rating)
            .bind(&app_rating.source_type)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// 插入原始 JSON 数据到 app_raw 表
    pub async fn insert_raw_data(&self, data: &AppRaw) -> Result<()> {
        let query = r#"
            INSERT INTO app_raw (app_id, raw_json_data, raw_json_star)
            VALUES ($1, $2, $3)
        "#;

        sqlx::query(query)
            .bind(data.app_id.clone())
            .bind(&data.raw_json_data)
            .bind(&data.raw_json_star)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

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
            Ok(&last_data == new_data)
        } else {
            Ok(false)
        }
    }

    pub async fn is_same_rating(&self, app_id: &str, new_rating: &Value) -> Result<bool> {
        if let Some(last_rating) = self.get_last_raw_json_star(app_id).await? {
            Ok(&last_rating == new_rating)
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
            JOIN app_raw ar ON ai.app_id = ar.app_id
            ORDER BY ar.created_at DESC
            LIMIT $1 OFFSET $2
        "#;

        let rows = sqlx::query(QUERY)
            .bind((range.end - range.start) as i64) // LIMIT
            .bind(range.start as i64) // OFFSET
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
            (total_count as f32 / page_size as f32).ceil() as u32
        };
        let offset = (page - 1) * page_size;

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

    /// 保存应用数据到数据库
    /// 返回布尔值表示是否插入了新数据
    pub async fn save_app_data(
        &self,
        raw_data: &RawJsonData,
        raw_star: Option<&RawRatingData>,
    ) -> anyhow::Result<(bool, bool)> {
        // 转换原始JSON数据用于比较
        let raw_json = AppRaw::from_raw_datas(raw_data, raw_star);
        let insert_data = if self
            .is_same_data(&raw_data.app_id, &raw_json.raw_json_data)
            .await?
        {
            println!(
                "{}",
                format!(
                    "数据与最后一条记录相同，跳过插入: {} ({})",
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
