// 图表模块
var DashboardCharts = (function() {
    let starChart = null;

    /**
     * 渲染下载量柱状图
     * @async
     * @param {string} api_url - API接口地址
     * @param {string} ctx_id - Canvas元素ID
     * @param {number} [y_axis_ratio=0.999] - Y轴最小值比例
     */
    async function render_top_download_chart(
        api_url,
        ctx_id,
        y_axis_ratio = 0.999,
    ) {
        try {
            const response = await fetch(api_url);
            const data = await response.json();

            let apps = [];
            if (data.success && data.data && Array.isArray(data.data)) {
                apps = data.data.map((item) => ({
                    name: item[0].name,
                    download_count: item[1].download_count,
                    icon_url: item[0].icon_url,
                    app_id: item[0].app_id,
                    pkg_name: item[0].pkg_name,
                }));
            }

            if (apps.length === 0) {
                console.error("图表数据无效或为空:", data);
                return;
            }

            // 预加载图标图像，避免悬停时异步加载问题
            const preloadedImages = await preloadImages(apps);

            const minValue = Math.min(...apps.map((item) => item.download_count || 0));
            const yAxisMin = Math.floor(minValue * y_axis_ratio);

            createChart(ctx_id, apps, preloadedImages, yAxisMin);
        } catch (error) {
            console.error("加载下载量图表失败:", error);
        }
    }

    /**
     * 预加载应用图标图像
     * @async
     * @param {Array} apps - 应用数据数组
     * @returns {Promise<Array>} 预加载的图像数组
     */
    async function preloadImages(apps) {
        const preloadedImages = [];
        const loadPromises = apps.map((app, index) => {
            if (!app || !app.icon_url) {
                preloadedImages[index] = null;
                return Promise.resolve();
            }
            return new Promise((resolve) => {
                const img = new Image();
                img.crossOrigin = "anonymous";
                img.onload = () => {
                    preloadedImages[index] = img;
                    resolve();
                };
                img.onerror = () => {
                    preloadedImages[index] = null;
                    resolve();
                };
                img.src = app.icon_url;
            });
        });
        await Promise.all(loadPromises);
        return preloadedImages;
    }

    /**
     * 创建Chart.js图表实例
     * @param {string} ctx_id - Canvas元素ID
     * @param {Array} apps - 应用数据数组
     * @param {Array} preloadedImages - 预加载的图像数组
     * @param {number} yAxisMin - Y轴最小值
     */
    function createChart(ctx_id, apps, preloadedImages, yAxisMin) {
        const ctx = document.getElementById(ctx_id).getContext("2d");
        if (window[ctx_id + "_chart"]) {
            window[ctx_id + "_chart"].destroy();
        }

        // 自定义插件：在柱状图上绘制图标
        const iconPlugin = {
            id: "iconPlugin",
            afterDatasetsDraw(chart) {
                const { ctx } = chart;
                const meta = chart.getDatasetMeta(0);
                meta.data.forEach((bar, index) => {
                    const img = preloadedImages[index];
                    if (!img) return;

                    const x = bar.x;
                    const y = bar.y - 17;

                    ctx.drawImage(img, x - 10, y - 20, 20, 20);
                });
            },
        };

        // 点击事件处理插件
        const clickPlugin = {
            id: "clickPlugin",
            afterEvent(chart, args) {
                const { event } = args;
                if (event.type === 'click') {
                    const elements = chart.getElementsAtEventForMode(
                        event,
                        'nearest',
                        { intersect: true },
                        false
                    );
                    
                    if (elements.length > 0) {
                        const elementIndex = elements[0].index;
                        const app = apps[elementIndex];
                        if (app && app.app_id) {
                            // 打开应用详情页面
                            if (typeof DashboardAppDetails !== 'undefined' && typeof DashboardAppDetails.showAppDetail === 'function') {
                                DashboardAppDetails.showAppDetail(app.app_id);
                            } else {
                                console.warn('应用详情模块未加载或showAppDetail函数不存在');
                            }
                        }
                    }
                }
            }
        };

        window[ctx_id + "_chart"] = new Chart(ctx, {
            type: "bar",
            data: {
                labels: apps.map((item) =>
                    item.name
                        ? item.name.length > 10
                            ? item.name.slice(0, 10) + "..."
                            : item.name
                        : "未知",
                ),
                datasets: [
                    {
                        label: "下载量",
                        data: apps.map((item) => item.download_count || 0),
                        backgroundColor: "rgba(59, 130, 246, 0.6)",
                        borderColor: "rgba(59, 130, 246, 1)",
                        borderWidth: 1,
                    },
                ],
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                scales: {
                    y: {
                        min: yAxisMin,
                        ticks: {
                            callback: function (value) {
                                return DashboardUtils.formatNumber(value);
                            },
                        },
                    },
                },
                plugins: {
                    tooltip: {
                        callbacks: {
                            label: function (context) {
                                return `下载量: ${DashboardUtils.formatNumber(context.raw)}`;
                            },
                        },
                    },
                    datalabels: {
                        anchor: "end",
                        align: "end",
                        offset: -3,
                        color: "#333",
                        font: { family: "console", size: 12 },
                        formatter: function (value) {
                            return DashboardUtils.formatNumber(value);
                        },
                    },
                },

            },
            plugins: [ChartDataLabels, iconPlugin, clickPlugin],
        });
    }

    /**
     * 加载星级分布饼图
     * @async
     */
    async function loadStarChart() {
        try {
            const response = await fetch(`${API_BASE}/charts/star-distribution`);
            const data = await response.json();
            const starData = data.data || data;

            const ctx = document.getElementById("starChart").getContext("2d");
            if (starChart) starChart.destroy();

            const starValues = [
                starData.star_1 || 0,
                starData.star_2 || 0,
                starData.star_3 || 0,
                starData.star_4 || 0,
                starData.star_5 || 0,
            ];

            // 创建带数字的标签
            const labels = [
                `[0-1)星 (${starValues[0]})`,
                `[1-2)星 (${starValues[1]})`,
                `[2-3)星 (${starValues[2]})`,
                `[3-4)星 (${starValues[3]})`,
                `[4-5]星 (${starValues[4]})`,
            ];

            starChart = new Chart(ctx, {
                type: "pie",
                data: {
                    labels: labels,
                    datasets: [
                        {
                            data: starValues,
                            backgroundColor: [
                                "#ef4444",
                                "#f97316",
                                "#eab308",
                                "#22c55e",
                                "#0ea5e9",
                            ],
                        },
                    ],
                },
                options: {
                    responsive: true,
                    maintainAspectRatio: false,
                    plugins: {
                        legend: {
                            position: "bottom",
                        },
                    },
                },
            });
        } catch (error) {
            console.error("加载星级分布图表失败:", error);
        }
    }

    /**
     * 加载所有图表
     * @async
     */
    async function loadCharts() {
        render_top_download_chart(
            `${API_BASE}/rankings/top-downloads?limit=20`,
            "top_download_chart",
            0.999,
        );
        render_top_download_chart(
            `${API_BASE}/rankings/top-downloads?limit=30&exclude_pattern=huawei`,
            "top_download_chart_not_huawei",
            0.9,
        );
        loadStarChart();
    }

    return {
        render_top_download_chart: render_top_download_chart,
        preloadImages: preloadImages,
        createChart: createChart,
        loadStarChart: loadStarChart,
        loadCharts: loadCharts
    };
})();