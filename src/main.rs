use std::time::Duration;

pub mod config;
pub mod sync;
pub mod server;
pub mod datas;
pub mod db;

fn main() -> anyhow::Result<()> {

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()?;

    rt.block_on(async_main())
}

async fn async_main() -> anyhow::Result<()> {
    // 加载配置
    let config = config::Config::load()?;

    let client = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(config.api_timeout_seconds()))
        .build()?;

    // 初始化数据库连接
    let db = db::Database::new(config.database_url()).await?;

    sync::sync_all(&client, &db, &config, config.packages()).await?;

    Ok(())
}
