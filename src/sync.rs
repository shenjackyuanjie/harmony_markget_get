use std::time::Duration;

use colored::Colorize;
use reqwest::Client;
use serde::Serialize;

use crate::{datas::RawJsonData, db::Database};

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

        match process_package(client, db, config.api_base_url(), package, locale).await {
            Ok(inserted) => {
                if inserted {
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
    println!("{}", "=".repeat(50).cyan());

    Ok(())
}

/// 处理单个应用包
pub async fn process_package(
    client: &reqwest::Client,
    db: &Database,
    api_url: &str,
    package_name: &str,
    locale: &str,
) -> anyhow::Result<bool> {
    let data = get_pkg_data_by_pkg_name(client, api_url, package_name, locale)
        .await
        .map_err(|e| anyhow::anyhow!("获取包 {} 的数据失败: {:#}", package_name, e))?;

    println!(
        "{}",
        format!(
            "获取到包 {} 的数据,应用ID: {}，应用名称: {}",
            package_name, data.app_id, data.name
        )
        .blue()
    );

    // 保存数据到数据库（包含重复检查）
    let inserted = db
        .save_app_data(&data)
        .await
        .map_err(|e| anyhow::anyhow!("保存包 {} 的数据失败: {:#}", package_name, e))?;

    Ok(inserted)
}

/// 查询单个应用包
pub async fn query_package(
    client: &reqwest::Client,
    db: &Database,
    api_url: &str,
    package_name: &str,
    locale: &str,
) -> anyhow::Result<RawJsonData> {
    let data = get_pkg_data_by_pkg_name(client, api_url, package_name, locale)
        .await
        .map_err(|e| anyhow::anyhow!("获取包 {} 的数据失败: {:#}", package_name, e))?;

    println!(
        "{}",
        format!(
            "获取到包 {} 的数据,应用ID: {}，应用名称: {}",
            package_name, data.app_id, data.name
        )
        .blue()
    );

    // 保存数据到数据库（包含重复检查）
    db
        .save_app_data(&data)
        .await
        .map_err(|e| anyhow::anyhow!("保存包 {} 的数据失败: {:#}", package_name, e))?;

    Ok(data)
}

pub async fn get_pkg_data_by_pkg_name(
    client: &reqwest::Client,
    api_url: &str,
    pkg_name: impl ToString,
    locale: impl ToString,
) -> anyhow::Result<crate::datas::RawJsonData> {
    #[derive(Debug, Serialize)]
    struct RequestBody {
        #[serde(rename = "pkgName")]
        pkg_name: String,
        locale: String,
    }
    let body = RequestBody {
        pkg_name: pkg_name.to_string(),
        locale: locale.to_string(),
    };

    let response = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .header(
            "User-Agent",
            format!("get_huawei_market/{}", env!("CARGO_PKG_VERSION")),
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

    let data = response.json::<crate::datas::RawJsonData>().await?;

    Ok(data)
}
