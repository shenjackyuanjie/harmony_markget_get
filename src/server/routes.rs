use axum::{Router, routing::get};
use std::sync::Arc;

use crate::server::handlers;
use crate::server::state::AppState;

/// 创建应用路由
pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        // 规范化后的原有路由
        // 根据包名查询应用信息
        .route("/api/apps/by-pkg-name/{pkg_name}", get(handlers::query_pkg))
        // 根据应用ID查询应用信息
        .route("/api/apps/by-app-id/{app_id}", get(handlers::query_app_id))
        // 获取应用列表基本信息
        .route("/api/apps/list/info", get(handlers::app_list_info))
        // 获取分页的应用详细信息
        .route(
            "/api/apps/list/{page_count}/detail",
            get(handlers::app_list_paged),
        )
        // 获取分页的应用简要信息
        .route(
            "/api/apps/list/{page_count}",
            get(handlers::app_list_paged_short),
        )
        // 新增排行API路由
        // 获取下载量排行榜
        .route(
            "/api/rankings/downloads",
            get(handlers::get_download_ranking),
        )
        // 获取评分排行榜
        .route("/api/rankings/ratings", get(handlers::get_rating_ranking))
        // 获取最新应用排行榜
        .route("/api/rankings/recent", get(handlers::get_recent_ranking))
        // 获取价格排行榜
        .route("/api/rankings/prices", get(handlers::get_price_ranking))
        // 获取评分数量排行榜
        .route(
            "/api/rankings/rating-counts",
            get(handlers::get_rating_count_ranking),
        )
        // 获取下载量增长率排行榜
        .route(
            "/api/rankings/download-growth",
            get(handlers::get_download_growth_ranking),
        )
        // 获取评分增长率排行榜
        .route(
            "/api/rankings/rating-growth",
            get(handlers::get_rating_growth_ranking),
        )
        // 获取开发者排行榜
        .route(
            "/api/rankings/developers",
            get(handlers::get_developer_ranking),
        )
        // 获取应用大小排行榜
        .route("/api/rankings/sizes", get(handlers::get_size_ranking))
        // 应用基本信息API
        // 通过应用ID获取应用基本信息
        .route("/api/apps/{app_id}", get(handlers::query_app_id))
        .with_state(app_state)
}
