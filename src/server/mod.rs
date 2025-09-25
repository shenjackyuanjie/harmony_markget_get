pub mod handlers;
pub mod routes;
pub mod state;

use std::sync::Arc;

use colored::Colorize;
use tracing::{Level, event};

use crate::{
    config::{Config, get_config},
    db::Database,
    sync::code::GLOBAL_CODE_MANAGER,
};

use self::state::AppState;

pub use handlers::*;
pub use routes::create_router;

/// Web服务器工作线程
pub async fn worker(mut waiter: tokio::sync::oneshot::Receiver<()>) -> anyhow::Result<()> {
    let config = get_config();
    event!(Level::INFO, "connecting to db");
    let db = crate::db::Database::new(config.database_url(), config.db_max_connect()).await?;
    event!(Level::INFO, "connected to db");
    let client = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(config.api_timeout_seconds()))
        .build()?;
    let _ = GLOBAL_CODE_MANAGER.update_token().await;

    let interval = config.api_interval();
    let web_part = tokio::spawn(web_main(config.clone(), db.clone()));

    loop {
        crate::sync::sync_all(&client, &db, config).await?;
        // 通过 select 同时等待/接受结束事件
        let wait_time = std::time::Duration::from_secs(interval);
        println!("{}", format!("等待 {:?} 后再同步", wait_time).green());
        tokio::select! {
            _ = tokio::time::sleep(wait_time) => {
            }
            _ = &mut waiter => {
                break;
            }
        }
    }

    web_part.abort();
    Ok(())
}

/// Web服务器主函数
pub async fn web_main(config: Config, db: Database) -> anyhow::Result<()> {
    let client = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(config.api_timeout_seconds()))
        .build()?;

    let app_state = Arc::new(AppState {
        db,
        client,
        cfg: config.clone(),
    });

    let router = routes::create_router(app_state);

    let listener = tokio::net::TcpListener::bind((config.serve_url(), config.serve_port())).await?;
    event!(
        Level::INFO,
        "开始监听 {}:{}",
        config.serve_url(),
        config.serve_port()
    );

    axum::serve(listener, router).await?;
    Ok(())
}
