use tracing::{Level, event};

use get_market::{config, db, utils};

fn main() -> anyhow::Result<()> {
    utils::init_log();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_all()
        .build()?;
    event!(Level::INFO, "async rt built");
    rt.block_on(async_main())
}

async fn async_main() -> anyhow::Result<()> {
    // 加载配置
    let config = config::Config::load()?;
    let _db = db::Database::new(config.database_url(), config.db_max_connect()).await?;

    Ok(())
}
