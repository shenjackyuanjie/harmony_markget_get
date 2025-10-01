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
 * 解析链接提取包名
 */
function parseLink() {
    const linkInput = document.getElementById("linkInput");
    const pkgInput = document.getElementById("pkgInput");
    const linkError = document.getElementById("linkError");

    const link = linkInput.value.trim();
    if (!link) {
        linkError.classList.add("hidden");
        return;
    }

    const regex = /(?<=id=)[\w\.]+/;
    const match = link.match(regex);
    if (match) {
        pkgInput.value = match[0];
        linkError.classList.add("hidden");
    } else {
        pkgInput.value = "";
        linkError.classList.remove("hidden");
    }
}

/**
 * 校验输入
 * @returns {boolean} 是否通过校验
 */
function validateInputs() {
    const pkgInput = document.getElementById("pkgInput").value.trim();
    const appIdInput = document.getElementById("appIdInput").value.trim();
    const pkgError = document.getElementById("pkgError");
    const appIdError = document.getElementById("appIdError");

    let valid = true;

    // 二选一必填
    if (!pkgInput && !appIdInput) {
        pkgError.textContent = "包名或 app_id 必须填写其中一个";
        pkgError.classList.remove("hidden");
        appIdError.classList.add("hidden");
        valid = false;
    } else if (pkgInput && !appIdInput) {
        // 校验包名
        const pkgRegex = /^[a-zA-Z0-9_\.]+$/;
        if (!pkgRegex.test(pkgInput)) {
            pkgError.classList.remove("hidden");
            valid = false;
        } else {
            pkgError.classList.add("hidden");
        }
        appIdError.classList.add("hidden");
    } else if (!pkgInput && appIdInput) {
        // 校验 app_id 非空
        if (!appIdInput) {
            appIdError.classList.remove("hidden");
            valid = false;
        } else {
            appIdError.classList.add("hidden");
        }
        pkgError.classList.add("hidden");
    } else {
        // 两者都填，优先用包名
        const pkgRegex = /^[a-zA-Z0-9_\.]+$/;
        if (!pkgRegex.test(pkgInput)) {
            pkgError.classList.remove("hidden");
            valid = false;
        } else {
            pkgError.classList.add("hidden");
        }
        appIdError.classList.add("hidden");
    }

    return valid;
}

/**
 * 查询应用
 */
async function queryApp() {
    const pkgName = document.getElementById("pkgInput").value.trim();
    const appId = document.getElementById("appIdInput").value.trim();
    const resultArea = document.getElementById("resultArea");
    const resultContent = document.getElementById("resultContent");

    let url;
    if (pkgName) {
        url = `/api/apps/pkg_name/${encodeURIComponent(pkgName)}`;
    } else if (appId) {
        url = `/api/apps/app_id/${encodeURIComponent(appId)}`;
    } else {
        alert("请输入包名或 app_id");
        return;
    }

    try {
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error("查询失败");
        }
        const data = await response.json();
        if (data.success) {
            renderResult(data.data);
            resultArea.classList.remove("hidden");
        } else {
            alert("查询失败，请检查输入");
            resultArea.classList.add("hidden");
        }
    } catch (error) {
        console.error("查询错误:", error);
        alert("查询失败，请检查输入");
        resultArea.classList.add("hidden");
    }
}

/**
 * 渲染查询结果
 * @param {Object} data - API 返回数据
 */
function renderResult(data) {
    const resultContent = document.getElementById("resultContent");
    const { info, metric, rating } = data;
    const new_info = data.is_new[0];
    const new_metric = data.is_new[1];

    if (!info || !metric) {
        resultContent.innerHTML = "<p class='text-red-500'>未找到应用信息</p>";
        return;
    }

    const iconUrl = info.icon_url || '';
    const downloadCount = metric.download_count || 0;
    const name = info.name || '未知应用';

    resultContent.innerHTML = `
        <div class="flex items-center space-x-3 mb-2">
            ${iconUrl ? `<img src="${iconUrl}" alt="图标" class="w-12 h-12 rounded-md object-cover">` : '<div class="w-12 h-12 bg-gray-300 rounded-md"></div>'}
            <div>
                <h4 class="font-semibold">${name}</h4>
                <p class="text-sm text-gray-600">包名: ${info.pkg_name || 'N/A'}</p>
                ${rating ? `<p class="text-sm text-gray-600">评分: ${rating.average_rating || 'N/A'} (${rating.total_ratings || 0} 人评价)</p>` : ''}
                ${new_info ? `<span class="bg-green-100 text-green-800 px-2 py-1 rounded-full text-xs ml-2">应用信息有更新</span>` : ''}
                ${new_metric ? `<span class="bg-green-100 text-green-800 px-2 py-1 rounded-full text-xs ml-2">指标数据有更新</span>` : ''}
                ${(!new_info && !new_metric) ? `<span class="bg-gray-100 text-gray-800 px-2 py-1 rounded-full text-xs ml-2">暂无数据更新</span>` : ''}
            </div>
        </div>
        <div class="grid grid-cols-2 gap-2 text-sm">
            <div><strong>下载量:</strong> ${downloadCount > 10000 ? (downloadCount / 10000).toFixed(1) + '万' : downloadCount}</div>
            ${metric.size_bytes ? `<div><strong>大小:</strong> ${(metric.size_bytes / 1024 / 1024).toFixed(1)} MB</div>` : ''}
            ${metric.version ? `<div><strong>版本:</strong> ${metric.version}</div>` : ''}
            ${metric.price ? `<div><strong>价格:</strong> ${metric.price}</div>` : ''}
            ${info.developer_name ? `<div><strong>开发者:</strong> ${info.developer_name}</div>` : ''}
            ${info.kind_name ? `<div><strong>分类:</strong> ${info.kind_name}</div>` : ''}
        </div>
    `;
}

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

    // 投稿按钮
    document.getElementById("submitBtn").addEventListener("click", () => {
        document.getElementById("submitModal").classList.remove("hidden");
        // 清空输入和错误
        document.getElementById("linkInput").value = "";
        document.getElementById("pkgInput").value = "";
        document.getElementById("appIdInput").value = "";
        document.getElementById("linkError").classList.add("hidden");
        document.getElementById("pkgError").classList.add("hidden");
        document.getElementById("appIdError").classList.add("hidden");
        document.getElementById("resultArea").classList.add("hidden");
    });

    // 关闭投稿模态框
    document.getElementById("closeSubmitModal").addEventListener("click", () => {
        document.getElementById("submitModal").classList.add("hidden");
    });

    // 链接输入事件
    document.getElementById("linkInput").addEventListener("input", parseLink);

    // 包名输入事件，隐藏错误
    document.getElementById("pkgInput").addEventListener("input", () => {
        document.getElementById("pkgError").classList.add("hidden");
    });

    // app_id 输入事件，隐藏错误
    document.getElementById("appIdInput").addEventListener("input", () => {
        document.getElementById("appIdError").classList.add("hidden");
    });

    // 查询按钮
    document.getElementById("queryBtn").addEventListener("click", () => {
        if (validateInputs()) {
            queryApp();
        }
    });
});
