use colored::Colorize;

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

    // C5765880207856366961
    // C5765880207852915863

    // let range = 2915863..=6366961;
    // let range = 0..=6366961;
    let range = 2915863..=9999999;
    let start = "C576588020785";

    let db = crate::db::Database::new(config.database_url(), config.db_max_connect()).await?;

    let batch = 1000;
    let wait_time = std::time::Duration::from_millis(50);

    let client = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(config.api_timeout_seconds()))
        .build()?;

    for bunch_id in range.collect::<Vec<_>>().chunks(batch) {
        let mut join_set = tokio::task::JoinSet::new();
        for id in bunch_id.iter() {
            let client = client.clone();
            let db = db.clone();
            let api_url = config.api_info_url().to_string();
            let locale = config.locale().to_string();
            let app_id = format!("{start}{id}");
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
                        if inserted.0 {
                            println!("{}", "已将 {app_id} 的数据插入数据库".green());
                        }
                        if inserted.1 {
                            println!("{}", format!("已将 {app_id} 的评分数据插入数据库").green());
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

    Ok(())
}
