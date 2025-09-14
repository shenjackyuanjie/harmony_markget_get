use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
};
use reqwest::Client;

use crate::{
    config::Config,
    datas::{AppInfo, AppMetric},
    db::Database,
};

// pub async fn server(config: &Config, )

pub async fn worker(
    config: Config,
    mut waiter: tokio::sync::oneshot::Receiver<()>,
) -> anyhow::Result<()> {
    let db = crate::db::Database::new(config.database_url(), config.db_max_connect()).await?;
    let client = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(config.api_timeout_seconds()))
        .build()?;

    let interval = config.api_interval();
    let web_part = tokio::spawn(web_main(config.clone(), db.clone()));

    loop {
        crate::sync::sync_all(&client, &db, &config, &[]).await?;
        // 通过 select 同时等待/接受结束事件
        let wait_time = std::time::Duration::from_secs(interval);
        println!("等待 {wait_time:?} 后再同步");
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

struct QueryState {
    pub db: Database,
    pub client: Client,
    pub cfg: Config,
}

async fn query_pkg(
    State(state): State<Arc<QueryState>>,
    Path(pkg_name): Path<String>,
) -> impl IntoResponse {
    println!("正在尝试获取 {pkg_name} 的信息");
    match crate::sync::query_package(
        &state.client,
        &state.db,
        state.cfg.api_info_url(),
        state.cfg.api_detail_url(),
        &pkg_name,
        state.cfg.locale(),
    )
    .await
    {
        Ok((data, star)) => {
            let metric = AppMetric::from_raw_data_and_star(&data, &star);
            let info: AppInfo = (&data).into();
            Json(serde_json::json!({"info": info, "metric": metric}))
        }
        Err(e) => Json(serde_json::json!({"data": "faild to fetch", "error": e.to_string()})),
    }
}

async fn web_main(config: Config, db: Database) -> anyhow::Result<()> {
    let client = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(config.api_timeout_seconds()))
        .build()?;
    let query_state = Arc::new(QueryState {
        db,
        client,
        cfg: config.clone(),
    });
    let router = axum::Router::new()
        .route("/query/{pkg_name}", get(query_pkg).post(query_pkg))
        .with_state(query_state);

    let listenr = tokio::net::TcpListener::bind((config.serve_url(), config.serve_port())).await?;
    println!("开始监听 {}:{}", config.serve_url(), config.serve_port());
    axum::serve(listenr, router).await?;
    Ok(())
}
