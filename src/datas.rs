use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::utils::sanitize_utf8_string;

// "starInfo": "{\"averageRating\":\"2.9\",\"oneStarRatingCount\":348,\"twoStarRatingCount\":129,\"threeStarRatingCount\":129,\"fourStarRatingCount\":81,\"fiveStarRatingCount\":344,\"myStarRating\":0,\"totalStarRatingCount\":1031,\"onlyStarCount\":511,\"fullAverageRating\":\"5.0\",\"sourceType\":\"USER_RATING\"}",
//
#[derive(Debug, Deserialize, Serialize)]
pub struct RawRatingData {
    #[serde(rename = "averageRating")]
    pub average_rating: String,
    #[serde(rename = "oneStarRatingCount")]
    pub star_1_rating_count: i32,
    #[serde(rename = "twoStarRatingCount")]
    pub star_2_rating_count: i32,
    #[serde(rename = "threeStarRatingCount")]
    pub star_3_rating_count: i32,
    #[serde(rename = "fourStarRatingCount")]
    pub star_4_rating_count: i32,
    #[serde(rename = "fiveStarRatingCount")]
    pub star_5_rating_count: i32,
    #[serde(rename = "myStarRating")]
    pub my_star_rating: i32,
    #[serde(rename = "totalStarRatingCount")]
    pub total_star_rating_count: i32,
    #[serde(rename = "onlyStarCount")]
    pub only_star_count: i32,
    #[serde(rename = "fullAverageRating")]
    pub full_average_rating: String,
    #[serde(rename = "sourceType")]
    pub source_type: String,
}

/// 1. 原始 JSON 数据直接映射
#[derive(Debug, Deserialize, Serialize)]
pub struct RawJsonData {
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "allianceAppId")]
    pub alliance_app_id: String,
    pub name: String,
    #[serde(rename = "pkgName")]
    pub pkg_name: String,
    #[serde(rename = "devId")]
    pub dev_id: String,
    #[serde(rename = "developerName")]
    pub developer_name: String,
    #[serde(rename = "devEnName")]
    pub dev_en_name: String,
    #[serde(rename = "supplier")]
    pub supplier: String,
    #[serde(rename = "kindId")]
    pub kind_id: String,
    #[serde(rename = "kindName")]
    pub kind_name: String,
    #[serde(rename = "tagName")]
    pub tag_name: Option<String>,
    #[serde(rename = "kindTypeId")]
    pub kind_type_id: String,
    #[serde(rename = "kindTypeName")]
    pub kind_type_name: String,
    #[serde(rename = "icon")]
    pub icon_url: String,
    #[serde(rename = "briefDes")]
    pub brief_desc: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "privacyUrl")]
    pub privacy_url: String,
    #[serde(rename = "ctype")]
    pub ctype: i32,
    #[serde(rename = "detailId")]
    pub detail_id: String,
    #[serde(rename = "appLevel")]
    pub app_level: i32,
    #[serde(rename = "jocatId")]
    pub jocat_id: i32,
    pub iap: i32,
    pub hms: i32,
    #[serde(rename = "tariffType")]
    pub tariff_type: String,
    #[serde(rename = "packingType")]
    pub packing_type: i32,
    #[serde(rename = "orderApp")]
    pub order_app: bool,
    #[serde(rename = "denpendGms")]
    pub denpend_gms: i32,
    #[serde(rename = "denpendHms")]
    pub denpend_hms: i32,
    #[serde(rename = "forceUpdate")]
    pub force_update: i32,
    #[serde(rename = "imgTag")]
    pub img_tag: String,
    #[serde(rename = "isPay")]
    pub is_pay: String,
    #[serde(rename = "isDisciplined")]
    pub is_disciplined: i32,
    #[serde(rename = "isShelves")]
    pub is_shelves: i32,
    #[serde(rename = "submitType")]
    pub submit_type: i32,
    #[serde(rename = "deleteArchive")]
    pub delete_archive: i32,
    #[serde(rename = "charging")]
    pub charging: i32,
    #[serde(rename = "buttonGrey")]
    pub button_grey: i32,
    #[serde(rename = "appGift")]
    pub app_gift: i32,
    #[serde(rename = "freeDays")]
    pub free_days: i32,
    #[serde(rename = "payInstallType")]
    pub pay_install_type: i32,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "versionCode")]
    pub version_code: i64,
    #[serde(rename = "size")]
    pub size_bytes: i64,
    #[serde(rename = "sha256")]
    pub sha256: String,
    #[serde(rename = "hot")]
    pub hot_score: String,
    #[serde(rename = "rateNum")]
    pub rate_num: String,
    #[serde(rename = "downCount")]
    pub download_count: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "releaseDate")]
    pub release_date: i64,
    #[serde(rename = "newFeatures")]
    pub new_features: String,
    #[serde(rename = "upgradeMsg")]
    pub upgrade_msg: String,
    #[serde(rename = "targetSdk")]
    pub target_sdk: String,
    #[serde(rename = "minsdk")]
    pub min_sdk: String,
    #[serde(rename = "compileSdkVersion")]
    pub compile_sdk_version: i32,
    #[serde(rename = "minHmosApiLevel")]
    pub min_hmos_api_level: i32,
    #[serde(rename = "apiReleaseType")]
    pub api_release_type: String,
}

/// 2. app_info 表
#[derive(Debug, Deserialize, Serialize)]
pub struct AppInfo {
    pub app_id: String,
    pub alliance_app_id: String,
    pub name: String,
    pub pkg_name: String,
    pub dev_id: String,
    pub developer_name: String,
    pub dev_en_name: String,
    pub supplier: String,
    pub kind_id: i32,
    pub kind_name: String,
    pub tag_name: Option<String>,
    pub kind_type_id: i32,
    pub kind_type_name: String,
    pub icon_url: String,
    pub brief_desc: String,
    pub description: String,
    pub privacy_url: String,
    pub ctype: i32,
    pub detail_id: String,
    pub app_level: i32,
    pub jocat_id: i32,
    pub iap: bool,
    pub hms: bool,
    pub tariff_type: String,
    pub packing_type: i32,
    pub order_app: bool,
    pub denpend_gms: bool,
    pub denpend_hms: bool,
    pub force_update: bool,
    pub img_tag: String,
    pub is_pay: bool,
    pub is_disciplined: bool,
    pub is_shelves: bool,
    pub submit_type: i32,
    pub delete_archive: bool,
    pub charging: bool,
    pub button_grey: bool,
    pub app_gift: bool,
    pub free_days: i32,
    pub pay_install_type: i32,
    pub created_at: DateTime<Local>,
}

impl From<&RawJsonData> for AppInfo {
    fn from(value: &RawJsonData) -> Self {
        Self {
            app_id: sanitize_utf8_string(&value.app_id).into_owned(),
            alliance_app_id: sanitize_utf8_string(&value.alliance_app_id).into_owned(),
            name: sanitize_utf8_string(&value.name).into_owned(),
            pkg_name: sanitize_utf8_string(&value.pkg_name).into_owned(),
            dev_id: sanitize_utf8_string(&value.dev_id).into_owned(),
            developer_name: sanitize_utf8_string(&value.developer_name).into_owned(),
            dev_en_name: sanitize_utf8_string(&value.dev_en_name).into_owned(),
            supplier: sanitize_utf8_string(&value.supplier).into_owned(),
            kind_id: value.kind_id.parse().unwrap_or(0),
            kind_name: sanitize_utf8_string(&value.kind_name).into_owned(),
            tag_name: value
                .tag_name
                .as_ref()
                .map(|s| sanitize_utf8_string(s).into_owned()),
            kind_type_id: value.kind_type_id.parse().unwrap_or(0),
            kind_type_name: sanitize_utf8_string(&value.kind_type_name).into_owned(),
            icon_url: sanitize_utf8_string(&value.icon_url).into_owned(),
            brief_desc: sanitize_utf8_string(&value.brief_desc).into_owned(),
            description: sanitize_utf8_string(&value.description).into_owned(),
            privacy_url: sanitize_utf8_string(&value.privacy_url).into_owned(),
            ctype: value.ctype,
            detail_id: sanitize_utf8_string(&value.detail_id).into_owned(),
            app_level: value.app_level,
            jocat_id: value.jocat_id,
            iap: value.iap != 0,
            hms: value.hms != 0,
            tariff_type: sanitize_utf8_string(&value.tariff_type).into_owned(),
            packing_type: value.packing_type,
            order_app: value.order_app,
            denpend_gms: value.denpend_gms != 0,
            denpend_hms: value.denpend_hms != 0,
            force_update: value.force_update != 0,
            img_tag: sanitize_utf8_string(&value.img_tag).into_owned(),
            is_pay: value.is_pay == "1",
            is_disciplined: value.is_disciplined != 0,
            is_shelves: value.is_shelves != 0,
            submit_type: value.submit_type,
            delete_archive: value.delete_archive != 0,
            charging: value.charging != 0,
            button_grey: value.button_grey != 0,
            app_gift: value.app_gift != 0,
            free_days: value.free_days,
            pay_install_type: value.pay_install_type,
            created_at: Local::now(),
        }
    }
}

/// 4. app_metrics 表
#[derive(Debug, Deserialize, Serialize)]
pub struct AppMetric {
    pub id: i64,
    pub app_id: String,
    pub version: String,
    pub version_code: i64,
    pub size_bytes: i64,
    pub sha256: String,
    pub info_score: f64,
    pub info_rate_count: i64,
    pub download_count: i64,
    pub price: String,
    pub release_date: i64,
    pub new_features: String,
    pub upgrade_msg: String,
    pub target_sdk: i32,
    pub minsdk: i32,
    pub compile_sdk_version: i32,
    pub min_hmos_api_level: i32,
    pub api_release_type: String,
    pub created_at: DateTime<Local>,
}

/// 5. app_rating 表
#[derive(Debug, Deserialize, Serialize)]
pub struct AppRating {
    pub id: i64,
    pub app_id: String,
    pub average_rating: f64,
    pub star_1_rating_count: i32,
    pub star_2_rating_count: i32,
    pub star_3_rating_count: i32,
    pub star_4_rating_count: i32,
    pub star_5_rating_count: i32,
    pub my_star_rating: i32,
    pub total_star_rating_count: i32,
    pub only_star_count: i32,
    pub full_average_rating: f64,
    pub source_type: String,
    pub created_at: DateTime<Local>,
}

impl AppMetric {
    pub fn from_raw_data(raw_data: &RawJsonData) -> Self {
        Self {
            id: 0,
            app_id: raw_data.app_id.clone(),
            version: sanitize_utf8_string(&raw_data.version).into_owned(),
            version_code: raw_data.version_code,
            size_bytes: raw_data.size_bytes,
            sha256: sanitize_utf8_string(&raw_data.sha256).into_owned(),
            info_score: raw_data.hot_score.parse().unwrap_or(0.0),
            info_rate_count: raw_data.rate_num.parse().unwrap_or(0),
            download_count: raw_data.download_count.parse().unwrap_or(0),
            price: sanitize_utf8_string(&raw_data.price).into_owned(),
            release_date: raw_data.release_date,
            new_features: sanitize_utf8_string(&raw_data.new_features).into_owned(),
            upgrade_msg: sanitize_utf8_string(&raw_data.upgrade_msg).into_owned(),
            target_sdk: raw_data.target_sdk.parse().unwrap_or(0),
            minsdk: raw_data.min_sdk.parse().unwrap_or(0),
            compile_sdk_version: raw_data.compile_sdk_version,
            min_hmos_api_level: raw_data.min_hmos_api_level,
            api_release_type: sanitize_utf8_string(&raw_data.api_release_type).into_owned(),
            created_at: Local::now(),
        }
    }
}

impl AppRating {
    pub fn from_raw_star(raw_data: &RawJsonData, raw_star: &RawRatingData) -> Self {
        Self {
            id: 0,
            app_id: raw_data.app_id.clone(),
            average_rating: raw_star.average_rating.parse().unwrap_or(0.0),
            star_1_rating_count: raw_star.star_1_rating_count,
            star_2_rating_count: raw_star.star_2_rating_count,
            star_3_rating_count: raw_star.star_3_rating_count,
            star_4_rating_count: raw_star.star_4_rating_count,
            star_5_rating_count: raw_star.star_5_rating_count,
            my_star_rating: raw_star.my_star_rating,
            total_star_rating_count: raw_star.total_star_rating_count,
            only_star_count: raw_star.only_star_count,
            full_average_rating: raw_star.full_average_rating.parse().unwrap_or(0.0),
            source_type: sanitize_utf8_string(&raw_star.source_type).into_owned(),
            created_at: Local::now(),
        }
    }
}

/// 6. app_raw 表
#[derive(Debug, Deserialize, Serialize)]
pub struct AppRaw {
    pub id: i64,
    pub app_id: String,
    pub raw_json_data: serde_json::Value,
    pub raw_json_star: serde_json::Value,
    pub created_at: DateTime<Local>,
}

impl AppRaw {
    pub fn from_raw_datas(data: &RawJsonData, star: Option<&RawRatingData>) -> Self {
        Self {
            id: 0,
            app_id: data.app_id.clone(),
            raw_json_data: serde_json::to_value(data).unwrap(),
            raw_json_star: match star {
                Some(star_data) => serde_json::to_value(star_data).unwrap(),
                None => serde_json::json!({}),
            },
            created_at: Local::now(),
        }
    }
}
