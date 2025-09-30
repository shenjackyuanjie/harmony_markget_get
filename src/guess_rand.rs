use colored::Colorize;

use crate::{model::AppQuery, sync::code::GLOBAL_CODE_MANAGER};
use std::time::{SystemTime, UNIX_EPOCH};

pub mod config;
pub mod db;
pub mod model;
pub mod server;
pub mod sync;
pub mod utils;

#[derive(Clone)]
struct Random {
    state: u64,
}

impl Random {
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        Self { state: seed }
    }

    pub fn next(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(1664525u64)
            .wrapping_add(1013904223u64);
        self.state
    }
}

fn main() -> anyhow::Result<()> {
    utils::init_log();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_all()
        .build()?;

    rt.block_on(async_main())
}

async fn async_main() -> anyhow::Result<()> {
    // 加载配置
    let config = config::Config::load()?;

    // C69175 59067092904725
    // C69175 59057467250518
    // C69175 84361454081323
    // C69175 85170011059280

    // let range = 59057467250518_u64..=84361454081323_u64;
    let code_start = 59067092904725_u64;
    let size = 85170011059280_u64 - code_start;
    let start = "C69175";

    let _token = GLOBAL_CODE_MANAGER.update_token().await;

    let db = crate::db::Database::new(config.database_url(), config.db_max_connect()).await?;

    let batch = 1000;
    let wait_time = std::time::Duration::from_millis(5);
    let mut batch_count = 0;

    let client = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(config.api_timeout_seconds()))
        .build()?;

    let mut rng = Random::new();

    loop {
        let mut ids: Vec<u64> = Vec::with_capacity(batch as usize);
        for _ in 0..batch {
            let id = code_start + (rng.next() % size);
            ids.push(id);
        }

        let mut join_set = tokio::task::JoinSet::new();
        for &id in &ids {
            let client = client.clone();
            let db = db.clone();
            let api_url = config.api_info_url().to_string();
            let star_url = config.api_detail_url().to_string();
            let locale = config.locale().to_string();
            let app_id = format!("{start}{}", id);
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
        print!(
            "\r[批次 {}] 已处理 {} 个随机 ID，等待 {:?}",
            batch_count, batch, wait_time
        );
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        tokio::time::sleep(wait_time).await;
    }
}
