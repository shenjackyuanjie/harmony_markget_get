use anyhow::Result;
use std::ops::Range;

pub mod config;
pub mod datas;
pub mod db;
pub mod server;
pub mod sync;
pub mod utils;

fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_all()
        .build()?;

    rt.block_on(async_main())
}

async fn async_main() -> anyhow::Result<()> {
    // 加载配置
    let config = config::Config::load()?;

    // 连接数据库
    let db = crate::db::Database::new(config.database_url(), config.db_max_connect()).await?;

    // 获取数据库中所有的 app_id
    println!("正在从数据库获取所有 app_id...");
    let existing_app_ids = db.get_all_app_ids().await?;
    println!("从数据库获取到 {} 个 app_id", existing_app_ids.len());

    // 解析 app_id 格式并生成需要猜测的范围
    let mut ranges_to_guess = Vec::new();
    
    for app_id in existing_app_ids {
        if let Some(numeric_part) = extract_numeric_part(&app_id) {
            let start_range = numeric_part.saturating_sub(1000);
            let end_range = numeric_part.saturating_add(1000);
            ranges_to_guess.push((app_id.clone(), start_range..end_range));
        }
    }

    println!("生成了 {} 个猜测范围", ranges_to_guess.len());

    let client = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(config.api_timeout_seconds()))
        .build()?;

    let batch = 1000;
    let wait_time = std::time::Duration::from_millis(50);

    // 处理每个 app_id 的猜测范围
    for (base_app_id, range) in ranges_to_guess {
        println!("开始处理基础 app_id: {} 的范围 {}..{}", base_app_id, range.start, range.end);
        
        let prefix = extract_prefix(&base_app_id).unwrap_or_else(|| "C576588020785".to_string());
        
        for bunch_id in range.collect::<Vec<_>>().chunks(batch) {
            let mut join_set = tokio::task::JoinSet::new();
            
            for id in bunch_id.iter() {
                let client = client.clone();
                let db = db.clone();
                let api_url = config.api_info_url().to_string();
                let locale = config.locale().to_string();
                let app_id = format!("{}{}", prefix, id);
                
                join_set.spawn(async move {
                    if let Ok(data) =
                        crate::sync::get_pkg_data_by_app_id(&client, &api_url, &app_id, &locale).await
                    {
                        let star_result =
                            crate::sync::get_star_by_app_id(&client, &api_url, &app_id).await;
                        let star = match star_result {
                            Ok(star_data) => Some(star_data),
                            Err(e) => {
                                eprintln!("获取应用 {} 的评分数据失败: {:#}", app_id, e);
                                None
                            }
                        };

                        if let Ok(inserted) = db.save_app_data(&data, star.as_ref()).await {
                            if inserted {
                                println!("已将 {app_id} 的数据插入数据库");
                            }
                        } else {
                            println!("插入数据库时出错");
                        }
                    }
                });
            }
            
            join_set.join_all().await;
            println!(
                "id {} - {} 的包处理完成，等待 {:?}",
                bunch_id[0],
                bunch_id[bunch_id.len() - 1],
                wait_time
            );
            tokio::time::sleep(wait_time).await;
        }
    }

    Ok(())
}

/// 从 app_id 中提取数字部分
fn extract_numeric_part(app_id: &str) -> Option<u64> {
    // app_id 格式通常是 "C5765880207856366961"
    // 我们需要提取最后面的数字部分
    let numeric_str: String = app_id.chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    
    if numeric_str.is_empty() {
        None
    } else {
        numeric_str.parse().ok()
    }
}

/// 从 app_id 中提取前缀部分
fn extract_prefix(app_id: &str) -> Option<String> {
    // 找到最后一个数字字符的位置，然后取前面的部分
    let last_digit_pos = app_id.rfind(|c: char| c.is_ascii_digit())?;
    let prefix_end = last_digit_pos + 1;
    Some(app_id[..prefix_end].to_string())
}