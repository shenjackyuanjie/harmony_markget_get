use axum::{
    Json,
    extract::{Path, Query, State},
    response::{Html, IntoResponse, Redirect},
};
use serde_json::json;
use tracing::{Level, event};

use crate::model::{AppInfo, AppMetric, AppQuery, AppRating};

use super::state::{ApiResponse, RankingQuery};

/// 查询应用包名信息
pub async fn query_pkg(
    State(state): State<std::sync::Arc<super::state::AppState>>,
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
    State(state): State<std::sync::Arc<super::state::AppState>>,
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
pub async fn app_list_info(
    State(state): State<std::sync::Arc<super::state::AppState>>,
) -> impl IntoResponse {
    event!(Level::INFO, "http 服务正在尝试获取应用列表信息");
    match state.db.count_apps().await {
        Ok(app_count) => match state.db.count_atomic_services().await {
            Ok(atomic_services_count) => Json(ApiResponse::success(
                json!({"app_count": app_count, "atomic_services_count": atomic_services_count}),
                None,
                None,
            )),
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
    State(state): State<std::sync::Arc<super::state::AppState>>,
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
                Ok(apps) => {
                    let total_count = apps.total_count;
                    Json(ApiResponse::success(
                        apps,
                        Some(total_count),
                        Some(PAGE_BATCH),
                    ))
                }
                Err(e) => {
                    event!(Level::WARN, "http服务获取分页应用信息失败: {e}");
                    Json(ApiResponse::error("Database error".to_string()))
                }
            }
        }
        Err(e) => Json(ApiResponse::error(format!("Failed to parse page: {}", e))),
    }
}

/// 分页获取应用简略信息
pub async fn app_list_paged_short(
    State(state): State<std::sync::Arc<super::state::AppState>>,
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
                Ok(apps) => {
                    let total_count = apps.total_count;
                    Json(ApiResponse::success(
                        apps,
                        Some(total_count),
                        Some(PAGE_BATCH),
                    ))
                }
                Err(e) => {
                    event!(Level::WARN, "http服务获取分页应用简略信息失败: {e}");
                    Json(ApiResponse::error("Database error".to_string()))
                }
            }
        }
        Err(e) => Json(ApiResponse::error(format!("Failed to parse page: {}", e))),
    }
}

/// 获取下载量排行
pub async fn get_download_ranking(
    State(state): State<std::sync::Arc<super::state::AppState>>,
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
    State(state): State<std::sync::Arc<super::state::AppState>>,
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
    State(state): State<std::sync::Arc<super::state::AppState>>,
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

/// 获取价格排行
pub async fn get_price_ranking(
    State(state): State<std::sync::Arc<super::state::AppState>>,
    Query(query): Query<RankingQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(10);
    event!(Level::INFO, "获取价格排行，限制: {}", limit);

    match state.db.get_top_priced_apps(limit).await {
        Ok(apps) => {
            let total_count = apps.len() as u32;
            Json(ApiResponse::success(apps, Some(total_count), Some(limit)))
        }
        Err(e) => {
            event!(Level::WARN, "获取价格排行失败: {e}");
            Json(ApiResponse::error("Database error".to_string()))
        }
    }
}

/// 获取评分人数排行
pub async fn get_rating_count_ranking(
    State(state): State<std::sync::Arc<super::state::AppState>>,
    Query(query): Query<RankingQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(10);
    event!(Level::INFO, "获取评分人数排行，限制: {}", limit);

    match state.db.get_top_rated_count_apps(limit).await {
        Ok(apps) => {
            let total_count = apps.len() as u32;
            Json(ApiResponse::success(apps, Some(total_count), Some(limit)))
        }
        Err(e) => {
            event!(Level::WARN, "获取评分人数排行失败: {e}");
            Json(ApiResponse::error("Database error".to_string()))
        }
    }
}

/// 获取下载量增长排行
pub async fn get_download_growth_ranking(
    State(state): State<std::sync::Arc<super::state::AppState>>,
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
    State(state): State<std::sync::Arc<super::state::AppState>>,
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
    State(state): State<std::sync::Arc<super::state::AppState>>,
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

/// 获取应用大小排行
pub async fn get_size_ranking(
    State(state): State<std::sync::Arc<super::state::AppState>>,
    Query(query): Query<RankingQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(10);
    event!(Level::INFO, "获取应用大小排行，限制: {}", limit);

    match state.db.get_largest_apps(limit).await {
        Ok(apps) => {
            let total_count = apps.len() as u32;
            Json(ApiResponse::success(apps, Some(total_count), Some(limit)))
        }
        Err(e) => {
            event!(Level::WARN, "获取应用大小排行失败: {e}");
            Json(ApiResponse::error("Database error".to_string()))
        }
    }
}

/// Get developer count
pub async fn get_developer_count(
    State(state): State<std::sync::Arc<super::state::AppState>>,
) -> impl IntoResponse {
    event!(Level::INFO, "http 服务正在尝试获取开发者数量");
    match state.db.count_developers().await {
        Ok(count) => Json(ApiResponse::success(
            json!({"developer_count": count}),
            None,
            None,
        )),
        Err(e) => {
            event!(Level::WARN, "http服务获取开发者数量失败: {e}");
            Json(ApiResponse::error(json!({"error": "Database error"})))
        }
    }
}
/// Get star distribution
pub async fn get_star_distribution(
    State(state): State<std::sync::Arc<super::state::AppState>>,
) -> impl IntoResponse {
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

/// Root path redirect to dashboard
pub async fn redirect_to_dashboard() -> impl IntoResponse {
    Redirect::permanent("/dashboard")
}

// 使用 include_str! 宏在编译时包含静态文件
const DASHBOARD_HTML: &str = include_str!("../../assets/html/main.html");
const DASHBOARD_JS: &str = include_str!("../../assets/js/dashboard.js");
const FAVICON_ICO: &[u8] = include_bytes!("../../assets/icon/favicon.ico");

/// Serve dashboard HTML
pub async fn serve_dashboard() -> impl IntoResponse {
    Html(DASHBOARD_HTML).into_response()
}

pub async fn serve_favicon() -> impl IntoResponse {
    (
        [(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("image/x-icon"),
        )],
        FAVICON_ICO,
    )
        .into_response()
}

/// Serve dashboard JavaScript
pub async fn serve_dashboard_js() -> impl IntoResponse {
    (
        [(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("application/javascript"),
        )],
        DASHBOARD_JS,
    )
        .into_response()
}
