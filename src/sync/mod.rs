use std::{sync::LazyLock, time::Duration};

use anyhow::Context;
use chrono::{DateTime, Local};
use colored::Colorize;
use reqwest::Client;
use serde_json::Value as JsonValue;
use tracing::{Level, event};

use crate::{
    db::Database,
    model::{AppQuery, RawJsonData, RawRatingData},
};

/// token 更新间隔
pub const TOKEN_UPDATE_INTERVAL: Duration = Duration::from_secs(600);

pub mod code;

/// UA
pub static USER_AGENT: LazyLock<String> = LazyLock::new(|| {
    format!(
        "get_huawei_market/{}-{}",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_NAME")
    )
});

/// 批量同步所有应用数据
///
/// # 参数
/// - `client`: HTTP客户端
/// - `db`: 数据库连接
/// - `config`: 配置信息
///
/// # 返回值
/// - `anyhow::Result<()>`: 同步结果
///
/// # 功能
/// 1. 获取配置中的包名列表
/// 2. 合并数据库中已存在的包名
/// 3. 随机打乱顺序
/// 4. 逐个同步每个包的数据
/// 5. 统计并输出结果
pub async fn sync_all(
    client: &Client,
    db: &crate::db::Database,
    config: &crate::config::Config,
) -> anyhow::Result<()> {
    let mut packages = config.packages().to_vec();
    let locale = config.locale();

    #[cfg(not(feature = "no_db_sync"))]
    for pkg in db.get_all_pkg_names().await?.iter() {
        if !packages.contains(pkg) {
            packages.push(pkg.to_string());
        }
    }

    packages.sort();
    packages.dedup();

    // 随机打乱顺序
    {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let mut state = seed;
        let n = packages.len();
        for i in 0..n {
            state = state
                .wrapping_mul(6364136223846793005u64)
                .wrapping_add(1442695040888963407u64);
            let j = i + ((state % (n - i) as u64) as usize);
            packages.swap(i, j);
        }
    }

    event!(Level::INFO, "开始同步 {} 个 包", packages.len());

    // 统计变量
    let start_time = std::time::Instant::now();
    let mut total_processed = 0;
    let mut total_inserted = 0;
    let mut total_skipped = 0;
    let mut total_failed = 0;

    for (index, package) in packages.iter().enumerate() {
        event!(
            Level::INFO,
            "[{}/{}] 同步包: {}",
            index + 1,
            packages.len(),
            package
        );

        total_processed += 1;
        match sync_app(
            client,
            db,
            config.api_url(),
            &AppQuery::pkg_name(package),
            locale,
            None,
            None,
        )
        .await
        {
            Ok(inserted) => {
                if inserted.0 || inserted.1 {
                    if inserted.0 {
                        event!(Level::INFO, "已将 {package} 的数据插入数据库");
                    }
                    if inserted.1 {
                        event!(Level::INFO, "已将 {package} 的评分数据插入数据库");
                    }
                    total_inserted += 1;
                    event!(
                        Level::INFO,
                        "[{}/{}] 包 {} 处理完成 (新数据已插入)",
                        index + 1,
                        packages.len(),
                        package
                    );
                } else {
                    total_skipped += 1;
                    event!(
                        Level::INFO,
                        "[{}/{}] 包 {} 处理完成 (数据相同，已跳过)",
                        index + 1,
                        packages.len(),
                        package
                    );
                }
            }
            Err(e) => {
                total_failed += 1;
                event!(
                    Level::WARN,
                    "[{}/{}] 包 {} 同步失败: {:#}",
                    index + 1,
                    packages.len(),
                    package,
                    e
                );
                continue;
            }
        }
    }

    println!("{}", "所有包处理完成！".green());

    let cost_time = start_time.elapsed();
    // 打印统计信息
    println!();
    println!("{}", "=".repeat(50).cyan());
    println!("{}", "处理统计信息:".cyan().bold());
    println!("{}", "=".repeat(50).cyan());
    println!("总处理包数: {}", total_processed.to_string().cyan());
    println!("新插入数据包数: {}", total_inserted.to_string().green());
    println!(
        "跳过相同数据包数: {}",
        total_skipped.to_string().bright_black()
    );
    println!("处理失败包数: {}", total_failed.to_string().red());
    println!("处理耗时: {:?}", cost_time);
    println!("{}", "=".repeat(50).cyan());

    Ok(())
}

/// 同步单个应用数据
///
/// # 参数
/// - `client`: HTTP客户端
/// - `db`: 数据库连接
/// - `api_url`: API地址
/// - `app_query`: 应用查询条件（包名或应用ID）
/// - `locale`: 语言设置
/// - `listed_at`: 上架时间（可选）
///
/// # 返回值
/// - `anyhow::Result<(bool, bool)>`: (数据是否插入, 评分数据是否插入)
///
/// # 功能
/// 1. 获取应用基本信息
/// 2. 获取应用评分信息（如果是普通应用）
/// 3. 保存数据到数据库
/// 4. 返回插入状态
pub async fn sync_app(
    client: &reqwest::Client,
    db: &Database,
    api_url: &str,
    app_query: &AppQuery,
    locale: &str,
    listed_at: Option<DateTime<Local>>,
    comment: Option<serde_json::Value>,
) -> anyhow::Result<(bool, bool, bool)> {
    let app_data = query_app(client, api_url, app_query, locale).await?;

    event!(
        Level::INFO,
        app_id = app_data.0.0.app_id,
        "获取到包 {app_query} 的数据, 应用名称: {}",
        app_data.0.0.name
    );

    // 保存数据到数据库（包含重复检查）
    let inserted = db
        .save_app_data(&app_data.0, app_data.1.as_ref(), listed_at, comment)
        .await
        .map_err(|e| anyhow::anyhow!("保存包 {} 的数据失败: {:#}", app_query, e))?;

    Ok(inserted)
}

/// 查询单个应用的完整数据
///
/// # 参数
/// - `client`: HTTP客户端
/// - `api_url`: API地址
/// - `app_query`: 应用查询条件（包名或应用ID）
/// - `locale`: 语言设置
///
/// # 返回值
/// - `anyhow::Result<(RawJsonData, Option<RawRatingData>)>`: (基本信息, 评分信息)
///
/// # 功能
/// 1. 获取应用基本信息
/// 2. 获取应用评分信息
/// 3. 返回完整数据但不保存到数据库
pub async fn query_app(
    client: &reqwest::Client,
    api_url: &str,
    app_query: &AppQuery,
    locale: &str,
) -> anyhow::Result<((RawJsonData, JsonValue), Option<RawRatingData>)> {
    let data = get_app_data(client, api_url, app_query, locale)
        .await
        .map_err(|e| anyhow::anyhow!("获取包 {} 的数据失败: {:#}", app_query, e))?;

    let (raw_data, data) = (
        data.clone(),
        serde_json::from_value::<RawJsonData>(data)
            .with_context(|| "不是，怎么又解析失败了")
            .unwrap(),
    );

    let star = if !data.pkg_name.starts_with("com.atomicservice") {
        let star_result = get_app_rating(client, api_url, &data.app_id).await;
        match star_result {
            Ok(star_data) => Some(star_data),
            Err(e) => {
                event!(Level::WARN, "获取包 {} 的评分数据失败: {}", app_query, e);
                None
            }
        }
    } else {
        event!(Level::INFO, "跳过元数据 {app_query} 的评分数据");
        None
    };

    event!(
        Level::INFO,
        app_id = data.app_id,
        "获取到包 {app_query} 的数据,应用ID: {}，应用名称: {}",
        data.app_id,
        data.name
    );

    Ok(((data, raw_data), star))
}

/// 获取应用基本信息
///
/// # 参数
/// - `client`: HTTP客户端
/// - `api_url`: API地址
/// - `app_query`: 应用查询条件（包名或应用ID）
/// - `locale`: 语言设置
///
/// # 返回值
/// - `anyhow::Result<RawJsonData>`: 应用基本信息
///
/// # 功能
/// 1. 构建请求体
/// 2. 发送HTTP请求
/// 3. 解析响应数据
/// 4. 返回应用基本信息
pub async fn get_app_data(
    client: &reqwest::Client,
    api_url: &str,
    app_query: &AppQuery,
    locale: impl ToString,
) -> anyhow::Result<JsonValue> {
    let body = serde_json::json!({
        app_query.app_info_type(): app_query.name(),
        "locale": locale.to_string(),
    });

    let token = code::GLOBAL_CODE_MANAGER.get_full_token().await;
    let response = client
        .post(format!("{api_url}/webedge/appinfo"))
        .header("Content-Type", "application/json")
        .header("User-Agent", USER_AGENT.to_string())
        .header("interface-code", token.interface_code)
        .header("identity-id", token.identity_id)
        .json(&body)
        .send()
        .await?;

    // 检查响应状态码
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "HTTP请求失败,状态码: {}",
            response.status()
        ));
    }

    // 检查响应体是否为空
    let content_length = response.content_length().unwrap_or(0);
    if content_length == 0 {
        return Err(anyhow::anyhow!("HTTP响应体为空"));
    }
    let mut raw = response.json::<serde_json::Value>().await?;
    let raw_obj = raw.as_object_mut().unwrap();
    if raw_obj.contains_key("AG-TraceId") {
        raw_obj.remove("AG-TraceId");
    };
    // 拜 cn.com.wind.wft_pc 所赐
    // 我们需要去掉可能的 \0
    let _ = raw_obj
        .get("privacyUrl")
        .and_then(|v| v.as_str())
        .map(|v| v.replace("\0", ""))
        .map(|v| raw_obj.insert("privacyUrl".to_string(), serde_json::Value::String(v)));
    Ok(raw)
}

/// 获取应用评分数据
///
/// # 参数
/// - `client`: HTTP客户端
/// - `api_url`: API地址
/// - `app_id`: 应用ID
///
/// # 返回值
/// - `anyhow::Result<RawRatingData>`: 应用评分数据
///
/// # 功能
/// 1. 构建评分查询请求体
/// 2. 发送HTTP请求
/// 3. 解析响应数据
/// 4. 返回评分信息
pub async fn get_app_rating(
    client: &reqwest::Client,
    api_url: &str,
    app_id: impl ToString,
) -> anyhow::Result<RawRatingData> {
    let body = serde_json::json!({
        "pageId": format!("webAgAppDetail|{}", app_id.to_string()),
        "pageNum": 1,
        "pageSize": 100,
        "zone": ""
    });

    let token = code::GLOBAL_CODE_MANAGER.get_full_token().await;
    let response = client
        .post(format!("{api_url}/harmony/page-detail"))
        .header("Content-Type", "application/json")
        .header("User-Agent", USER_AGENT.to_string())
        .header("Interface-Code", token.interface_code)
        .header("identity-id", token.identity_id)
        .json(&body)
        .send()
        .await?;

    // 检查响应状态码
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "HTTP请求失败,状态码: {}\nurl: {} body: {}",
            response.status(),
            api_url,
            body
        ));
    }

    // 检查响应体是否为空
    let content_length = response.content_length().unwrap_or(0);
    if content_length == 0 {
        return Err(anyhow::anyhow!(
            "HTTP响应体为空 \nurl: {api_url} data: {body}"
        ));
    }

    // 华为我谢谢你
    let data = {
        let raw = response.json::<serde_json::Value>().await?;
        let layouts = raw["pages"][0]["data"]["cardlist"]["layoutData"]
            .as_array()
            .expect("faild to parse page info");
        let comment_card = layouts
            .iter()
            .filter(|v| v["type"].as_str().expect("type not str") == "fl.card.comment")
            .collect::<Vec<_>>();
        if comment_card.is_empty() {
            return Err(anyhow::anyhow!("comment card not found"));
        }
        let star_data = comment_card[0]["data"].get(0).expect("data not found");
        if let Some(star_str) = star_data.get("starInfo") {
            serde_json::from_str(star_str.as_str().expect("starInfo not str"))?
        } else {
            return Err(anyhow::anyhow!("starInfo not found"));
        }
    };

    Ok(data)
}
