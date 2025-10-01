use colored::Colorize;

use crate::{model::AppQuery, sync::code::GLOBAL_CODE_MANAGER};

pub mod config;
pub mod db;
pub mod model;
pub mod server;
pub mod sync;
pub mod utils;

fn main() -> anyhow::Result<()> {
    utils::init_log();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_all()
        .build()?;

    rt.block_on(async_main())
}

#[allow(unused)]
fn i32_to_letters(mut n: i32) -> String {
    // 假设 n >= 0
    let mut s = String::new();
    while n >= 0 {
        let c = (b'a' + (n % 26) as u8) as char;
        s.insert(0, c);
        n = n / 26 - 1;
    }
    s
}

async fn async_main() -> anyhow::Result<()> {
    // 加载配置
    let config = config::Config::load()?;

    // C576588020785 6374145
    // C576588020785 6366961
    // C576588020785 2915863
    // C576588020785 2866435

    // let range = 2915863..=6366961;
    // let range = 0..=6366961;
    let range = 2000000..=6390000;
    // let range = 0000000..=9999999;
    // let range = 0..=475254;
    let start = "C576588020785";

    let _token = GLOBAL_CODE_MANAGER.update_token().await;

    let db = crate::db::Database::new(config.database_url(), config.db_max_connect()).await?;

    let batch = 1000;
    let wait_time = std::time::Duration::from_millis(25);
    let mut batch_count = 0;
    let total_batches = ((*range.end() - *range.start()) / batch as u64) + 1;
    let total_batches_u32 = total_batches as u32;
    let start_time = std::time::Instant::now();

    let client = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(config.api_timeout_seconds()))
        .build()?;
    let range_vec: Vec<u64> = range.collect();
    for bunch_id in range_vec.chunks(batch) {
        let mut join_set = tokio::task::JoinSet::new();
        for id in bunch_id.iter() {
            let client = client.clone();
            let db = db.clone();
            let api_url = config.api_info_url().to_string();
            let star_url = config.api_detail_url().to_string();
            let locale = config.locale().to_string();
            let app_id = format!("{start}{id:07}");
            // let app_id = format!("com.chinasoft.app.api12.{}", i32_to_letters(*id as i32));
            // let app_id = format!("com.fkccc.{}", i32_to_letters(*id as i32));
            // let app_id = format!("xkkj.uni.UNI{:X}", id);
            // let app_id = format!("com.fengyun.app{id}");
            join_set.spawn(async move {
                if let Ok(data) = crate::sync::get_app_info(
                    &client,
                    &api_url,
                    &AppQuery::app_id(&app_id),
                    &locale,
                )
                .await
                {
                    let star_result =
                        crate::sync::get_star_by_app_id(&client, &star_url, &app_id).await;
                    let star = match star_result {
                        Ok(star_data) => Some(star_data),
                        Err(e) => {
                            eprintln!("获取应用 {} 的评分数据失败: {:#}", app_id, e);
                            None
                        }
                    };

                    if let Ok(inserted) = db.save_app_data(&data, star.as_ref()).await {
                        if inserted.0 {
                            println!("{}", format!("已将 {app_id} 的数据插入数据库").on_green());
                        }
                        if inserted.1 {
                            println!(
                                "{}",
                                format!("已将 {app_id} 的评分数据插入数据库").on_green()
                            );
                        }
                    } else {
                        println!("插入数据库时出错");
                    }
                }
            });
        }
        join_set.join_all().await;
        batch_count += 1;
        let total_elapsed = start_time.elapsed();
        let avg_time_per_batch = total_elapsed / batch_count;
        let estimated_total_time = avg_time_per_batch * total_batches_u32;
        let remaining_time = estimated_total_time.saturating_sub(total_elapsed);

        print!(
            "\r[批次 {}/{}] id {} - {} 处理完成，总耗时 {:?}，预计剩余 {:?}，等待 {:?}",
            batch_count,
            total_batches,
            bunch_id[0],
            bunch_id[bunch_id.len() - 1],
            total_elapsed,
            remaining_time,
            wait_time
        );
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        tokio::time::sleep(wait_time).await;
    }

    Ok(())
}
