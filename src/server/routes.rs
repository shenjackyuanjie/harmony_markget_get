use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use std::sync::Arc;

use crate::server::state::{ApiResponse, AppState};
use crate::server::{handle_static, handlers};

/// 创建应用路由
pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api_router = Router::new()
        // 根据包名查询应用信息
        .route("/apps/pkg_name/{pkg_name}", get(handlers::query_pkg))
        // 根据应用ID查询应用信息
        .route("/apps/app_id/{app_id}", get(handlers::query_app_id))
        // 获取市场信息
        .route("/market_info", get(handlers::app_list_info))
        // 获取分页的应用信息
        .route("/apps/list/{page_count}", get(handlers::app_list_paged))
        // 新增排行API路由
        // 获取下载量排行榜
        .route(
            "/rankings/top-downloads",
            get(handlers::get_download_ranking),
        )
        // 获取评分排行榜
        .route("/rankings/ratings", get(handlers::get_rating_ranking))
        // 获取最新应用排行榜
        .route("/rankings/recent", get(handlers::get_recent_ranking))
        // 获取价格排行榜
        .route("/rankings/prices", get(handlers::get_price_ranking))
        // 获取评分数量排行榜
        .route(
            "/rankings/rating-counts",
            get(handlers::get_rating_count_ranking),
        )
        // 获取下载量增长率排行榜
        .route(
            "/rankings/download-growth",
            get(handlers::get_download_growth_ranking),
        )
        // 获取评分增长率排行榜
        .route(
            "/rankings/rating-growth",
            get(handlers::get_rating_growth_ranking),
        )
        // 获取开发者排行榜
        .route("/rankings/developers", get(handlers::get_developer_ranking))
        // 获取应用大小排行榜
        .route("/rankings/sizes", get(handlers::get_size_ranking))
        // 获取星级分布
        .route(
            "/charts/star-distribution",
            get(handlers::get_star_distribution),
        )
        .fallback(api_not_found)
        .with_state(app_state.clone());

    Router::new()
        // Dashboard routes
        .route("/", get(handle_static::redirect_to_dashboard))
        .route("/dashboard", get(handle_static::serve_dashboard))
        .route("/js/dashboard.js", get(handle_static::serve_dashboard_js))
        .route("/favicon.ico", get(handle_static::serve_favicon))
        .nest("/api", api_router)
        .fallback(handle_static::serve_not_found)
        .with_state(app_state)
}

/// API-specific 404 handler returning JSON error
async fn api_not_found() -> impl IntoResponse {
    let payload = ApiResponse::error("Api Not Found");
    (StatusCode::NOT_FOUND, Json(payload))
}
