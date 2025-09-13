pub mod config;
pub mod datas;
pub mod db;
pub mod server;
pub mod sync;

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

    let range = 2915863..=6366961;
    let start = "C576588020785";

    let db = crate::db::Database::new(config.database_url(), config.db_max_connect()).await?;

    let batch = 100;
    let wait_time = std::time::Duration::from_millis(50);

    let client = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(config.api_timeout_seconds()))
        .build()?;

    for bunch_id in range.collect::<Vec<_>>().chunks(batch) {
        let mut handles = vec![];
        for id in bunch_id.iter() {
            let client = client.clone();
            let db = db.clone();
            let api_url = config.api_base_url().to_string();
            let locale = config.locale().to_string();
            let app_id = format!("{start}{id}");
            handles.push(tokio::spawn(async move {
                match crate::sync::get_pkg_data_by_app_id(&client, &api_url, &app_id, &locale).await
                {
                    Ok(data) => {
                        match db.save_app_data(&data).await {
                            Ok(inserted) => {
                                if inserted {
                                    println!("已将 {app_id} 的数据插入数据库");
                                }
                                // } else {
                                //     println!("数据与最后一条记录相同，跳过插入: {app_id}");
                                // }
                            }
                            Err(e) => {
                                println!("插入数据库时出错: {e}");
                            }
                        }
                    }
                    Err(e) => {
                        // println!("获取 {app_id} 的数据时出错: {e}");
                    }
                }
            }));
        }
        for handle in handles {
            handle.await?;
        }
        println!("id {} - {} 的包处理完成，等待 {:?}", bunch_id[0], bunch_id[bunch_id.len()-1], wait_time);
        tokio::time::sleep(wait_time).await;
    }

    Ok(())
}
