// 图表渲染工具模块

import type { StarDistribution, RankingApp } from './types';
import { formatNumber } from './formatters';
import { CHART_COLORS } from './constants';

// Chart.js 类型声明
declare global {
  interface Window {
    Chart: any;
    ChartDataLabels: any;
  }
}


const chart_ids = ['top_down_chart', 'top_down_no_hw_chart', 'rating_chart', 'target_api_chart', 'min_api_chart', 'target_device_chart'];

/**
 * 预加载应用图标图像
 * @param apps - 应用数据数组
 * @returns Promise<Array> 预加载的图像数组
 */
export async function preloadImages(apps: RankingApp[]): Promise<(HTMLImageElement | null)[]> {
  const preloadedImages: (HTMLImageElement | null)[] = [];
  const loadPromises = apps.map((app, index) => {
    if (!app || !app.icon_url) {
      preloadedImages[index] = null;
      return Promise.resolve();
    }

    return new Promise<void>((resolve) => {
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
      img.src = app.icon_url!;
    });
  });

  await Promise.all(loadPromises);
  return preloadedImages;
}

/**
 * 创建Chart.js图表实例
 * @param ctxId - Canvas元素ID
 * @param apps - 应用数据数组
 * @param preloadedImages - 预加载的图像数组
 * @param yAxisMin - Y轴最小值
 */
function createChart(
  ctxId: string,
  apps: RankingApp[],
  preloadedImages: (HTMLImageElement | null)[],
  yAxisMin: number
): void {
  const canvas = document.getElementById(ctxId) as HTMLCanvasElement;
  if (!canvas) return;

  const ctx = canvas.getContext("2d");
  if (!ctx) return;

  // 自定义插件：在柱状图上绘制图标
  const iconPlugin = {
    id: "iconPlugin",
    afterDatasetsDraw(chart: any) {
      const { ctx: chartCtx } = chart;
      const meta = chart.getDatasetMeta(0);
      meta.data.forEach((bar: any, index: number) => {
        const img = preloadedImages[index];
        if (!img) return;

        const x = bar.x;
        const y = bar.y - 17;

        chartCtx.drawImage(img, x - 10, y - 20, 20, 20);
      });
    },
  };

  // 点击事件处理插件
  const clickPlugin = {
    id: "clickPlugin",
    afterEvent(chart: any, args: any) {
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
            if (typeof window.showAppDetail === 'function') {
              window.showAppDetail(app.app_id);
            } else {
              console.warn('应用详情显示函数未定义');
            }

            // 更新URL参数
            if (typeof window.updateUrlParam === 'function') {
              window.updateUrlParam('app_id', app.app_id);
            }
          }
        }
      }
    }
  };

  // 截断长标签
  const labels = apps.map((item) =>
    item.name
      ? item.name.length > 10
        ? item.name.slice(0, 10) + "..."
        : item.name
      : "未知"
  );

  window[chartKey] = new window.Chart(ctx, {
    type: "bar",
    data: {
      labels,
      datasets: [{
        label: "下载量",
        data: apps.map((item) => item.download_count || 0),
        backgroundColor: CHART_COLORS.DOWNLOAD_BAR,
        borderColor: CHART_COLORS.DOWNLOAD_BORDER,
        borderWidth: 1,
      }],
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      scales: {
        y: {
          min: yAxisMin,
          ticks: {
            callback: function (value: number) {
              return formatNumber(value);
            },
          },
        },
      },
      plugins: {
        tooltip: {
          callbacks: {
            label: function (context: any) {
              return `下载量: ${formatNumber(context.raw)}`;
            },
          },
        },
        datalabels: {
          anchor: "end",
          align: "end",
          offset: -3,
          color: "#333",
          font: { family: "console", size: 12 },
          formatter: function (value: number) {
            return formatNumber(value);
          },
        },
      },
    },
    plugins: [window.ChartDataLabels, iconPlugin, clickPlugin],
  });
}

/**
 * 渲染下载量柱状图
 * @param apiUrl - API接口地址
 * @param ctxId - Canvas元素ID
 * @param yAxisRatio - Y轴最小值比例
 */
export async function renderDownloadChart(
  apiUrl: string,
  ctxId: string,
  yAxisRatio: number = 0.999
): Promise<void> {
  try {
    const response = await fetch(apiUrl);
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data = await response.json();
    let apps: RankingApp[] = [];

    if (data.success && data.data && Array.isArray(data.data)) {
      apps = data.data.map((item: [any, any]) => ({
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
    const yAxisMin = Math.floor(minValue * yAxisRatio);

    createChart(ctxId, apps, preloadedImages, yAxisMin);
  } catch (error) {
    console.error("加载下载量图表失败:", error);
  }
}

/**
 * 加载星级分布饼图
 */
export async function render_rating_chart(): Promise<void> {
  try {
    const response = await fetch('/api/charts/star-distribution');
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data = await response.json();
    const starData = data.data || data;

    const canvas = document.getElementById("starChart") as HTMLCanvasElement;
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    // 销毁现有图表
    if (window.starChart) {
      window.starChart.destroy();
    }

    const starValues = [
      starData.star_1 || 0,
      starData.star_2 || 0,
      starData.star_3 || 0,
      starData.star_4 || 0,
      starData.star_5 || 0,
    ];

    // 创建带数字的标签
    const labels = [
      `无评分 (${starValues[0]})`,
      `[1-2)星 (${starValues[1]})`,
      `[2-3)星 (${starValues[2]})`,
      `[3-4)星 (${starValues[3]})`,
      `[4-5]星 (${starValues[4]})`,
    ];

    window.starChart = new window.Chart(ctx, {
      type: "pie",
      data: {
        labels,
        datasets: [{
          data: starValues,
          backgroundColor: CHART_COLORS.STAR_RATING,
        }],
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: {
            position: "bottom",
          },
          tooltip: {
            callbacks: {
              label: function (context: any) {
                const label = context.label || '';
                const value = context.raw || 0;
                const total = context.dataset.data.reduce((sum: number, val: number) => sum + val, 0);
                const percentage = ((value / total) * 100).toFixed(1);
                return `${label}: ${formatNumber(value)} (${percentage}%)`;
              },
            },
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
 */
export async function loadAllCharts(): Promise<void> {
  await Promise.all([
    renderDownloadChart('/api/rankings/top-downloads?limit=20', 'top_down_chart', 0.9),
    renderDownloadChart('/api/rankings/top-downloads?limit=30&exclude_pattern=huawei', 'top_down_no_hw_chart', 0.9),
    render_rating_chart(),
  ]);
}

/**
 * 销毁所有图表实例
 */
export function destroyAllCharts(): void {

  chart_ids.forEach(id => {
    const chartKey = `${id}_chart`;
    if (window[chartKey]) {
      window[chartKey].destroy();
      delete window[chartKey];
    }
  });

  if (window.starChart) {
    window.starChart.destroy();
    delete window.starChart;
  }
}

/**
 * 更新图表主题
 * @param isDark - 是否为深色主题
 */
export function updateChartTheme(isDark: boolean): void {
  const textColor = isDark ? '#e5e7eb' : '#374151';
  const gridColor = isDark ? '#374151' : '#e5e7eb';

  // 更新所有图表的文字和网格颜色
  Object.keys(window).forEach(key => {
    if (key.endsWith('_chart') && window[key] && window[key].options) {
      const chart = window[key];

      // 更新坐标轴颜色
      if (chart.options.scales) {
        Object.values(chart.options.scales).forEach((scale: any) => {
          if (scale.ticks) {
            scale.ticks.color = textColor;
          }
          if (scale.grid) {
            scale.grid.color = gridColor;
          }
        });
      }

      // 更新图例颜色
      if (chart.options.plugins && chart.options.plugins.legend) {
        chart.options.plugins.legend.labels = {
          ...chart.options.plugins.legend.labels,
          color: textColor,
        };
      }

      chart.update();
    }
  });
}

// 全局图表实例声明
declare global {
  interface Window {
    showAppDetail?: (appId: string) => void;
    updateUrlParam?: (key: string, value: string) => void;
    starChart?: any;
  }
}