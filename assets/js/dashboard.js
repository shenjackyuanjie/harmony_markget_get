// Global variables
// 全局变量：用于管理页面状态和图表实例
let currentPage = 1;  // 当前页码
let totalPages = 1;   // 总页数
let currentSort = { field: "download_count", desc: true };  // 当前排序字段和方向
let searchTerm = "";  // 搜索关键词
let categoryFilter = "all";  // 分类过滤器
let starChart = null;  // 星级分布图表实例
const PAGE_SIZE = 20;  // 每页显示的应用数量
const API_BASE = "/api";  // API 基础路径，根据需要调整

// 格式化数字，四位一分隔
function formatNumber(num) {
  return num.toString().replace(/\B(?=(\d{4})+(?!\d))/g, ",");
}

// 格式化文件大小
function formatSize(size) {
  if (size < 1024) return size + " B";
  if (size < 1024 * 1024) return (size / 1024).toFixed(2) + " KB";
  if (size < 1024 * 1024 * 1024)
    return (size / (1024 * 1024)).toFixed(2) + " MB";
  return (size / (1024 * 1024 * 1024)).toFixed(2) + " GB";
}

// 使用 Unicode 渲染星级评分
function renderStars(rating) {
  if (!rating) return null;
  const fullStars = Math.floor(rating);
  const hasHalf = rating % 1 >= 0.5;
  let stars = "";
  for (let i = 0; i < 5; i++) {
    if (i < fullStars) {
      stars += "★";  // 满星
    } else if (i === fullStars && hasHalf) {
      stars += "☆";  // 半星（简化表示，可替换为更好符号）
    } else {
      stars += "☆";  // 空星
    }
  }
  return stars + ` ${rating.toFixed(1)}`;
}

// 加载概览数据
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

    document.getElementById("totalCount").textContent = formatNumber(
      (market_info.data.app_count || 0) + (market_info.data.atomic_services_count || 0),
    );
    document.getElementById("appCount").textContent = formatNumber(
      market_info.data.app_count || 0,
    );
    document.getElementById("atomicServiceCount").textContent = formatNumber(
      market_info.data.atomic_services_count || 0,
    );
    document.getElementById("developerCount").textContent = formatNumber(
      market_info.data.developer_count || 0,
    );

    // 隐藏加载指示器
    loadingElements.forEach((id) => {
      const el = document.getElementById(id);
      if (el) el.style.display = "none";
    });
  } catch (error) {
    console.error("Failed to load overview:", error);
    // 错误时隐藏加载指示器
    loadingElements.forEach((id) => {
      const el = document.getElementById(id);
      if (el) el.style.display = "none";
    });
  }
}

// Load apps with pagination, sorting, search, and category filter
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

    let url = `${API_BASE}/apps/list/${page}?sort=${sortField}&desc=${sort_desc}&page_size=${PAGE_SIZE}`;
    if (search) url += `&search=${encodeURIComponent(search)}`;
    if (category && category !== "all")
      url += `&category=${encodeURIComponent(category)}`;

    const response = await fetch(url);
    const data = await response.json();

    if (data.data && data.data.total_count) {
      totalPages = Math.ceil(data.data.total_count / PAGE_SIZE);
      currentPage = page;
    }

    let apps = data.data.data || [];
    // Additional client-side filtering if needed
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

    renderApps(apps.slice(0, PAGE_SIZE)); // Paginate client-side if needed
    renderPagination();
  } catch (error) {
    console.error("Failed to load apps:", error);
    document.getElementById("appTableBody").innerHTML =
      '<tr><td colspan="8" class="text-center py-4 text-gray-500">Error loading data</td></tr>';
  }
}

// Render apps in the table
function renderApps(apps) {
  const tableBody = document.getElementById("appTableBody");
  tableBody.innerHTML = "";

  if (!apps || apps.length === 0) {
    tableBody.innerHTML =
      '<tr><td colspan="8" class="text-center py-4 text-gray-500">No apps found</td></tr>';
    return;
  }
  apps.forEach((app) => {
    const app_info = app.info;
    const app_metric = app.metric;
    const app_rating = app.rating || {};

    const tr = document.createElement("tr");
    tr.className = "hover:bg-gray-50 cursor-pointer transition-colors";
    tr.onclick = () => showAppDetail(app_info.app_id);
    tr.innerHTML = `
      <td class="px-6 py-4 whitespace-nowrap">
        <div class="flex items-center">
          <img src="${app_info.icon_url || "/img/default-app-icon.png"}" class="app-icon mr-3" alt="${app_info.name}">
          <span class="font-medium text-gray-900">${app_info.name || "Unknown"}</span>
        </div>
      </td>
      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">${app_info.developer_name || "Unknown"}</td>
      <td class="px-6 py-4 whitespace-nowrap">
        <span class="inline-flex items-center px-2 py-1 text-xs font-medium rounded-full bg-blue-100 text-blue-800">${app_info.kind_type_name || "未知"}-${app_info.kind_name || "未知"}</span>
      </td>
      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">${renderStars(app_rating.average_rating)}</td>
      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">${formatNumber(app_metric.download_count || 0)}</td>
      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">${formatSize(app_metric.size_bytes || 0)}</td>
      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">${app_metric.created_at ? new Date(app_metric.created_at).toLocaleDateString("zh-CN") : "Unknown"}</td>
    `;

    tableBody.appendChild(tr);
  });
}

// Update sort icons in table headers
function updateSortIcons() {
  const headers = document.querySelectorAll("th[data-sort]");
  headers.forEach((header) => {
    const field = header.getAttribute("data-sort");
    const span = header.querySelector("span");
    if (!span) return;

    if (field === currentSort.field) {
      span.textContent = currentSort.desc === false ? "↑" : "↓";
      header.classList.add("bg-gray-100"); // Active state
    } else {
      span.textContent = "↑";
      header.classList.remove("bg-gray-100");
    }
  });
}

// Render pagination controls with Tailwind
function renderPagination() {
  const paginationEl = document.getElementById("pagination");
  paginationEl.innerHTML = "";

  if (totalPages <= 1) return;

  const ul = document.createElement("ul");
  ul.className = "flex items-center space-x-1";

  // First page button
  const firstLi = document.createElement("li");
  firstLi.className = `flex ${currentPage === 1 ? "opacity-50 cursor-not-allowed" : "cursor-pointer"}`;
  const firstA = document.createElement("a");
  firstA.className =
    "px-3 py-2 text-sm font-medium rounded-md border border-gray-300 bg-white text-gray-500 hover:bg-gray-50";
  firstA.textContent = "第一页";
  if (currentPage > 1) {
    firstA.onclick = (e) => {
      e.preventDefault();
      loadApps(1);
    };
  }
  firstLi.appendChild(firstA);
  ul.appendChild(firstLi);

  // Previous button
  const prevLi = document.createElement("li");
  prevLi.className = `flex ${currentPage === 1 ? "opacity-50 cursor-not-allowed" : "cursor-pointer"}`;
  const prevA = document.createElement("a");
  prevA.className =
    "px-3 py-2 text-sm font-medium rounded-md border border-gray-300 bg-white text-gray-500 hover:bg-gray-50";
  prevA.textContent = "上一页";
  if (currentPage > 1) {
    prevA.onclick = (e) => {
      e.preventDefault();
      loadApps(currentPage - 1);
    };
  }
  prevLi.appendChild(prevA);
  ul.appendChild(prevLi);

  // Page numbers
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
      loadApps(i);
    };
    li.appendChild(a);
    ul.appendChild(li);
  }

  // Next button
  const nextLi = document.createElement("li");
  nextLi.className = `flex ${currentPage === totalPages ? "opacity-50 cursor-not-allowed" : "cursor-pointer"}`;
  const nextA = document.createElement("a");
  nextA.className =
    "px-3 py-2 text-sm font-medium rounded-md border border-gray-300 bg-white text-gray-500 hover:bg-gray-50";
  nextA.textContent = "下一页";
  if (currentPage < totalPages) {
    nextA.onclick = (e) => {
      e.preventDefault();
      loadApps(currentPage + 1);
    };
  }
  nextLi.appendChild(nextA);
  ul.appendChild(nextLi);

  // Last page button
  const lastLi = document.createElement("li");
  lastLi.className = `flex ${currentPage === totalPages ? "opacity-50 cursor-not-allowed" : "cursor-pointer"}`;
  const lastA = document.createElement("a");
  lastA.className =
    "px-3 py-2 text-sm font-medium rounded-md border border-gray-300 bg-white text-gray-500 hover:bg-gray-50";
  lastA.textContent = "最后一页";
  if (currentPage < totalPages) {
    lastA.onclick = (e) => {
      e.preventDefault();
      loadApps(totalPages);
    };
  }
  lastLi.appendChild(lastA);
  ul.appendChild(lastLi);

  paginationEl.appendChild(ul);
}

// Load categories
async function loadCategories() {
  try {
    const categorySelect = document.getElementById("categoryFilter");
    categorySelect.innerHTML = '<option value="all">所有分类</option>';

    // Hardcoded or fetch from API
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
    console.error("Failed to load categories:", error);
  }
}

// 渲染下载量柱状图
async function render_top_download_chart(api_url, ctx_id, y_axis_ratio = 0.999) {
  try {
    const response = await fetch(api_url);
    const data = await response.json();

    let apps = [];
    if (data.success && data.data && Array.isArray(data.data)) {
      apps = data.data.map((item) => ({
        name: item[0].name,
        download_count: item[1].download_count,
        icon_url: item[0].icon_url,
      }));
    }

    if (apps.length === 0) {
      console.error("Invalid or empty apps data for chart:", data);
      return;
    }

    // 子函数：预加载图标图像，避免悬停时异步加载问题
    const preloadedImages = await preloadImages(apps);

    const minValue = Math.min(...apps.map((item) => item.download_count || 0));
    const yAxisMin = Math.floor(minValue * y_axis_ratio);

    createChart(ctx_id, apps, preloadedImages, yAxisMin);
  } catch (error) {
    console.error("Failed to load top download chart:", error);
  }
}

// 子函数：预加载图像
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

// 子函数：创建 Chart.js 图表实例
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

  window[ctx_id + "_chart"] = new Chart(ctx, {
    type: "bar",
    data: {
      labels: apps.map((item) =>
        item.name
          ? item.name.length > 10
            ? item.name.slice(0, 10) + "..."
            : item.name
          : "Unknown",
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
              return formatNumber(value);
            },
          },
        },
      },
      plugins: {
        tooltip: {
          callbacks: {
            label: function (context) {
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
          formatter: function (value) {
            return formatNumber(value);
          },
        },
      },
    },
    plugins: [ChartDataLabels, iconPlugin],  // 假设 ChartDataLabels 已引入
  });
}

// 加载星级分布饼图
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
    console.error("Failed to load star distribution chart:", error);
  }
}

// Load all charts
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

// Show app details in modal
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

    let html = `
      <div class="flex flex-col md:flex-row gap-6">
        <div class="md:w-1/4 text-center md:text-left">
          <img src="${app_info.icon_url || "/img/default-app-icon.png"}" class="w-24 h-24 mx-auto md:mx-0 app-icon rounded-lg mb-3" alt="${app_info.name}">
          <p class="mb-1 text-lg">${renderStars(app_rating.average_rating) || "无评分"}</p>
          <p class="text-gray-500">${app_rating.total_star_rating_count || "无"} 评分</p>
        </div>
        <div class="md:w-3/4">
          <h4 class="text-2xl font-bold text-gray-900 mb-2">${app_info.name || "Unknown App"}</h4>
          <p class="text-gray-600 mb-4">${app_info.developer_name || "Unknown Developer"}</p>
          <div class="flex flex-wrap gap-2 mb-4">
            <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800">${app_info.kind_type_name || "未知"}-${app_info.kind_name || "未知"}</span>
            <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-gray-100 text-gray-800">${formatSize(app_metric.size_bytes || 0)}</span>
            <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-indigo-100 text-indigo-800">${app_metric.version || "Unknown Version"}</span>
          </div>
          <div class="space-y-2 mb-4">
            <p><strong class="text-gray-900">下载量:</strong> <span class="text-gray-600">${formatNumber(app_metric.download_count || 0)}</span></p>
            <p><strong class="text-gray-900">价格:</strong> <span class="text-gray-600">${app_metric.price ? `¥${app_metric.price}` : "免费"}</span></p>
            <p><strong class="text-gray-900">上次更新:</strong> <span class="text-gray-600">${app_metric.created_at ? new Date(app_metric.created_at).toLocaleDateString("zh-CN") : "未知"}</span></p>
          </div>
          <hr class="my-4 border-gray-200">
          <p class="text-gray-700">${(app_info.description || "无描述").replace(/\n/g, '<br>')}</p>
        </div>
      </div>
    `;

    modalContent.innerHTML = html;
    modal.classList.remove("hidden");
  } catch (error) {
    console.error("Failed to load app details:", error);
    document.getElementById("appDetailContent").innerHTML =
      '<div class="text-center py-4 text-red-500">Failed to load details</div>';
    document.getElementById("appDetailModal").classList.remove("hidden");
  }
}

// 更新最后更新时间戳
function updateLastUpdate() {
  const now = new Date();
  document.getElementById("lastUpdate").textContent =
    now.toLocaleString("zh-CN");
}

// 刷新所有数据
async function refreshData() {
  updateLastUpdate();
  await loadOverview();
  await loadApps(1);
  await loadCharts();
}

// Event listeners
document.addEventListener("DOMContentLoaded", () => {
  loadOverview();
  loadApps();
  loadCategories();
  loadCharts();
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
      updateSortIcons();
      loadApps(1);
    });
  });

  // Search button
  document.getElementById("searchBtn").addEventListener("click", () => {
    searchTerm = document.getElementById("searchInput").value.trim();
    categoryFilter = document.getElementById("categoryFilter").value;
    loadApps(1);
  });

  // Search input enter key
  document.getElementById("searchInput").addEventListener("keypress", (e) => {
    if (e.key === "Enter") {
      searchTerm = e.target.value.trim();
      categoryFilter = document.getElementById("categoryFilter").value;
      loadApps(1);
    }
  });

  // Category filter change
  document.getElementById("categoryFilter").addEventListener("change", (e) => {
    categoryFilter = e.target.value;
    searchTerm = document.getElementById("searchInput").value.trim();
    loadApps(1);
  });

  // Refresh button
  document.getElementById("refreshBtn").addEventListener("click", refreshData);

  // Modal close (already handled in HTML onclick)
});
