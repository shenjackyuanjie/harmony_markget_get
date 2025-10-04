// 数据加载模块
var DashboardDataLoaders = (function () {
    /**
     * 加载应用市场概览统计信息
     * @async
     */
    async function loadOverview() {
        try {
            // 显示加载指示器
            const loadingElements = [
                "loadingOverview",
                "loadingDeveloperCount",
                "loadingAtomicServiceCount",
                "loadingTotalCount",
            ];
            loadingElements.forEach((id) => {
                const el = document.getElementById(id);
                if (el) el.style.display = "inline-block";
            });

            // 获取应用统计
            const appResponse = await fetch(`${API_BASE}/market_info`);
            const market_info = await appResponse.json();
            const app_count = market_info.data.app_count;

            // 更新统计数据到页面
            document.getElementById("totalCount").textContent = DashboardUtils.formatNumber(
                app_count.total
            );
            document.getElementById("appCount").textContent = DashboardUtils.formatNumber(
                app_count.apps
            );
            document.getElementById("atomicServiceCount").textContent = DashboardUtils.formatNumber(
                app_count.atomic_services
            );
            document.getElementById("developerCount").textContent = DashboardUtils.formatNumber(
                market_info.data.developer_count
            );

            // 隐藏加载指示器
            loadingElements.forEach((id) => {
                const el = document.getElementById(id);
                if (el) el.style.display = "none";
            });
        } catch (error) {
            console.error("加载概览统计失败:", error);
            // 错误时隐藏加载指示器
            loadingElements.forEach((id) => {
                const el = document.getElementById(id);
                if (el) el.style.display = "none";
            });
        }
    }

    /**
     * 加载应用列表，支持分页、排序、搜索和分类过滤
     * @async
     * @param {number} [page=1] - 页码
     * @param {string} [sortField=currentSort.field] - 排序字段
     * @param {boolean} [sort_desc=currentSort.desc] - 是否降序
     * @param {string} [search=searchTerm] - 搜索关键词
     * @param {string} [category=categoryFilter] - 分类过滤
     */
    async function loadApps(
        page = 1,
        sortField = currentSort.field,
        sort_desc = currentSort.desc,
        search_value = searchTerm,
        search_key = searchKey,
        search_exact = searchExact,
    ) {
        totalPages = 1; // 清空
        DashboardRenderers.renderPagination(); // 清空分页
        try {
            const tableBody = document.getElementById("appTableBody");
            tableBody.innerHTML =
                '<tr><td colspan="8" class="text-center py-12"><div class="inline-block w-8 h-8 border-2 border-blue-600 border-t-transparent rounded-full animate-spin"></div></td></tr>';

            // 构建API请求URL
            let url = `${API_BASE}/apps/list/${page}?sort=${sortField}&desc=${sort_desc}&page_size=${PAGE_SIZE}`;
            if (search_value) {
                url += `&search_key=${encodeURIComponent(search_key)}&search_value=${encodeURIComponent(search_value)}&search_exact=${search_exact}`;
            }

            const response = await fetch(url);
            const data = await response.json();

            // 更新分页信息
            if (data.data && data.data.total_count) {
                totalPages = Math.ceil(data.data.total_count / PAGE_SIZE);
                currentPage = page;
            }

            let apps = data.data.data || [];

            DashboardRenderers.renderApps(apps.slice(0, PAGE_SIZE)); // 客户端分页（如果需要）
            DashboardRenderers.renderPagination();
        } catch (error) {
            console.error("加载应用列表失败:", error);
            document.getElementById("appTableBody").innerHTML =
                '<tr><td colspan="8" class="text-center py-4 text-gray-500">加载数据失败</td></tr>';
        }
    }

    /**
     * 刷新所有数据
     * @async
     */
    async function refreshData() {
        await loadOverview();
        await loadApps();
        DashboardCharts.loadCharts();
        updateLastUpdate();
    }

    return {
        loadOverview: loadOverview,
        loadApps: loadApps,
        refreshData: refreshData
    };
})();
