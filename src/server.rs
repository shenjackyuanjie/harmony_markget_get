use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
};
use colored::Colorize;
use reqwest::Client;
use tracing::{Level, event};

use crate::{
    config::{Config, get_config},
    db::Database,
    model::{AppInfo, AppMetric, AppQuery, AppRating},
};

pub async fn worker(mut waiter: tokio::sync::oneshot::Receiver<()>) -> anyhow::Result<()> {
    let config = get_config();
    event!(Level::INFO, "connecting to db");
    let db = crate::db::Database::new(config.database_url(), config.db_max_connect()).await?;
    event!(Level::INFO, "connected to db");
    let client = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(config.api_timeout_seconds()))
        .build()?;

    let interval = config.api_interval();
    let web_part = tokio::spawn(web_main(config.clone(), db.clone()));

    loop {
        crate::sync::sync_all(&client, &db, config, &[]).await?;
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

struct QueryState {
    pub db: Database,
    pub client: Client,
    pub cfg: Config,
}

async fn query_pkg(
    State(state): State<Arc<QueryState>>,
    Path(pkg_name): Path<String>,
) -> impl IntoResponse {
    event!(Level::INFO, "http 服务正在尝试获取 {pkg_name} 的信息");
    match crate::sync::query_package(
        &state.client,
        &state.db,
        state.cfg.api_info_url(),
        state.cfg.api_detail_url(),
        &AppQuery::pkg_name(&pkg_name),
        state.cfg.locale(),
    )
    .await
    {
        Ok((data, star, is_new)) => {
            let metric = AppMetric::from_raw_data(&data);
            let rating = star
                .as_ref()
                .map(|star_data| AppRating::from_raw_star(&data, star_data));
            let info: AppInfo = (&data).into();
            Json(
                serde_json::json!({"info": info, "metric": metric, "rating": rating, "is_new": is_new}),
            )
        }
        Err(e) => {
            event!(Level::WARN, "http服务获取 {pkg_name} 的信息失败: {e}");
            Json(serde_json::json!({"data": "faild to fetch", "error": e.to_string()}))
        }
    }
}

async fn query_app_id(
    State(state): State<Arc<QueryState>>,
    Path(app_id): Path<String>,
) -> impl IntoResponse {
    event!(Level::INFO, "http 服务正在尝试获取 {app_id} 的信息");
    match crate::sync::query_package(
        &state.client,
        &state.db,
        state.cfg.api_info_url(),
        state.cfg.api_detail_url(),
        &AppQuery::app_id(&app_id),
        state.cfg.locale(),
    )
    .await
    {
        Ok((data, star, is_new)) => {
            let metric = AppMetric::from_raw_data(&data);
            let rating = star
                .as_ref()
                .map(|star_data| AppRating::from_raw_star(&data, star_data));
            let info: AppInfo = (&data).into();
            Json(
                serde_json::json!({"info": info, "metric": metric, "rating": rating, "is_new": is_new}),
            )
        }
        Err(e) => {
            event!(Level::WARN, "http服务获取 {app_id} 的信息失败: {e}");
            Json(serde_json::json!({"data": "faild to fetch", "error": e.to_string()}))
        }
    }
}

async fn app_list_info(State(state): State<Arc<QueryState>>) -> impl IntoResponse {
    event!(Level::INFO, "http 服务正在尝试获取应用列表信息");
    if let Ok(app_count) = state.db.count_apps().await
        && let Ok(atomic_services_count) = state.db.count_atomic_services().await
    {
        Json(
            serde_json::json!({"app_count": app_count, "atomic_services_count": atomic_services_count}),
        )
    } else {
        event!(Level::WARN, "http服务获取应用列表信息失败");
        Json(serde_json::json!({"data": "faild to fetch", "error": "Database error"}))
    }
}

async fn app_list_paged(
    State(state): State<Arc<QueryState>>,
    Path(page): Path<String>,
) -> impl IntoResponse {
    const PAGE_BATCH: u32 = 100;
    match page.parse::<u32>() {
        Ok(page) => {
            match state
                .db
                .get_app_info_paginated_enhanced(page, PAGE_BATCH)
                .await
            {
                Ok(apps) => Json(serde_json::to_value(apps).unwrap()),
                Err(_) => {
                    Json(serde_json::json!({"data": "faild to fetch", "error": "Database error"}))
                }
            }
        }
        Err(e) => {
            Json(serde_json::json!({"data": "faild to parse page", "error": format!("{}", e)}))
        }
    }
}

async fn app_list_paged_short(
    State(state): State<Arc<QueryState>>,
    Path(page): Path<String>,
) -> impl IntoResponse {
    const PAGE_BATCH: u32 = 100;
    match page.parse::<u32>() {
        Ok(page) => {
            match state
                .db
                .get_app_info_paginated_short(page, PAGE_BATCH)
                .await
            {
                Ok(apps) => Json(serde_json::to_value(apps).unwrap()),
                Err(_) => {
                    Json(serde_json::json!({"data": "faild to fetch", "error": "Database error"}))
                }
            }
        }
        Err(e) => {
            Json(serde_json::json!({"data": "faild to parse page", "error": format!("{}", e)}))
        }
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
        .route("/query/pkg_name/{pkg_name}", get(query_pkg).post(query_pkg))
        .route(
            "/query/app_id/{app_id}",
            get(query_app_id).post(query_app_id),
        )
        .route("/app_list/info", get(app_list_info).post(app_list_info))
        .route(
            "/app_list/{page_count}/detail",
            get(app_list_paged).post(app_list_paged),
        )
        .route(
            "/app_list/{page_count}",
            get(app_list_paged_short).post(app_list_paged_short),
        )
        .with_state(query_state);

    let listenr = tokio::net::TcpListener::bind((config.serve_url(), config.serve_port())).await?;
    event!(
        Level::INFO,
        "开始监听 {}:{}",
        config.serve_url(),
        config.serve_port()
    );
    axum::serve(listenr, router).await?;
    Ok(())
}
