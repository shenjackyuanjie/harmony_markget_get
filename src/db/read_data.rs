use crate::model::{AppInfo, AppMetric, AppRating};
use sqlx::{Row, postgres::PgRow};

use crate::db::Database;

pub const SELECT_APP_INFO_FIELDS: &str = r#"
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
    pay_install_type, created_at
"#;

pub const SELECT_APP_METRIC_FIELDS: &str = r#"
    app_id, version, version_code, size_bytes,
    sha256, info_score::text, info_rate_count,
    download_count, price::text, release_date,
    new_features, upgrade_msg, target_sdk,
    minsdk, compile_sdk_version, min_hmos_api_level,
    api_release_type, created_at AS metrics_created_at
"#;

pub const SELECT_APP_RATING_FIELDS: &str = r#"
    average_rating::text, star_1_rating_count,
    star_2_rating_count, star_3_rating_count,
    star_4_rating_count, star_5_rating_count,
    my_star_rating, total_star_rating_count,
    only_star_count, full_average_rating::text,
    source_type, created_at AS rating_created_at
"#;

impl Database {
    pub fn read_app_info_from_row(row: &PgRow) -> AppInfo {
        AppInfo {
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
        }
    }

    pub fn read_app_metric_from_row(row: &PgRow) -> AppMetric {
        AppMetric {
            id: row.try_get("id").unwrap_or(0),
            app_id: row.get("app_id"),
            version: row.get("version"),
            version_code: row.get("version_code"),
            size_bytes: row.get("size_bytes"),
            sha256: row.get("sha256"),
            info_score: {
                let raw: String = row.get("info_score");
                raw.parse().unwrap_or(0.0)
            },
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
            created_at: row.get("metrics_created_at"),
        }
    }

    pub fn read_app_rating_from_row(row: &PgRow) -> AppRating {
        AppRating {
            id: row.try_get("id").unwrap_or(0),
            app_id: row.get("app_id"),
            average_rating: {
                let raw: String = row.get("average_rating");
                raw.parse().unwrap_or(0.0)
            },
            star_1_rating_count: row.get("star_1_rating_count"),
            star_2_rating_count: row.get("star_2_rating_count"),
            star_3_rating_count: row.get("star_3_rating_count"),
            star_4_rating_count: row.get("star_4_rating_count"),
            star_5_rating_count: row.get("star_5_rating_count"),
            my_star_rating: row.get("my_star_rating"),
            total_star_rating_count: row.get("total_star_rating_count"),
            only_star_count: row.get("only_star_count"),
            full_average_rating: {
                let raw: String = row.get("full_average_rating");
                raw.parse().unwrap_or(0.0)
            },
            source_type: row.get("source_type"),
            created_at: row.get("rating_created_at"),
        }
    }
}
