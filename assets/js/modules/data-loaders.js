// 数据加载模块
var DashboardDataLoaders = (function() {
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

            // 更新统计数据到页面
            document.getElementById("totalCount").textContent = DashboardUtils.formatNumber(
                (market_info.data.app_count || 0) +
                (market_info.data.atomic_services_count || 0),
            );
            document.getElementById("appCount").textContent = DashboardUtils.formatNumber(
                market_info.data.app_count || 0,
            );
            document.getElementById("atomicServiceCount").textContent = DashboardUtils.formatNumber(
                market_info.data.atomic_services_count || 0,
            );
            document.getElementById("developerCount").textContent = DashboardUtils.formatNumber(
                market_info.data.developer_count || 0,
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
        search = searchTerm,
        category = categoryFilter,
    ) {
        try {
            const tableBody = document.getElementById("appTableBody");
            tableBody.innerHTML =
                '<tr><td colspan="8" class="text-center py-12"><div class="inline-block w-8 h-8 border-2 border-blue-600 border-t-transparent rounded-full animate-spin"></div></td></tr>';

            // 构建API请求URL
            let url = `${API_BASE}/apps/list/${page}?sort=${sortField}&desc=${sort_desc}&page_size=${PAGE_SIZE}`;
            if (search) url += `&search=${encodeURIComponent(search)}`;
            if (category && category !== "all")
                url += `&category=${encodeURIComponent(category)}`;

            const response = await fetch(url);
            const data = await response.json();

            // 更新分页信息
            if (data.data && data.data.total_count) {
                totalPages = Math.ceil(data.data.total_count / PAGE_SIZE);
                currentPage = page;
            }

            let apps = data.data.data || [];
            // 客户端额外过滤（如果需要）
            if (search) {
                apps = apps.filter((app) =>
                    app.name.toLowerCase().includes(search.toLowerCase()),
                );
            }
            if (category && category !== "all") {
                apps = apps.filter(
                    (app) => app.category.toLowerCase() === category.toLowerCase(),
                );
            }

            DashboardRenderers.renderApps(apps.slice(0, PAGE_SIZE)); // 客户端分页（如果需要）
            DashboardRenderers.renderPagination();
        } catch (error) {
            console.error("加载应用列表失败:", error);
            document.getElementById("appTableBody").innerHTML =
                '<tr><td colspan="8" class="text-center py-4 text-gray-500">加载数据失败</td></tr>';
        }
    }

    /**
     * 加载应用分类列表
     * @async
     */
    async function loadCategories() {
        try {
            const categorySelect = document.getElementById("categoryFilter");
            categorySelect.innerHTML = '<option value="all">所有分类</option>';

            // 硬编码分类列表（可从API获取）
            const categories = [
                { value: "games", label: "游戏" },
                { value: "social", label: "社交" },
                { value: "productivity", label: "生产力" },
                { value: "entertainment", label: "娱乐" },
                { value: "education", label: "教育" },
                { value: "lifestyle", label: "生活" },
                { value: "utilities", label: "工具" },
            ];

            categories.forEach((cat) => {
                const option = document.createElement("option");
                option.value = cat.value;
                option.textContent = cat.label;
                categorySelect.appendChild(option);
            });
        } catch (error) {
            console.error("加载分类列表失败:", error);
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
        loadCategories: loadCategories,
        refreshData: refreshData
    };
})();