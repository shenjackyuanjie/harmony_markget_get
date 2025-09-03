use colored::*;
use serde::Serialize;
use std::env;
use std::time::Duration;

use crate::datas::AppRaw;

pub mod config;
pub mod datas;
pub mod db;

fn main() -> anyhow::Result<()> {
    // 解析命令行参数
    let args: Vec<String> = env::args().collect();
    let cli_packages: Vec<String> = if args.len() > 1 {
        args[1..].iter().map(|s| s.to_string()).collect()
    } else {
        Vec::new()
    };

    // 显示帮助信息（如果有帮助参数）
    if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
        println!("华为应用市场数据采集工具");
        println!();
        println!("用法:");
        println!("  {} [包名1] [包名2] ...", args[0]);
        println!("  {} -h | --help", args[0]);
        println!();
        println!("参数:");
        println!("  包名1, 包名2, ...  要查询的应用包名列表");
        println!("  -h, --help        显示帮助信息");
        println!();
        println!("示例:");
        println!("  {} com.example.app1", args[0]);
        println!("  {} com.huawei.app1 com.tencent.app2", args[0]);
        println!("  {} -h", args[0]);
        return Ok(());
    }

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()?;

    rt.block_on(async_main(cli_packages))
}

async fn async_main(cli_packages: Vec<String>) -> anyhow::Result<()> {
    // 加载配置
    let config = config::Config::load()?;

    let client = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(config.api_timeout_seconds()))
        .build()?;

    // 初始化数据库连接
    let db = db::Database::new(config.database_url()).await?;

    let locale = config.locale();
    let mut packages = if !cli_packages.is_empty() {
        // 使用命令行参数中的包名
        println!(
            "{}",
            format!("使用命令行参数中的 {} 个包", cli_packages.len()).cyan()
        );
        cli_packages
    } else {
        // 使用配置文件中的包名
        println!("{}", "使用配置文件中的包".cyan());
        config.packages().to_vec()
    };

    for pkg in db.get_all_pkg_names().await?.iter() {
        if !packages.contains(pkg) {
            packages.push(pkg.to_string());
        }
    }

    println!(
        "{}",
        format!("开始处理 {} 个应用包...", packages.len()).cyan()
    );

    for (index, package) in packages.iter().enumerate() {
        println!(
            "{}",
            format!("[{}/{}] 处理包: {}", index + 1, packages.len(), package).yellow()
        );

        match process_package(&client, &db, config.api_base_url(), package, locale).await {
            Ok(_) => {
                println!(
                    "{}",
                    format!("[{}/{}] 包 {} 处理完成", index + 1, packages.len(), package).green()
                );
            }
            Err(e) => {
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
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    println!("{}", "所有包处理完成！".green());
    Ok(())
}

/// 处理单个应用包
async fn process_package(
    client: &reqwest::Client,
    db: &db::Database,
    api_url: &str,
    package: &str,
    locale: &str,
) -> anyhow::Result<()> {
    let data = get_raw_json_data(client, api_url, package, locale)
        .await
        .map_err(|e| anyhow::anyhow!("获取包 {} 的数据失败: {:#}", package, e))?;

    println!(
        "{}",
        format!(
            "获取到包 {} 的数据，应用ID: {}，应用名称: {}",
            package, data.app_id, data.name
        )
        .blue()
    );

    // 保存数据到数据库（包含重复检查）
    save_app_data(db, &data)
        .await
        .map_err(|e| anyhow::anyhow!("保存包 {} 的数据失败: {:#}", package, e))?;

    Ok(())
}

async fn get_raw_json_data(
    client: &reqwest::Client,
    api_url: &str,
    pkg_name: impl ToString,
    locale: impl ToString,
) -> anyhow::Result<datas::RawJsonData> {
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
            "HTTP请求失败，状态码: {}",
            response.status()
        ));
    }

    // 检查响应体是否为空
    let content_length = response.content_length().unwrap_or(0);
    if content_length == 0 {
        return Err(anyhow::anyhow!("HTTP响应体为空"));
    }

    let data = response.json::<datas::RawJsonData>().await?;

    Ok(data)
}

/// 保存应用数据到数据库
async fn save_app_data(db: &db::Database, raw_data: &datas::RawJsonData) -> anyhow::Result<()> {
    // 转换原始JSON数据用于比较
    let new_raw_json: AppRaw = raw_data.into();
    let new_json_value = &new_raw_json.raw_json;

    // 检查是否与最后一条数据相同
    if db
        .is_same_as_last_data(&raw_data.app_id, new_json_value)
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
        return Ok(());
    }

    // 转换并保存应用信息
    let app_info = datas::AppInfo::from(raw_data);
    db.insert_app_info(&app_info).await?;

    // 保存指标信息
    let app_metric = datas::AppMetric::from(raw_data);
    db.insert_app_metric(&app_metric).await?;

    // 保存原始JSON数据
    db.insert_raw_data(&new_raw_json).await?;

    println!(
        "{}",
        format!("应用数据保存成功: {} ({})", raw_data.app_id, raw_data.name).green()
    );
    Ok(())
}
