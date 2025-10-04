use axum::response::{Html, IntoResponse, Redirect};

/// Root path redirect to dashboard
pub async fn redirect_to_dashboard() -> impl IntoResponse {
    Redirect::permanent("/dashboard")
}

// 使用 include_str! 宏在编译时包含静态文件
const DASHBOARD_HTML: &str = include_str!("../../assets/html/main.html");

// const DASHBOARD_JS: &str = include_str!("../../assets/js/dashboard.js");
// const CHART_JS: &str = include_str!("../../assets/js/chart.js");
// const CHARTJS_PLUGIN_DATALABELS_JS: &str =
//     include_str!("../../assets/js/chartjs-plugin-datalabels.js");
// const CHARTJS_ADAPTER_DATE_FNS_JS: &str =
//     include_str!("../../assets/js/chartjs-adapter-date-fns.js");

// // 模块文件
// const UTILS_JS: &str = include_str!("../../assets/js/modules/utils.js");
// const DATA_LOADERS_JS: &str = include_str!("../../assets/js/modules/data-loaders.js");
// const RENDERERS_JS: &str = include_str!("../../assets/js/modules/renderers.js");
// const CHARTS_JS: &str = include_str!("../../assets/js/modules/charts.js");
// const APP_DETAILS_JS: &str = include_str!("../../assets/js/modules/app-details.js");

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

// /// Serve dashboard JavaScript
// pub async fn serve_dashboard_js() -> impl IntoResponse {
//     (
//         [(
//             axum::http::header::CONTENT_TYPE,
//             axum::http::HeaderValue::from_static("application/javascript"),
//         )],
//         DASHBOARD_JS,
//     )
//         .into_response()
// }

// pub async fn serve_chart_js() -> impl IntoResponse {
//     (
//         [(
//             axum::http::header::CONTENT_TYPE,
//             axum::http::HeaderValue::from_static("application/javascript"),
//         )],
//         CHART_JS,
//     )
//         .into_response()
// }

// pub async fn serve_chart_plugin_datalables_js() -> impl IntoResponse {
//     (
//         [(
//             axum::http::header::CONTENT_TYPE,
//             axum::http::HeaderValue::from_static("application/javascript"),
//         )],
//         CHARTJS_PLUGIN_DATALABELS_JS,
//     )
//         .into_response()
// }

// pub async fn serve_chartjs_adapter_date_fns_js() -> impl IntoResponse {
//     (
//         [(
//             axum::http::header::CONTENT_TYPE,
//             axum::http::HeaderValue::from_static("application/javascript"),
//         )],
//         CHARTJS_ADAPTER_DATE_FNS_JS,
//     )
//         .into_response()
// }

// /// Serve utils.js module
// pub async fn serve_utils_js() -> impl IntoResponse {
//     (
//         [(
//             axum::http::header::CONTENT_TYPE,
//             axum::http::HeaderValue::from_static("application/javascript"),
//         )],
//         UTILS_JS,
//     )
//         .into_response()
// }

// /// Serve data-loaders.js module
// pub async fn serve_data_loaders_js() -> impl IntoResponse {
//     (
//         [(
//             axum::http::header::CONTENT_TYPE,
//             axum::http::HeaderValue::from_static("application/javascript"),
//         )],
//         DATA_LOADERS_JS,
//     )
//         .into_response()
// }

// /// Serve renderers.js module
// pub async fn serve_renderers_js() -> impl IntoResponse {
//     (
//         [(
//             axum::http::header::CONTENT_TYPE,
//             axum::http::HeaderValue::from_static("application/javascript"),
//         )],
//         RENDERERS_JS,
//     )
//         .into_response()
// }

// /// Serve charts.js module
// pub async fn serve_charts_js() -> impl IntoResponse {
//     (
//         [(
//             axum::http::header::CONTENT_TYPE,
//             axum::http::HeaderValue::from_static("application/javascript"),
//         )],
//         CHARTS_JS,
//     )
//         .into_response()
// }

// /// Serve app-details.js module
// pub async fn serve_app_details_js() -> impl IntoResponse {
//     (
//         [(
//             axum::http::header::CONTENT_TYPE,
//             axum::http::HeaderValue::from_static("application/javascript"),
//         )],
//         APP_DETAILS_JS,
//     )
//         .into_response()
// }

/// Serve 404 Not Found page
const NOT_FOUND_HTML: &str = r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>404 - 页面未找到 | 华为应用市场看板</title>
    <!-- Tailwind CSS CDN -->
    <script src="https://cdn.tailwindcss.com"></script>
    <style>
        body { font-family: 'Inter', system-ui, -apple-system, sans-serif; background-color: #f8f9fa; }
    </style>
</head>
<body class="bg-gray-50">
    <div class="min-h-screen flex flex-col">
        <!-- Header -->
        <div class="mb-6">
            <div class="bg-white rounded-lg shadow-sm border border-gray-200">
                <div class="p-6">
                    <h1 class="text-2xl font-semibold text-gray-900 mb-3">
                        华为应用市场看板 数据收集自 appgallery api 不保证来源的准确性、完整性和真实性 仅供参考
                    </h1>
                </div>
            </div>
        </div>

        <!-- 404 Content -->
        <div class="flex-grow flex items-center justify-center px-4">
            <div class="text-center">
                <div class="text-9xl font-bold text-gray-200 mb-4">404</div>
                <h2 class="text-3xl font-semibold text-gray-900 mb-2">页面未找到</h2>
                <p class="text-gray-500 mb-8 max-w-md mx-auto">
                    抱歉，您访问的页面不存在。可能路径有误，或者页面已被移除。
                </p>
                <a href="/dashboard" class="inline-flex items-center px-6 py-3 bg-blue-600 text-white text-sm font-medium rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-colors">
                    返回仪表板
                </a>
            </div>
        </div>
    </div>
</body>
</html>"#;

pub async fn serve_not_found() -> impl IntoResponse {
    Html(NOT_FOUND_HTML).into_response()
}
