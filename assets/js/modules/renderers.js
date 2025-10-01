// 渲染模块
var DashboardRenderers = (function() {
    /**
     * 渲染应用列表到表格
     * @param {Array} apps - 应用数据数组
     */
    function renderApps(apps) {
        const tableBody = document.getElementById("appTableBody");
        tableBody.innerHTML = "";

        if (!apps || apps.length === 0) {
            tableBody.innerHTML =
                '<tr><td colspan="9" class="text-center py-4 text-gray-500">未找到应用</td></tr>';
            return;
        }

        apps.forEach((app, index) => {
            const app_info = app.info;
            const app_metric = app.metric;
            const app_rating = app.rating || {};

            const tr = document.createElement("tr");
            tr.className = "hover:bg-gray-50 cursor-pointer transition-colors";
            tr.onclick = () => DashboardAppDetails.showAppDetail(app_info.app_id);
            tr.innerHTML = `
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">${(currentPage - 1) * PAGE_SIZE + index + 1}</td>
                <td class="px-6 py-4 whitespace-nowrap">
                    <div class="flex items-center">
                        <img src="${app_info.icon_url || "/img/default-app-icon.png"}" class="app-icon mr-3" alt="${app_info.name}">
                        <span class="font-medium text-gray-900">${app_info.name || "未知"}</span>
                    </div>
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">${app_info.developer_name || "未知"}</td>
                <td class="px-6 py-4 whitespace-nowrap">
                    <span class="inline-flex items-center px-2 py-1 text-xs font-medium rounded-full bg-blue-100 text-blue-800">${app_info.kind_type_name || "未知"}-${app_info.kind_name || "未知"}</span>
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">${DashboardUtils.renderStars(app_rating.average_rating)}</td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">${DashboardUtils.formatNumber(app_metric.download_count || 0)}</td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">${DashboardUtils.formatSize(app_metric.size_bytes || 0)}</td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">${app_metric.created_at ? DashboardUtils.formatDate(app_metric.created_at) : "未知"}</td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">${app_info.created_at ? new Date(app_info.created_at).toLocaleDateString("zh-CN") : "未知"}</td>
            `;

            tableBody.appendChild(tr);
        });
    }

    /**
     * 更新表格排序图标状态
     */
    function updateSortIcons() {
        const headers = document.querySelectorAll("th[data-sort]");
        headers.forEach((header) => {
            const field = header.getAttribute("data-sort");
            const span = header.querySelector("span");
            if (!span) return;

            if (field === currentSort.field) {
                span.textContent = currentSort.desc === false ? "↑" : "↓";
                header.classList.add("bg-gray-100"); // 激活状态
            } else {
                span.textContent = "↑";
                header.classList.remove("bg-gray-100");
            }
        });
    }

    /**
     * 渲染分页控件（使用Tailwind CSS样式）
     */
    function renderPagination() {
        const paginationEl = document.getElementById("pagination");
        paginationEl.innerHTML = "";

        if (totalPages <= 1) return;

        const ul = document.createElement("ul");
        ul.className = "flex items-center space-x-1";

        // 第一页按钮
        const firstLi = document.createElement("li");
        firstLi.className = `flex ${currentPage === 1 ? "opacity-50 cursor-not-allowed" : "cursor-pointer"}`;
        const firstA = document.createElement("a");
        firstA.className =
            "px-3 py-2 text-sm font-medium rounded-md border border-gray-300 bg-white text-gray-500 hover:bg-gray-50";
        firstA.textContent = "第一页";
        if (currentPage > 1) {
            firstA.onclick = (e) => {
                e.preventDefault();
                DashboardDataLoaders.loadApps(1);
            };
        }
        firstLi.appendChild(firstA);
        ul.appendChild(firstLi);

        // 上一页按钮
        const prevLi = document.createElement("li");
        prevLi.className = `flex ${currentPage === 1 ? "opacity-50 cursor-not-allowed" : "cursor-pointer"}`;
        const prevA = document.createElement("a");
        prevA.className =
            "px-3 py-2 text-sm font-medium rounded-md border border-gray-300 bg-white text-gray-500 hover:bg-gray-50";
        prevA.textContent = "上一页";
        if (currentPage > 1) {
            prevA.onclick = (e) => {
                e.preventDefault();
                DashboardDataLoaders.loadApps(currentPage - 1);
            };
        }
        prevLi.appendChild(prevA);
        ul.appendChild(prevLi);

        // 页码数字
        const startPage = Math.max(1, currentPage - 2);
        const endPage = Math.min(totalPages, currentPage + 2);
        for (let i = startPage; i <= endPage; i++) {
            const li = document.createElement("li");
            li.className = `flex ${i === currentPage ? "z-10" : ""}`;
            const a = document.createElement("a");
            a.className = `px-3 py-2 text-sm font-medium rounded-md border ${
                i === currentPage
                    ? "border-blue-500 bg-blue-50 text-blue-600"
                    : "border-gray-300 bg-white text-gray-500 hover:bg-gray-50"
            }`;
            a.textContent = i;
            a.onclick = (e) => {
                e.preventDefault();
                DashboardDataLoaders.loadApps(i);
            };
            li.appendChild(a);
            ul.appendChild(li);
        }

        // 下一页按钮
        const nextLi = document.createElement("li");
        nextLi.className = `flex ${currentPage === totalPages ? "opacity-50 cursor-not-allowed" : "cursor-pointer"}`;
        const nextA = document.createElement("a");
        nextA.className =
            "px-3 py-2 text-sm font-medium rounded-md border border-gray-300 bg-white text-gray-500 hover:bg-gray-50";
        nextA.textContent = "下一页";
        if (currentPage < totalPages) {
            nextA.onclick = (e) => {
                e.preventDefault();
                DashboardDataLoaders.loadApps(currentPage + 1);
            };
        }
        nextLi.appendChild(nextA);
        ul.appendChild(nextLi);

        // 最后一页按钮
        const lastLi = document.createElement("li");
        lastLi.className = `flex ${currentPage === totalPages ? "opacity-50 cursor-not-allowed" : "cursor-pointer"}`;
        const lastA = document.createElement("a");
        lastA.className =
            "px-3 py-2 text-sm font-medium rounded-md border border-gray-300 bg-white text-gray-500 hover:bg-gray-50";
        lastA.textContent = "最后一页";
        if (currentPage < totalPages) {
            lastA.onclick = (e) => {
                e.preventDefault();
                DashboardDataLoaders.loadApps(totalPages);
            };
        }
        lastLi.appendChild(lastA);
        ul.appendChild(lastLi);

        paginationEl.appendChild(ul);
    }

    return {
        renderApps: renderApps,
        renderPagination: renderPagination,
        updateSortIcons: updateSortIcons
    };
})();
