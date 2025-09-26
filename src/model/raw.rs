use serde::{Deserialize, Serialize};

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

fn hot_default() -> String {
    "0.0".to_string()
}

fn rate_num_default() -> String {
    "0".to_string()
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
    #[serde(rename = "orderApp", default)]
    pub order_app: bool,
    #[serde(rename = "denpendGms", default)]
    pub denpend_gms: i32,
    #[serde(rename = "denpendHms", default)]
    pub denpend_hms: i32,
    #[serde(rename = "forceUpdate", default)]
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
    #[serde(rename = "hot", default = "hot_default")]
    pub hot_score: String,
    #[serde(rename = "rateNum", default = "rate_num_default")]
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
    #[serde(rename = "minHmosApiLevel", default)]
    pub min_hmos_api_level: i32,
    #[serde(rename = "apiReleaseType")]
    pub api_release_type: String,
}
