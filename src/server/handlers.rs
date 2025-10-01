use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use serde_json::json;
use tracing::{Level, event};

use std::sync::Arc;

use crate::{
    model::{AppInfo, AppMetric, AppQuery, AppRating, FullAppInfo, ShortAppInfo},
    server::state::{ApiResponse, AppListQuery, AppState, RankingQuery},
};

/// 查询应用包名信息
pub async fn query_pkg(
    State(state): State<Arc<AppState>>,
    Path(pkg_name): Path<String>,
) -> impl IntoResponse {
    event!(
        Level::INFO,
        "http 服务正在尝试通过 pkg name 获取 {pkg_name} 的信息"
    );
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
            Json(ApiResponse::success(
                json!({"info": info, "metric": metric, "rating": rating, "is_new": is_new}),
                None,
                None,
            ))
        }
        Err(e) => {
            event!(
                Level::WARN,
                "http服务获取 pkg name: {pkg_name} 的信息失败: {e}"
            );
            Json(ApiResponse::error(json!({"error": e.to_string()})))
        }
    }
}

/// 查询应用ID信息
pub async fn query_app_id(
    State(state): State<Arc<AppState>>,
    Path(app_id): Path<String>,
) -> impl IntoResponse {
    event!(
        Level::INFO,
        "http 服务正在尝试通过 appid 获取 {app_id} 的信息"
    );
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
            Json(ApiResponse::success(
                json!({"info": info, "metric": metric, "rating": rating, "is_new": is_new}),
                None,
                None,
            ))
        }
        Err(e) => {
            event!(Level::WARN, "http服务获取 appid: {app_id} 的信息失败: {e}");
            Json(ApiResponse::error(json!({"error": e.to_string()})))
        }
    }
}

/// 获取应用列表统计信息
pub async fn app_list_info(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    event!(Level::INFO, "http 服务正在尝试获取应用列表信息");
    #[derive(serde::Deserialize, serde::Serialize)]
    struct MarketInfo {
        app_count: u32,
        atomic_services_count: u32,
        full_count: u32,
        developer_count: i64,
    }
    match state.db.count_apps().await {
        Ok(app_count) => match state.db.count_atomic_services().await {
            Ok(atomic_services_count) => match state.db.count_developers().await {
                Ok(developer_count) => {
                    let data = MarketInfo {
                        app_count,
                        atomic_services_count,
                        full_count: app_count + atomic_services_count,
                        developer_count,
                    };
                    Json(ApiResponse::success(data, None, None))
                }
                Err(e) => {
                    event!(Level::WARN, "http服务获取开发者数量失败: {e}");
                    Json(ApiResponse::error(json!({"error": "Database error"})))
                }
            },

            Err(e) => {
                event!(Level::WARN, "http服务获取原子服务数量失败: {e}");
                Json(ApiResponse::error(json!({"error": "Database error"})))
            }
        },
        Err(e) => {
            event!(Level::WARN, "http服务获取应用数量失败: {e}");
            Json(ApiResponse::error(json!({"error": "Database error"})))
        }
    }
}

/// 分页获取应用详细信息
pub async fn app_list_paged(
    State(state): State<Arc<AppState>>,
    Path(page): Path<String>,
    Query(query): Query<AppListQuery>,
) -> impl IntoResponse {
    match page.parse::<u32>() {
        Ok(page) => {
            if query.detail() {
                match state
                    .db
                    .get_app_info_paginated_enhanced::<FullAppInfo>(
                        page,
                        query.page_size(),
                        &query.sort_key(),
                        query.desc.unwrap_or_default(),
                        query.search_option(),
                    )
                    .await
                {
                    Ok(apps) => {
                        let total_count = apps.total_count;
                        Json(ApiResponse::success(
                            apps,
                            Some(total_count),
                            Some(query.page_size()),
                        ))
                    }
                    Err(e) => {
                        event!(Level::WARN, "http服务获取分页应用信息失败: {e}");
                        Json(ApiResponse::error("Database error".to_string()))
                    }
                }
            } else {
                match state
                    .db
                    .get_app_info_paginated_enhanced::<ShortAppInfo>(
                        page,
                        query.page_size(),
                        &query.sort_key(),
                        query.desc.unwrap_or_default(),
                        query.search_option(),
                    )
                    .await
                {
                    Ok(apps) => {
                        let total_count = apps.total_count;
                        Json(ApiResponse::success(
                            apps,
                            Some(total_count),
                            Some(query.page_size()),
                        ))
                    }
                    Err(e) => {
                        event!(Level::WARN, "http服务获取分页应用信息失败: {e}");
                        Json(ApiResponse::error("Database error".to_string()))
                    }
                }
            }
        }
        Err(e) => Json(ApiResponse::error(format!("Failed to parse page: {}", e))),
    }
}

/// 获取下载量排行
pub async fn get_download_ranking(
    State(state): State<Arc<AppState>>,
    Query(query): Query<RankingQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(10);
    event!(
        Level::INFO,
        "获取下载量排行，限制: {} exclude 模式: {:?}",
        limit,
        query.exclude_pattern
    );
    match state
        .db
        .get_top_downloads(limit, query.exclude_pattern)
        .await
    {
        Ok(apps) => {
            let total_count = apps.len() as u32;
            Json(ApiResponse::success(apps, Some(total_count), Some(limit)))
        }
        Err(e) => {
            event!(Level::WARN, "获取下载量排行失败: {e}");
            Json(ApiResponse::error("Database error".to_string()))
        }
    }
}

/// 获取评分排行
pub async fn get_rating_ranking(
    State(state): State<Arc<AppState>>,
    Query(query): Query<RankingQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(10);
    event!(Level::INFO, "获取评分排行，限制: {}", limit);

    match state.db.get_top_rated_apps(limit).await {
        Ok(apps) => {
            let total_count = apps.len() as u32;
            Json(ApiResponse::success(apps, Some(total_count), Some(limit)))
        }
        Err(e) => {
            event!(Level::WARN, "获取评分排行失败: {e}");
            Json(ApiResponse::error("Database error".to_string()))
        }
    }
}

/// 获取最近更新排行
pub async fn get_recent_ranking(
    State(state): State<Arc<AppState>>,
    Query(query): Query<RankingQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(10);
    event!(Level::INFO, "获取最近更新排行，限制: {}", limit);

    match state.db.get_recently_updated_apps(limit).await {
        Ok(apps) => {
            let total_count = apps.len() as u32;
            Json(ApiResponse::success(apps, Some(total_count), Some(limit)))
        }
        Err(e) => {
            event!(Level::WARN, "获取最近更新排行失败: {e}");
            Json(ApiResponse::error("Database error".to_string()))
        }
    }
}

/// 获取下载量增长排行
pub async fn get_download_growth_ranking(
    State(state): State<Arc<AppState>>,
    Query(query): Query<RankingQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(10);
    let time_range = query.time_range.unwrap_or_else(|| "7d".to_string());
    event!(
        Level::INFO,
        "获取下载量增长排行，限制: {}, 时间范围: {}",
        limit,
        time_range
    );

    match state.db.get_download_growth_apps(limit, &time_range).await {
        Ok(apps) => {
            let total_count = apps.len() as u32;
            Json(ApiResponse::success(apps, Some(total_count), Some(limit)))
        }
        Err(e) => {
            event!(Level::WARN, "获取下载量增长排行失败: {e}");
            Json(ApiResponse::error("Database error".to_string()))
        }
    }
}

/// 获取评分增长排行
pub async fn get_rating_growth_ranking(
    State(state): State<Arc<AppState>>,
    Query(query): Query<RankingQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(10);
    let time_range = query.time_range.unwrap_or_else(|| "7d".to_string());
    event!(
        Level::INFO,
        "获取评分增长排行，限制: {}, 时间范围: {}",
        limit,
        time_range
    );

    match state.db.get_rating_growth_apps(limit, &time_range).await {
        Ok(apps) => {
            let total_count = apps.len() as u32;
            Json(ApiResponse::success(apps, Some(total_count), Some(limit)))
        }
        Err(e) => {
            event!(Level::WARN, "获取评分增长排行失败: {e}");
            Json(ApiResponse::error("Database error".to_string()))
        }
    }
}

/// 获取开发者排行
pub async fn get_developer_ranking(
    State(state): State<Arc<AppState>>,
    Query(query): Query<RankingQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(10);
    event!(Level::INFO, "获取开发者排行，限制: {}", limit);

    match state.db.get_top_developers(limit).await {
        Ok(developers) => {
            let total_count = developers.len() as u32;
            Json(ApiResponse::success(
                developers,
                Some(total_count),
                Some(limit),
            ))
        }
        Err(e) => {
            event!(Level::WARN, "获取开发者排行失败: {e}");
            Json(ApiResponse::error("Database error".to_string()))
        }
    }
}

/// Get star distribution
pub async fn get_star_distribution(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    event!(Level::INFO, "http 服务正在尝试获取星级分布");
    match state.db.get_star_distribution().await {
        Ok((star_1, star_2, star_3, star_4, star_5)) => Json(ApiResponse::success(
            json!({"star_1": star_1, "star_2": star_2, "star_3": star_3, "star_4": star_4, "star_5": star_5}),
            None,
            None,
        )),
        Err(e) => {
            event!(Level::WARN, "http服务获取星级分布失败: {e}");
            Json(ApiResponse::error(json!({"error": "Database error"})))
        }
    }
}

/// 获取指定应用的下载量历史变化数据
pub async fn get_app_download_history(
    State(state): State<Arc<AppState>>,
    Path(pkg_name): Path<String>,
) -> impl IntoResponse {
    event!(
        Level::INFO,
        "http 服务正在尝试获取应用 {} 的下载量历史数据",
        pkg_name
    );
    match state.db.get_app_metrics_by_pkg_id(&pkg_name).await {
        Ok(metrics) => Json(ApiResponse::success(metrics, None, None)),
        Err(e) => {
            event!(
                Level::WARN,
                "http服务获取应用 {} 下载量历史失败: {e}",
                pkg_name
            );
            Json(ApiResponse::error(json!({"error": "Database error"})))
        }
    }
}
