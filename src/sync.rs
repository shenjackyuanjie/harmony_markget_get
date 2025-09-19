use std::time::Duration;

use colored::Colorize;
use reqwest::Client;

use crate::{
    datas::{RawJsonData, RawRatingData},
    db::Database,
};

pub mod interface_code;

pub async fn sync_all(
    client: &Client,
    db: &crate::db::Database,
    config: &crate::config::Config,
    pack: &[String],
) -> anyhow::Result<()> {
    let mut packages = pack.to_vec();
    let locale = config.locale();

    for pkg in db.get_all_pkg_names().await?.iter() {
        if !packages.contains(pkg) {
            packages.push(pkg.to_string());
        }
    }

    packages.sort();
    packages.dedup();

    println!(
        "{}",
        format!("开始处理 {} 个应用包...", packages.len()).cyan()
    );

    // 统计变量
    let start_time = std::time::Instant::now();
    let mut total_processed = 0;
    let mut total_inserted = 0;
    let mut total_skipped = 0;
    let mut total_failed = 0;

    for (index, package) in packages.iter().enumerate() {
        println!(
            "{}",
            format!("[{}/{}] 处理包: {}", index + 1, packages.len(), package).yellow()
        );

        total_processed += 1;
        match process_package(
            client,
            db,
            config.api_info_url(),
            config.api_detail_url(),
            package,
            locale,
        )
        .await
        {
            Ok(inserted) => {
                if inserted.0 || inserted.1 {
                    if inserted.0 {
                        println!("{}", format!("已将 {package} 的数据插入数据库").on_green());
                    }
                    if inserted.1 {
                        println!(
                            "{}",
                            format!("已将 {package} 的评分数据插入数据库").on_green()
                        );
                    }
                    total_inserted += 1;
                    println!(
                        "{}",
                        format!(
                            "[{}/{}] 包 {} 处理完成 (新数据已插入)",
                            index + 1,
                            packages.len(),
                            package
                        )
                        .green()
                    );
                } else {
                    total_skipped += 1;
                    println!(
                        "{}",
                        format!(
                            "[{}/{}] 包 {} 处理完成 (数据相同，已跳过)",
                            index + 1,
                            packages.len(),
                            package
                        )
                        .bright_black()
                    );
                }
            }
            Err(e) => {
                total_failed += 1;
                eprintln!(
                    "{}",
                    format!(
                        "[{}/{}] 包 {} 处理失败: {:#}",
                        index + 1,
                        packages.len(),
                        package,
                        e
                    )
                    .red()
                );
                // 继续处理下一个包，不中断整个流程
                continue;
            }
        }

        // 添加短暂延迟，避免请求过于频繁
        if index < packages.len() - 1 {
            tokio::time::sleep(Duration::from_millis(50)).await;
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

/// 处理单个应用包
pub async fn process_package(
    client: &reqwest::Client,
    db: &Database,
    data_url: &str,
    star_url: &str,
    package_name: &str,
    locale: &str,
) -> anyhow::Result<(bool, bool)> {
    let data = get_pkg_data_by_pkg_name(client, data_url, package_name, locale)
        .await
        .map_err(|e| anyhow::anyhow!("获取包 {} 的数据失败: {:#}", package_name, e))?;

    let star = if !package_name.starts_with("com.atomicservice") {
        let star_result = get_star_by_app_id(client, star_url, &data.app_id).await;
        match star_result {
            Ok(star_data) => Some(star_data),
            Err(e) => {
                eprintln!(
                    "{}",
                    format!("获取包 {} 的评分数据失败: {:#}", package_name, e).yellow()
                );
                None
            }
        }
    } else {
        println!("跳过元数据 {package_name} 的评分数据");
        None
    };

    println!(
        "{}",
        format!(
            "获取到包 {package_name} 的数据,应用ID: {}，应用名称: {}",
            data.app_id, data.name
        )
        .blue()
    );

    // 保存数据到数据库（包含重复检查）
    let inserted = db
        .save_app_data(&data, star.as_ref())
        .await
        .map_err(|e| anyhow::anyhow!("保存包 {} 的数据失败: {:#}", package_name, e))?;

    Ok(inserted)
}

/// 查询单个应用包
pub async fn query_package_by_pkg_name(
    client: &reqwest::Client,
    db: &Database,
    data_url: &str,
    star_url: &str,
    package_name: &str,
    locale: &str,
) -> anyhow::Result<(RawJsonData, Option<RawRatingData>, (bool, bool))> {
    let data = get_pkg_data_by_pkg_name(client, data_url, package_name, locale)
        .await
        .map_err(|e| anyhow::anyhow!("获取包 {} 的数据失败: {:#}", package_name, e))?;

    let star_result = get_star_by_app_id(client, star_url, &data.app_id).await;
    let star = match star_result {
        Ok(star_data) => Some(star_data),
        Err(e) => {
            eprintln!(
                "{}",
                format!("获取包 {} 的评分数据失败: {:#}", package_name, e).yellow()
            );
            None
        }
    };

    println!(
        "{}",
        format!(
            "获取到包 {} 的数据,应用ID: {}，应用名称: {}",
            package_name, data.app_id, data.name
        )
        .blue()
    );

    // 保存数据到数据库（包含重复检查）
    let is_new = db
        .save_app_data(&data, star.as_ref())
        .await
        .map_err(|e| anyhow::anyhow!("保存包 {} 的数据失败: {:#}", package_name, e))?;

    Ok((data, star, is_new))
}

/// 查询单个应用包
pub async fn query_package_by_app_id(
    client: &reqwest::Client,
    db: &Database,
    data_url: &str,
    star_url: &str,
    app_id: &str,
    locale: &str,
) -> anyhow::Result<(RawJsonData, Option<RawRatingData>, (bool, bool))> {
    let data = get_pkg_data_by_app_id(client, data_url, app_id, locale)
        .await
        .map_err(|e| anyhow::anyhow!("获取包 {} 的数据失败: {:#}", app_id, e))?;

    let star_result = get_star_by_app_id(client, star_url, &data.app_id).await;
    let star = match star_result {
        Ok(star_data) => Some(star_data),
        Err(e) => {
            eprintln!(
                "{}",
                format!("获取包 {} 的评分数据失败: {:#}", data.name, e).yellow()
            );
            None
        }
    };

    println!(
        "{}",
        format!(
            "获取到包 {} 的数据,应用ID: {}，应用名称: {}",
            data.name, data.app_id, data.name
        )
        .blue()
    );

    // 保存数据到数据库（包含重复检查）
    let is_new = db
        .save_app_data(&data, star.as_ref())
        .await
        .map_err(|e| anyhow::anyhow!("保存包 {} 的数据失败: {:#}", data.name, e))?;

    Ok((data, star, is_new))
}

pub async fn get_star_by_app_id(
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

    let response = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .header(
            "User-Agent",
            format!("get_market/{}", env!("CARGO_PKG_VERSION")),
        )
        .header(
            "Interface-Code",
            interface_code::GLOBAL_CODE.get_full_token().await,
        )
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

    // let data = response.json::<RawStarData>().await?;
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

pub async fn get_pkg_data_by_app_id(
    client: &reqwest::Client,
    api_url: &str,
    app_id: impl ToString,
    locale: impl ToString,
) -> anyhow::Result<RawJsonData> {
    get_pkg_data(client, api_url, app_id, locale, "appId").await
}

pub async fn get_pkg_data_by_pkg_name(
    client: &reqwest::Client,
    api_url: &str,
    pkg_name: impl ToString,
    locale: impl ToString,
) -> anyhow::Result<RawJsonData> {
    get_pkg_data(client, api_url, pkg_name, locale, "pkgName").await
}

#[inline]
pub async fn get_pkg_data(
    client: &reqwest::Client,
    api_url: &str,
    name: impl ToString,
    locale: impl ToString,
    name_type: impl ToString,
) -> anyhow::Result<RawJsonData> {
    let body = serde_json::json!({
        name_type.to_string(): name.to_string(),
        "locale": locale.to_string(),
    });

    let response = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .header(
            "User-Agent",
            format!("get_market/{}", env!("CARGO_PKG_VERSION")),
        )
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

    let data = response.json::<serde_json::Value>().await?;

    let parsed_data = serde_json::from_value(data.clone()).map_err(|e| anyhow::anyhow!("json 解析错误 {e}\n{data}"))?;

    Ok(parsed_data)
}
