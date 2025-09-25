pub mod config;
pub mod db;
pub mod model;
pub mod server;
pub mod sync;
pub mod utils;

use chrono::{DateTime, FixedOffset};
use tracing::{Level, event};

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
    event!(Level::INFO, "connecting to db");
    let _db = db::Database::new(config.database_url(), config.db_max_connect()).await?;
    event!(Level::INFO, "connected to db");
    let git_ver = get_log_time();
    event!(Level::INFO, "git version: {}", git_ver);

    Ok(())
}

fn get_log_time() -> DateTime<FixedOffset> {
    let out = std::process::Command::new("git")
        .args(["log", "-1", "--format=%cd", "--date=iso"])
        .output()
        .expect("Failed to execute git command");
    let time_str = String::from_utf8_lossy(&out.stdout).trim().to_string();
    DateTime::parse_from_str(&time_str, "%Y-%m-%d %H:%M:%S %z").expect("Failed to parse datetime")
}
