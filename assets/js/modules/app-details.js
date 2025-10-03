// 应用详情模块
var DashboardAppDetails = (function () {
    /**
     * 在模态框中显示应用详细信息
     * @async
     * @param {string} appId - 应用ID
     */
    async function showAppDetail(appId) {
        try {
            const modal = document.getElementById("appDetailModal");
            const modalContent = document.getElementById("appDetailContent");
            modalContent.innerHTML =
                '<div class="flex justify-center items-center py-8"><div class="w-8 h-8 border-2 border-blue-600 border-t-transparent rounded-full animate-spin"></div></div>';

            const response = await fetch(`${API_BASE}/apps/app_id/${appId}`);
            const data = await response.json();
            const app_info = data.data.info;
            const app_metric = data.data.metric;
            const app_rating = data.data.rating || {};

            const same_css = `class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium`;
            let html = `
                <div class="flex flex-col md:flex-row gap-2">
                    <div class="md:w-1/6 text-center md:text-center">
                        <img src="${app_info.icon_url || "/img/default-app-icon.png"}" class="w-24 h-24 app-icon mx-auto item-center" alt="${app_info.name}">
                        <p class="mb-1 text-lg">${DashboardUtils.renderStars(app_rating.average_rating) || "无评分"}</p>
                        <p class="text-gray-500 mb-2">${app_rating.total_star_rating_count || "无"} 评分</p>
                        <a href="https://appgallery.huawei.com/app/detail?id=${app_info.pkg_name}" target="_blank" class="mt-4 px-4 py-2 bg-blue-600 text-white text-sm font-medium rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
                            在华为应用市场查看
                        </a>
                    </div>
                    <div class="md:w-5/6">
                        <h4 class="text-2xl font-bold text-gray-900 mb-2">${app_info.name || "未知应用"}</h4>
                        <p class="text-gray-600 mb-4">${app_info.developer_name || "未知开发者"}</p>
                        <div class="flex flex-wrap gap-2 mb-4">
                            <span ${same_css} bg-blue-100 text-blue-800">${app_info.kind_type_name || "未知"}-${app_info.kind_name || "未知"}</span>
                            <span ${same_css} bg-purple-100 text-purple-800">${app_metric.version} (${app_metric.version_code})</span>
                            <span ${same_css} bg-cyan-100 text-cyan-800">目标 api 版本 ${app_metric.target_sdk}</span>
                            <span ${same_css} bg-emerald-100 text-emerald-800">最小 api 版本 ${app_metric.minsdk}</span>
                            <span ${same_css} bg-amber-100 text-amber-800">编译 api 版本 ${app_metric.compile_sdk_version}</span>
                        </div>
                        <div class="space-y-2 mb-2">
                            <p><strong>数据更新时间:</strong> <span class="text-gray-600" id="dataUpdateTime">加载中...</span></p>
                            <p><strong>应用更新时间:</strong> <span class="text-gray-600">${DashboardUtils.formatDate(app_metric.release_date)}</span></p>
                            <p><strong>下载量:</strong> <span class="text-gray-600">${DashboardUtils.formatNumber(app_metric.download_count || 0)}</span></p>
                            <p><strong>应用大小:</strong> <span class="text-gray-600">${DashboardUtils.formatSize(app_metric.size_bytes || 0)}</span></p>
                            <p><strong>App ID:</strong> <span class="text-gray-600">${app_info.app_id}</span></p>
                            <p><strong>Package Name:</strong> <span class="text-gray-600">${app_info.pkg_name}</span></p>
                        </div>
                        <hr class="my-4 border-gray-200">
                        <div id="descriptionContainer"></div>
                    </div>
                </div>
                <div class="mt-6">
                    <h5 class="text-lg font-semibold text-gray-900 mb-3">下载量变化趋势</h5>
                    <div class="chart-container" style="height: 300px;">
                        <canvas id="downloadHistoryChart"></canvas>
                    </div>
                    <div id="noHistoryMessage" class="text-center py-4 text-gray-500 hidden">暂无历史下载数据</div>
                </div>
                <div class="mt-6">
                    <h5 class="text-lg font-semibold text-gray-900 mb-3">下载量增量趋势</h5>
                    <div class="chart-container" style="height: 300px;">
                        <canvas id="downloadIncrementChart"></canvas>
                    </div>
                    <div id="noIncrementMessage" class="text-center py-4 text-gray-500 hidden">暂无历史下载数据</div>
                </div>
            `;

            modalContent.innerHTML = html;

            // 处理描述文本展开/收起
            const plainDesc = app_info.description || "无描述";
            const description = plainDesc.replace(/\n/g, "<br>");
            const descContainer = document.getElementById("descriptionContainer");
            const MAX_LENGTH = 200;
            const MAX_LINES = 7;
            let isExpanded = false;

            // 计算行数
            const lineCount = plainDesc.split('\n').length;

            if (plainDesc.length > MAX_LENGTH || lineCount > MAX_LINES) {
                let truncated = "";
                // 如果行数超过限制，按行数截断
                if (lineCount > MAX_LINES) {
                    const lines = plainDesc.split('\n');
                    truncated = lines.slice(0, MAX_LINES).join('\n') + "...";
                } else {
                    // 否则按字符数截断
                    truncated = plainDesc.substring(0, MAX_LENGTH) + "...";
                }
                const truncatedHtml = truncated.replace(/\n/g, "<br>");
                descContainer.innerHTML = `
                    <p id="descriptionText" class="text-gray-700">${truncatedHtml}</p>
                    <button id="toggleDescription" class="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 text-sm font-medium mt-2">展开更多</button>
                `;
                document
                    .getElementById("toggleDescription")
                    .addEventListener("click", function () {
                        if (!isExpanded) {
                            document.getElementById("descriptionText").innerHTML = description;
                            this.textContent = "收起";
                            isExpanded = true;
                        } else {
                            // 收起时也要根据条件显示不同的截断内容
                            let truncated = "";
                            if (lineCount > MAX_LINES) {
                                const lines = plainDesc.split('\n');
                                truncated = lines.slice(0, MAX_LINES).join('\n') + "...";
                            } else {
                                truncated = plainDesc.substring(0, MAX_LENGTH) + "...";
                            }
                            document.getElementById("descriptionText").innerHTML =
                                truncated.replace(/\n/g, "<br>");
                            this.textContent = "展开更多";
                            isExpanded = false;
                        }
                    });
            } else {
                descContainer.innerHTML = `<p class="text-gray-700">${description}</p>`;
            }

            modal.classList.remove("hidden");
            // 异步加载下载历史图表
            if (app_info && app_info.pkg_name) {
                const pkgName = app_info.pkg_name;
                fetch(`${API_BASE}/apps/metrics/${pkgName}`)
                    .then((response) => {
                        if (!response.ok) {
                            throw new Error("Network response was not ok");
                        }
                        return response.json();
                    })
                    .then((historyResult) => {
                        // 原始数据是从新到旧
                        let history = historyResult.data || [];
                        // 更新数据更新时间为历史数据最新记录的时间（在倒序前获取）
                        if (history && history.length > 0) {
                            const latestRecord = history[0]; // 原始数据中最后一个是最新的
                            const dataUpdateTimeElement = document.getElementById("dataUpdateTime");
                            if (dataUpdateTimeElement) {
                                dataUpdateTimeElement.textContent = DashboardUtils.formatDate(latestRecord.created_at);
                            }
                        }
                        // 去重 download_count（原有逻辑）
                        if (Array.isArray(history) && history.length > 1) {
                            const deduped = [history[0]];
                            for (let i = 1; i < history.length; i++) {
                                if (history[i].download_count !== history[i - 1].download_count) {
                                    deduped.push(history[i]);
                                }
                            }
                            history = deduped;
                        }

                        const chartCanvas = document.getElementById("downloadHistoryChart");
                        const incrementCanvas = document.getElementById(
                            "downloadIncrementChart",
                        );
                        const noHistoryMsg = document.getElementById("noHistoryMessage");
                        const noIncrementMsg = document.getElementById("noIncrementMessage");

                        if (history.length > 1) {
                            // 倒序, chart.js 需要从旧到新
                            history.reverse();
                            /**
                             * @type {Array} 下载量历史数据
                             */
                            const downloadData = history.map((item) => ({
                                x: new Date(item.created_at),
                                y: item.download_count,
                            }));

                            // 计算增量数据和每小时增量数据
                            /**
                             * @type {Array} 下载增量数据
                             */
                            const increments = [];
                            /**
                             * @type {Array} 每小时增量数据
                             */
                            const hourlyIncrements = [];

                            if (history.length > 0) {
                                // 添加第一个点，y为0，x为下载量的第一个时间
                                increments.push({
                                    x: new Date(history[0].created_at),
                                    y: 0,
                                });
                                hourlyIncrements.push({
                                    x: new Date(history[0].created_at),
                                    y: 0,
                                });
                            }

                            for (let i = 1; i < history.length; i++) {
                                const increment =
                                    history[i].download_count - history[i - 1].download_count;
                                increments.push({
                                    x: new Date(history[i].created_at),
                                    y: increment,
                                });

                                // 计算每小时增量
                                const timeDiff =
                                    (new Date(history[i].created_at) -
                                        new Date(history[i - 1].created_at)) /
                                    (1000 * 60 * 60); // 转换为小时
                                const hourlyIncrement = timeDiff > 0 ? increment / timeDiff : 0;
                                hourlyIncrements.push({
                                    x: new Date(history[i].created_at),
                                    y: Math.round(hourlyIncrement), // 取整，避免小数过多
                                });
                            }

                            // 把增量的第一个点的y值设置为第二个点的y值
                            if (increments.length > 1) {
                                increments[0].y = increments[1].y;
                                hourlyIncrements[0].y = hourlyIncrements[1].y;
                            }

                            /**
                             * @type {Object} 图表插件配置
                             */
                            const chart_plugin = {
                                legend: { display: true, position: "top" },
                                tooltip: {
                                    callbacks: {
                                        // 顶部标题行（时间）
                                        title: function (contexts) {
                                            const date = new Date(contexts[0].parsed.x);
                                            return DashboardUtils.formatDate(date);
                                        },
                                        label: function (context) {
                                            return `下载量: ${DashboardUtils.formatNumber(context.parsed.y)}`;
                                        },
                                    },
                                },
                            };

                            // 创建下载量图表（原有）
                            const ctx = chartCanvas.getContext("2d");
                            window.downloadHistoryChart = new Chart(ctx, {
                                type: "line",
                                data: {
                                    datasets: [{
                                        label: "下载量",
                                        data: downloadData,
                                        borderColor: "rgb(59, 130, 246)",
                                        backgroundColor: "rgba(59, 130, 246, 0.1)",
                                        fill: true,
                                        tension: 0.1,
                                    },],
                                },
                                options: {
                                    responsive: true,
                                    maintainAspectRatio: false,
                                    scales: {
                                        x: {
                                            type: "time",
                                            title: { display: true, text: "日期" },
                                            time: {
                                                displayFormats: {
                                                    minute: "yyyy-MM-dd HH:mm",
                                                    hour: "yyyy-MM-dd HH:mm",
                                                    day: "yyyy-MM-dd HH:mm",
                                                },
                                            },
                                            ticks: {
                                                callback: function (value) {
                                                    const date = new Date(value);
                                                    return DashboardUtils.formatDate(date);
                                                },
                                            },
                                        },
                                        y: {
                                            beginAtZero: false,
                                            ticks: {
                                                callback: function (value) {
                                                    return DashboardUtils.formatNumber(value);
                                                },
                                            },
                                        },
                                    },
                                    plugins: chart_plugin,
                                },
                            });

                            // 创建增量图表，包含总增量和每小时增量
                            if (increments.length > 0) {
                                const incrementCtx = incrementCanvas.getContext("2d");

                                // 修改tooltip以区分两个数据集
                                /**
                                 * @type {Object} 增量图表插件配置
                                 */
                                const incrementChartPlugin = {
                                    legend: { display: true, position: "top" },
                                    tooltip: {
                                        callbacks: {
                                            title: function (contexts) {
                                                const date = new Date(contexts[0].parsed.x);
                                                return DashboardUtils.formatDate(date);
                                            },
                                            label: function (context) {
                                                const datasetLabel = context.dataset.label || "";
                                                if (datasetLabel === "下载增量") {
                                                    return `下载增量: ${DashboardUtils.formatNumber(context.parsed.y)}`;
                                                } else if (datasetLabel === "每小时增量") {
                                                    return `每小时增量: ${DashboardUtils.formatNumber(context.parsed.y)}`;
                                                }
                                                return `${datasetLabel}: ${DashboardUtils.formatNumber(context.parsed.y)}`;
                                            },
                                        },
                                    },
                                };

                                window.downloadIncrementChart = new Chart(incrementCtx, {
                                    type: "line",
                                    data: {
                                        datasets: [{
                                            label: "下载增量",
                                            data: increments,
                                            borderColor: "rgb(59, 130, 246)",
                                            backgroundColor: "rgba(59, 130, 246, 0.1)",
                                            fill: true,
                                            tension: 0.1,
                                        },
                                        {
                                            label: "每小时增量",
                                            data: hourlyIncrements,
                                            borderColor: "rgb(255, 99, 132)",
                                            backgroundColor: "rgba(255, 99, 132, 0.1)",
                                            fill: true,
                                            tension: 0.1,
                                        },
                                        ],
                                    },
                                    options: {
                                        responsive: true,
                                        maintainAspectRatio: false,
                                        scales: {
                                            x: {
                                                type: "time",
                                                title: { display: true, text: "日期" },
                                                time: {
                                                    displayFormats: {
                                                        minute: "yyyy-MM-dd HH:mm",
                                                        hour: "yyyy-MM-dd HH:mm",
                                                        day: "yyyy-MM-dd HH:mm",
                                                    },
                                                },
                                                ticks: {
                                                    callback: function (value) {
                                                        const date = new Date(value);
                                                        return DashboardUtils.formatDate(date);
                                                    },
                                                },
                                            },
                                            y: {
                                                beginAtZero: false,
                                                title: { display: true, text: "下载量" },
                                                ticks: {
                                                    callback: function (value) {
                                                        return DashboardUtils.formatNumber(value);
                                                    },
                                                },
                                            },
                                        },
                                        plugins: incrementChartPlugin,
                                    },
                                });
                                incrementCanvas.style.display = "block";
                                noIncrementMsg.classList.add("hidden");
                            } else {
                                incrementCanvas.style.display = "none";
                                noIncrementMsg.classList.remove("hidden");
                            }

                            // 显示原有图表
                            chartCanvas.style.display = "block";
                            noHistoryMsg.classList.add("hidden");
                        } else {
                            // 无数据：隐藏两个图表
                            chartCanvas.style.display = "none";
                            incrementCanvas.style.display = "none";
                            noHistoryMsg.classList.remove("hidden");
                            noIncrementMsg.classList.remove("hidden");
                        }
                    })
                    .catch((historyError) => {
                        // 错误处理（原有，扩展到两个图表）
                        console.error("Failed to load download history:", historyError);
                        document.getElementById("downloadHistoryChart").style.display =
                            "none";
                        document.getElementById("downloadIncrementChart").style.display =
                            "none";
                        document.getElementById("noHistoryMessage").innerHTML =
                            "加载历史数据失败";
                        document.getElementById("noIncrementMessage").innerHTML =
                            "加载历史数据失败";
                        document
                            .getElementById("noHistoryMessage")
                            .classList.remove("hidden");
                        document
                            .getElementById("noIncrementMessage")
                            .classList.remove("hidden");
                    });
            } else {
                document.getElementById("downloadHistoryChart").style.display = "none";
                document.getElementById("noHistoryMessage").classList.remove("hidden");
            }
        } catch (error) {
            console.error("Failed to load app details:", error);
            document.getElementById("appDetailContent").innerHTML =
                '<div class="text-center py-4 text-red-500">Failed to load details</div>';
            document.getElementById("appDetailModal").classList.remove("hidden");
        }
    }

    return {
        showAppDetail: showAppDetail
    };
})();
