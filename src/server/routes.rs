use axum::routing::post;
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use std::sync::Arc;
use tower_http::services::ServeDir;

use crate::server::state::{ApiResponse, AppState};
use crate::server::{frontend_handlers, handlers};

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
        // 获取应用下载量历史数据
        .route(
            "/apps/metrics/{pkg_id}",
            get(handlers::get_app_download_history),
        )
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
        // 获取星级分布
        .route(
            "/charts/star-distribution",
            get(handlers::get_star_distribution),
        )
        // 投稿
        .route("/submit", post(handlers::submit_app))
        .fallback(api_not_found)
        .with_state(app_state.clone());

    Router::new()
        // Dashboard routes
        .route("/", get(frontend_handlers::redirect_to_dashboard))
        .route("/dashboard", get(frontend_handlers::serve_dashboard))
        .route("/favicon.ico", get(frontend_handlers::serve_favicon))
        .nest_service("/js", ServeDir::new("assets/js"))
        .nest("/api", api_router)
        .fallback(frontend_handlers::serve_not_found)
        .with_state(app_state)
}

/// API-specific 404 handler returning JSON error
async fn api_not_found() -> impl IntoResponse {
    let payload = ApiResponse::error("Api Not Found");
    (StatusCode::NOT_FOUND, Json(payload))
}
