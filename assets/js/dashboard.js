// 应用市场仪表板 JavaScript 文件
// 提供应用列表、统计图表、搜索排序等功能

// ==============================================
// 全局变量定义
// ==============================================

/**
 * @type {number} 当前页码，从1开始
 */
let currentPage = 1;

/**
 * @type {number} 总页数，根据数据总量和分页大小计算
 */
let totalPages = 1;

/**
 * @type {Object} 当前排序配置
 * @property {string} field - 排序字段名
 * @property {boolean} desc - 是否降序排列
 */
let currentSort = { field: "download_count", desc: true };

/**
 * @type {string} 搜索关键词，用于过滤应用列表
 */
let searchTerm = "";

/**
 * @type {string} 分类过滤器，'all'表示所有分类
 */
let categoryFilter = "all";

/**
 * @type {number} 每页显示的应用数量
 */
const PAGE_SIZE = 20;

/**
 * @type {string} API基础路径，根据部署环境调整
 */
const API_BASE = "/api";

// ==============================================
// 工具函数
// ==============================================

/**
 * 更新最后更新时间戳
 */
function updateLastUpdate() {
    const now = new Date();
    document.getElementById("lastUpdate").textContent =
        now.toLocaleString("zh-CN");
}

// ==============================================
// 事件监听器
// ==============================================

// ESC键关闭详情弹窗
document.addEventListener("keydown", function (event) {
    if (event.key === "Escape") {
        const appModal = document.getElementById("appDetailModal");
        const helpModal = document.getElementById("helpModal");
        if (!appModal.classList.contains("hidden")) {
            appModal.classList.add("hidden");
        }
        if (!helpModal.classList.contains("hidden")) {
            helpModal.classList.add("hidden");
        }
    }
});

// DOM加载完成后初始化
document.addEventListener("DOMContentLoaded", () => {
    DashboardDataLoaders.loadOverview();
    DashboardDataLoaders.loadApps();
    DashboardDataLoaders.loadCategories();
    DashboardCharts.loadCharts();
    updateLastUpdate();

    // Sorting event listeners
    document.querySelectorAll("th[data-sort]").forEach((header) => {
        header.addEventListener("click", () => {
            const field = header.getAttribute("data-sort");
            let desc = false;
            if (field === currentSort.field) {
                desc = currentSort.desc === false ? true : false;
            }
            currentSort = { field, desc };
            DashboardRenderers.updateSortIcons();
            DashboardDataLoaders.loadApps(1);
        });
    });

    // Search button
    document.getElementById("searchBtn").addEventListener("click", () => {
        searchTerm = document.getElementById("searchInput").value.trim();
        categoryFilter = document.getElementById("categoryFilter").value;
        DashboardDataLoaders.loadApps(1);
    });

    // Search input enter key
    document.getElementById("searchInput").addEventListener("keypress", (e) => {
        if (e.key === "Enter") {
            searchTerm = e.target.value.trim();
            categoryFilter = document.getElementById("categoryFilter").value;
            DashboardDataLoaders.loadApps(1);
        }
    });

    // Category filter change
    document.getElementById("categoryFilter").addEventListener("change", (e) => {
        categoryFilter = e.target.value;
        searchTerm = document.getElementById("searchInput").value.trim();
        DashboardDataLoaders.loadApps(1);
    });

    // Refresh button
    document.getElementById("refreshBtn").addEventListener("click", DashboardDataLoaders.refreshData);

    // Help button
    document.getElementById("helpBtn").addEventListener("click", () => {
        document.getElementById("helpModal").classList.remove("hidden");
    });

    // 联系方式 button
    document.getElementById("contactBtn").addEventListener("click", () => {
        document.getElementById("contactModal").classList.remove("hidden");
    });
});
