use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};

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
    #[serde(rename = "permissions")]
    pub permissions: Vec<RawPermission>,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct RawPermission {
    #[serde(rename = "groupDesc")]
    pub group_desc: String,
    pub hide: String,
    #[serde(rename = "permissionDesc")]
    pub permission_desc: String,
    #[serde(rename = "permissionLabel")]
    pub permission_label: String,
}

/// 2. app_info 表
#[derive(Debug, Deserialize)]
pub struct AppInfo {
    pub app_id: String,
    pub alliance_app_id: String,
    pub name: String,
    pub pkg_name: String,
    pub dev_id: String,
    pub developer_name: String,
    pub dev_en_name: String,
    pub supplier: String,
    pub kind_id: String,
    pub kind_name: String,
    pub tag_name: Option<String>,
    pub kind_type_id: String,
    pub kind_type_name: String,
    pub icon_url: String,
    pub brief_desc: String,
    pub description: String,
    pub privacy_url: String,
    pub ctype: i32,
    pub detail_id: String,
    pub app_level: i32,
    pub jocat_id: i32,
    pub iap: i32,
    pub hms: i32,
    pub tariff_type: String,
    pub packing_type: i32,
    pub order_app: bool,
    pub denpend_gms: i32,
    pub denpend_hms: i32,
    pub force_update: i32,
    pub img_tag: String,
    pub is_pay: String,
    pub is_disciplined: i32,
    pub is_shelves: i32,
    pub submit_type: i32,
    pub delete_archive: i32,
    pub charging: i32,
    pub button_grey: i32,
    pub app_gift: i32,
    pub free_days: i32,
    pub pay_install_type: i32,
}

impl From<&RawJsonData> for AppInfo {
    fn from(value: &RawJsonData) -> Self {
        Self {
            app_id: value.app_id.clone(),
            alliance_app_id: value.alliance_app_id.clone(),
            name: value.name.clone(),
            pkg_name: value.pkg_name.clone(),
            dev_id: value.dev_id.clone(),
            developer_name: value.developer_name.clone(),
            dev_en_name: value.dev_en_name.clone(),
            supplier: value.supplier.clone(),
            kind_id: value.kind_id.clone(),
            kind_name: value.kind_name.clone(),
            tag_name: value.tag_name.clone(),
            kind_type_id: value.kind_type_id.clone(),
            kind_type_name: value.kind_type_name.clone(),
            icon_url: value.icon_url.clone(),
            brief_desc: value.brief_desc.clone(),
            description: value.description.clone(),
            privacy_url: value.privacy_url.clone(),
            ctype: value.ctype,
            detail_id: value.detail_id.clone(),
            app_level: value.app_level,
            jocat_id: value.jocat_id,
            iap: value.iap,
            hms: value.hms,
            tariff_type: value.tariff_type.clone(),
            packing_type: value.packing_type,
            order_app: value.order_app,
            denpend_gms: value.denpend_gms,
            denpend_hms: value.denpend_hms,
            force_update: value.force_update,
            img_tag: value.img_tag.clone(),
            is_pay: value.is_pay.clone(),
            is_disciplined: value.is_disciplined,
            is_shelves: value.is_shelves,
            submit_type: value.submit_type,
            delete_archive: value.delete_archive,
            charging: value.charging,
            button_grey: value.button_grey,
            app_gift: value.app_gift,
            free_days: value.free_days,
            pay_install_type: value.pay_install_type,
        }
    }
}

/// 3. app_permissions 表
#[derive(Debug, Deserialize)]
pub struct AppPermission {
    pub id: i64,
    pub app_id: String,
    pub group_desc: String,
    pub permission_label: String,
    pub permission_desc: String,
    pub hide: String,
}

impl From<&RawPermission> for AppPermission {
    fn from(value: &RawPermission) -> Self {
        Self {
            id: 0,
            app_id: String::new(),
            group_desc: value.group_desc.clone(),
            permission_label: value.permission_label.clone(),
            permission_desc: value.permission_desc.clone(),
            hide: value.hide.clone(),
        }
    }
}

impl AppPermission {
    /// 设置应用ID
    pub fn with_app_id(mut self, app_id: String) -> Self {
        self.app_id = app_id;
        self
    }
}

/// 4. app_metrics 表
#[derive(Debug, Deserialize)]
pub struct AppMetric {
    pub id: i64,
    pub app_id: String,
    pub version: String,
    pub version_code: i64,
    pub size_bytes: i64,
    pub sha256: String,
    pub hot_score: String,
    pub rate_num: String,
    pub download_count: String,
    pub price: String,
    pub release_date: i64,
    pub new_features: String,
    pub upgrade_msg: String,
    pub target_sdk: String,
    pub minsdk: String,
    pub compile_sdk_version: i32,
    pub min_hmos_api_level: i32,
    pub api_release_type: String,
    pub created_at: DateTime<Local>,
}

impl From<&RawJsonData> for AppMetric {
    fn from(value: &RawJsonData) -> Self {
        Self {
            id: 0,
            app_id: value.app_id.clone(),
            version: value.version.clone(),
            version_code: value.version_code,
            size_bytes: value.size_bytes,
            sha256: value.sha256.clone(),
            hot_score: value.hot_score.clone(),
            rate_num: value.rate_num.clone(),
            download_count: value.download_count.clone(),
            price: value.price.clone(),
            release_date: value.release_date,
            new_features: value.new_features.clone(),
            upgrade_msg: value.upgrade_msg.clone(),
            target_sdk: value.target_sdk.clone(),
            minsdk: value.min_sdk.clone(),
            compile_sdk_version: value.compile_sdk_version,
            min_hmos_api_level: value.min_hmos_api_level,
            api_release_type: value.api_release_type.clone(),
            created_at: Local::now(),
        }
    }
}

/// 5. app_raw 表
#[derive(Debug, Deserialize)]
pub struct AppRaw {
    pub id: i64,
    pub app_id: String,
    pub raw_json: serde_json::Value,
    pub created_at: DateTime<Local>,

}

impl From<&RawJsonData> for AppRaw {
    fn from(value: &RawJsonData) -> Self {
        Self {
            id: 0,
            app_id: value.app_id.clone(),
            raw_json: serde_json::to_value(value).unwrap(),
            created_at: Local::now(),
        }
    }
}
