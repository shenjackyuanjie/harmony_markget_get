use colored::Colorize;
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

fn api_release_type_default() -> String {
    "Release".to_string()
}

/// 支持的设备
/// 虽说按理来说应该是都有的, 但是, 万一呢
fn main_device_codes_default() -> Vec<u8> {
    const DEFAULT: [u8; 1] = [0];
    // 真用上了就报告一下
    println!("{}", "沟槽！真用上了!\n\n\n\n\n".on_red());
    DEFAULT.to_vec()
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
    /// 感谢 com.atomicservice.5765880207855314153
    /// 让我知道 hot 也可以没有的
    #[serde(rename = "hot", default = "hot_default")]
    pub hot_score: String,
    /// 感谢 com.atomicservice.5765880207855317715
    /// 让我知道 rate num 也可以没有的
    #[serde(rename = "rateNum", default = "rate_num_default")]
    pub rate_num: String,
    #[serde(rename = "downCount")]
    pub download_count: String,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "releaseDate")]
    pub release_date: i64,
    /// 感谢 C5765880207853227715
    /// 让我知道 new features 也能不带
    #[serde(rename = "newFeatures", default)]
    pub new_features: String,
    /// 感谢 C5765880207855312913
    /// 让我知道 upgrade msg 也能不带
    #[serde(rename = "upgradeMsg", default)]
    pub upgrade_msg: String,
    #[serde(rename = "targetSdk")]
    pub target_sdk: String,
    #[serde(rename = "minsdk")]
    pub min_sdk: String,
    /// 感谢 C5765880207852868633
    /// 让我知道 compile sdk version 也是可以不带的
    #[serde(rename = "compileSdkVersion", default)]
    pub compile_sdk_version: i32,
    /// 感谢 com.harmonyfzmj.huawei
    /// 让我知道 min hmos api level 也能不带
    #[serde(rename = "minHmosApiLevel", default)]
    pub min_hmos_api_level: i32,
    /// 感谢 com.cxy.chinaposthar
    /// 让我知道 api release type 也能不带
    #[serde(rename = "apiReleaseType", default = "api_release_type_default")]
    pub api_release_type: String,
    /// 支持的设备
    #[serde(rename = "mainDeviceCodes", default = "main_device_codes_default")]
    pub main_device_codes: Vec<u8>,
    /// 发布的国家
    #[serde(rename = "releaseCountries")]
    pub release_countries: Vec<String>,
}
