use crate::model::{AppInfo, AppMetric, AppRating, AppRaw};
use anyhow::Result;

use crate::db::Database;

impl Database {
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
}
