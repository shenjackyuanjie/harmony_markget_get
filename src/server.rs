use std::sync::Arc;

use axum::{
    extract::{Path, State}, response::IntoResponse, routing::get, Json
};
use reqwest::Client;

use crate::{config::Config, datas::AppMetric, db::Database};

// pub async fn server(config: &Config, )

pub async fn worker(
    config: Config,
    db: Database,
    mut waiter: tokio::sync::oneshot::Receiver<()>,
) -> anyhow::Result<()> {
    let client = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(config.api_timeout_seconds()))
        .build()?;

    let interval = config.api_interval();
    let web_part = tokio::spawn(web_main(config.clone(), db.clone()));

    loop {
        // 通过 select 同时等待/接受结束事件
        tokio::select! {
            _ = tokio::time::sleep(std::time::Duration::from_secs(interval)) => {
            }
            _ = &mut waiter => {
                break;
            }
        }
        crate::sync::sync_all(&client, &db, &config, &[]).await?;
    }

    web_part.abort();
    Ok(())
}

struct QueryState {
    pub db: Database,
    pub client: Client,
    pub cfg: Config
}

async fn query_pkg(State(state): State<Arc<QueryState>>, Path(pkg_name): Path<String>) -> impl IntoResponse {
    println!("正在尝试获取 {pkg_name} 的信息");
    match crate::sync::query_package(&state.client, &state.db, state.cfg.api_base_url(), &pkg_name, state.cfg.locale()).await {
        Ok(pkg) => {
            let data: AppMetric = (&pkg).into();
            Json(serde_json::to_value(data).expect("wtf"))
        }
        Err(e) => {
            Json(serde_json::json!({"data": "faild to fetch", "error": e.to_string()}))
        }
    }
}

async fn web_main(
    config: Config,
    db: Database,
) -> anyhow::Result<()> {
    let client = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(config.api_timeout_seconds()))
        .build()?;
    let query_state = Arc::new(QueryState {
        db, client, cfg: config.clone()
    });
    let router = axum::Router::new()
        .route("/query/{pkg_name}", get(query_pkg))
        .with_state(query_state);

    let listenr = tokio::net::TcpListener::bind((config.serve_url(), config.serve_port())).await?;
    axum::serve(listenr, router).await?;
    Ok(())
}
