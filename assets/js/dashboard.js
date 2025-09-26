// Global variables
let currentPage = 1;
let totalPages = 1;
let currentSort = { field: "download_count", direction: "desc" };
let searchTerm = "";
let starChart = null;
let categoryFilter = "";
let top_download_chart = null;
const PAGE_SIZE = 20;
const API_BASE = "/api"; // Adjust if needed

// Format number
function formatNumber(num) {
  return num.toString().replace(/\B(?=(\d{4})+(?!\d))/g, ",");
}

// Format file size
function formatSize(size) {
  if (size < 1024) return size + " B";
  if (size < 1024 * 1024) return (size / 1024).toFixed(2) + " KB";
  if (size < 1024 * 1024 * 1024)
    return (size / (1024 * 1024)).toFixed(2) + " MB";
  return (size / (1024 * 1024 * 1024)).toFixed(2) + " GB";
}

// Render stars
function renderStars(rating) {
  if (!rating) return "N/A";
  const fullStars = Math.floor(rating);
  const halfStar = rating % 1 >= 0.5;
  let html = "";
  for (let i = 0; i < fullStars; i++) {
    html += '<i class="fas fa-star text-warning"></i>';
  }
  if (halfStar) html += '<i class="fas fa-star-half-alt text-warning"></i>';
  return html + ` ${rating.toFixed(1)}`;
}

// Load overview data
async function loadOverview() {
  try {
    // 显示所有加载指示器
    const loadingOverview = document.getElementById("loadingOverview");
    const loadingDeveloperCount = document.getElementById(
      "loadingDeveloperCount",
    );
    const loadingAtomicServiceCount = document.getElementById(
      "loadingAtomicServiceCount",
    );

    if (loadingOverview) loadingOverview.style.display = "block";
    if (loadingDeveloperCount) loadingDeveloperCount.style.display = "block";
    if (loadingAtomicServiceCount)
      loadingAtomicServiceCount.style.display = "block";

    // Get app count
    const appResponse = await fetch(`${API_BASE}/apps/list/info`);
    const appData = await appResponse.json();

    document.getElementById("appCount").textContent = formatNumber(
      appData.data.app_count || 0,
    );
    document.getElementById("atomicServiceCount").textContent = formatNumber(
      appData.data.atomic_services_count || 0,
    );

    // Get developer count
    const developerResponse = await fetch(`${API_BASE}/stats/developers/count`);
    const developerData = await developerResponse.json();

    document.getElementById("developerCount").textContent = formatNumber(
      developerData.data.developer_count || 0,
    );

    // 隐藏所有加载指示器
    if (loadingOverview) loadingOverview.style.display = "none";
    if (loadingDeveloperCount) loadingDeveloperCount.style.display = "none";
    if (loadingAtomicServiceCount)
      loadingAtomicServiceCount.style.display = "none";
  } catch (error) {
    console.error("Failed to load overview:", error);
    // 错误时隐藏所有加载指示器
    const loadingOverview = document.getElementById("loadingOverview");
    const loadingDeveloperCount = document.getElementById(
      "loadingDeveloperCount",
    );
    const loadingAtomicServiceCount = document.getElementById(
      "loadingAtomicServiceCount",
    );

    if (loadingOverview) loadingOverview.style.display = "none";
    if (loadingDeveloperCount) loadingDeveloperCount.style.display = "none";
    if (loadingAtomicServiceCount)
      loadingAtomicServiceCount.style.display = "none";
  }
}

// Load apps with pagination and sorting
async function loadApps(
  page = 1,
  sortField = currentSort.field,
  sortDirection = currentSort.direction,
  search = searchTerm,
  category = categoryFilter,
) {
  try {
    document.getElementById("appTableBody").innerHTML =
      '<tr><td colspan="8" class="text-center"><div class="spinner-border" role="status"><span class="sr-only">Loading...</span></div></td></tr>';

    let url = `${API_BASE}/apps/list/${page}`;
    if (search) {
      // In a real implementation, you would have a search endpoint
      console.log("Search term:", search);
    }

    const response = await fetch(url);
    const data = await response.json();

    if (data.data && data.data.total_count) {
      totalPages = Math.ceil(data.data.total_count / PAGE_SIZE);
      currentPage = page;
    }

    // Sort and filter the data
    let apps = data.data.apps || [];
    if (category && category !== "all") {
      apps = apps.filter((app) => app.category === category);
    }

    // Sort data
    apps.sort((a, b) => {
      let valueA = a[sortField] || 0;
      let valueB = b[sortField] || 0;

      if (typeof valueA === "string" && typeof valueB === "string") {
        return sortDirection === "asc"
          ? valueA.localeCompare(valueB)
          : valueB.localeCompare(valueA);
      }

      return sortDirection === "asc" ? valueA - valueB : valueB - valueA;
    });

    renderApps(apps);
    renderPagination();
  } catch (error) {
    console.error("Failed to load apps:", error);
    document.getElementById("appTableBody").innerHTML =
      '<tr><td colspan="8" class="text-center">Error loading data</td></tr>';
  }
}

// Render pagination controls
function renderPagination() {
  const paginationEl = document.getElementById("pagination");
  paginationEl.innerHTML = "";

  if (totalPages <= 1) return;

  const ul = document.createElement("ul");
  ul.className = "pagination";

  // Previous button
  const prevLi = document.createElement("li");
  prevLi.className = `page-item ${currentPage === 1 ? "disabled" : ""}`;
  const prevLink = document.createElement("a");
  prevLink.className = "page-link";
  prevLink.href = "#";
  prevLink.textContent = "上一页";
  if (currentPage > 1) {
    prevLink.addEventListener("click", (e) => {
      e.preventDefault();
      loadApps(currentPage - 1);
    });
  }
  prevLi.appendChild(prevLink);
  ul.appendChild(prevLi);

  // Generate page numbers
  const startPage = Math.max(1, currentPage - 2);
  const endPage = Math.min(totalPages, currentPage + 2);

  for (let i = startPage; i <= endPage; i++) {
    const li = document.createElement("li");
    li.className = `page-item ${i === currentPage ? "active" : ""}`;
    const link = document.createElement("a");
    link.className = "page-link";
    link.href = "#";
    link.textContent = i;
    link.addEventListener("click", (e) => {
      e.preventDefault();
      loadApps(i);
    });
    li.appendChild(link);
    ul.appendChild(li);
  }

  // Next button
  const nextLi = document.createElement("li");
  nextLi.className = `page-item ${currentPage === totalPages ? "disabled" : ""}`;
  const nextLink = document.createElement("a");
  nextLink.className = "page-link";
  nextLink.href = "#";
  nextLink.textContent = "下一页";
  if (currentPage < totalPages) {
    nextLink.addEventListener("click", (e) => {
      e.preventDefault();
      loadApps(currentPage + 1);
    });
  }
  nextLi.appendChild(nextLink);
  ul.appendChild(nextLi);

  paginationEl.appendChild(ul);
}

// Load app categories
async function loadCategories() {
  try {
    const categorySelect = document.getElementById("categoryFilter");

    // You might want to create an API endpoint for categories
    // For now, we'll hardcode some common categories
    const categories = [
      "All",
      "Games",
      "Social",
      "Productivity",
      "Entertainment",
      "Education",
      "Lifestyle",
      "Utilities",
    ];

    // Add all option first
    const allOption = document.createElement("option");
    allOption.value = "all";
    allOption.textContent = "All Categories";
    categorySelect.appendChild(allOption);

    // Add other categories
    categories.forEach((category) => {
      if (category === "All") return; // Skip "All" as we already added it
      const option = document.createElement("option");
      option.value = category.toLowerCase();
      option.textContent = category;
      categorySelect.appendChild(option);
    });
  } catch (error) {
    console.error("Failed to load categories:", error);
  }
}

async function renderTopDownloadChart(apiUrl, ctxId, yAxisRatio = 0.999) {
  try {
    const response = await fetch(apiUrl);
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

    const minValue = Math.min(...apps.map((item) => item.download_count || 0));
    const yAxisMin = Math.floor(minValue * yAxisRatio);

    const ctx = document.getElementById(ctxId).getContext("2d");
    if (window[ctxId + "_chart"]) {
      window[ctxId + "_chart"].destroy();
    }

    // 自定义插件：在柱子上方绘制图标
    const iconPlugin = {
      id: "iconPlugin",
      afterDatasetsDraw(chart) {
        const { ctx } = chart;
        chart.getDatasetMeta(0).data.forEach((bar, index) => {
          const app = apps[index];
          if (!app || !app.icon_url) return;

          const x = bar.x;
          const y = bar.y - 30;

          const img = new Image();
          img.src = app.icon_url;
          img.onload = () => {
            ctx.drawImage(img, x - 10, y - 20, 20, 20);
          };
        });
      },
    };

    window[ctxId + "_chart"] = new Chart(ctx, {
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
            backgroundColor: "rgba(54, 162, 235, 0.6)",
            borderColor: "rgba(54, 162, 235, 1)",
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
            color: "#333",
            font: { weight: "bold" },
            formatter: function (value) {
              return formatNumber(value);
            },
          },
        },
      },
      plugins: [ChartDataLabels, iconPlugin],
    });
  } catch (error) {
    console.error("Failed to load top download chart:", error);
  }
}


async function loadCharts() {
  renderTopDownloadChart(
    `${API_BASE}/rankings/top-downloads?limit=20`,
    "top_download_chart",
    0.999
  );
  renderTopDownloadChart(
    `${API_BASE}/rankings/top-downloads?limit=30&exclude_pattern=huawei`,
    "top_download_chart_not_huawei",
    0.9
  );


  // Star Distribution Chart
  try {
    const response = await fetch(`${API_BASE}/charts/star-distribution`);
    const data = await response.json();

    const starData = data.data || data;

    const ctx2 = document.getElementById("starChart").getContext("2d");
    if (starChart) starChart.destroy();

    const starValues = [
      starData.star_1 || 0,
      starData.star_2 || 0,
      starData.star_3 || 0,
      starData.star_4 || 0,
      starData.star_5 || 0,
    ];

    starChart = new Chart(ctx2, {
      type: "pie",
      data: {
        labels: ["1星", "2星", "3星", "4星", "5星"],
        datasets: [
          {
            data: starValues,
            backgroundColor: [
              "#dc3545",
              "#fd7e14",
              "#ffc107",
              "#28a745",
              "#17a2b8",
            ],
          },
        ],
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          tooltip: {
            callbacks: {
              label: function (context) {
                const label = context.label || "";
                const value = context.parsed || 0;
                return `${label}: ${value} 个`;
              },
            },
          },
          legend: {
            labels: {
              generateLabels: function (chart) {
                const data = chart.data;
                if (data.labels.length && data.datasets.length) {
                  return data.labels.map((label, i) => {
                    const value = data.datasets[0].data[i];
                    return {
                      text: `${label} (${value} 个)`,
                      fillStyle: data.datasets[0].backgroundColor[i],
                      strokeStyle: data.datasets[0].backgroundColor[i],
                      lineWidth: 1,
                      hidden: false,
                      index: i,
                    };
                  });
                }
                return [];
              },
            },
          },
        },
      },
    });
  } catch (error) {
    console.error("Failed to load star distribution chart:", error);
  }
}

// Show app details in modal
async function showAppDetail(appId) {
  try {
    const modal = document.getElementById("appDetailModal");
    const modalBody = document.getElementById("appDetailContent");
    modalBody.innerHTML =
      '<div class="text-center"><div class="spinner-border" role="status"><span class="sr-only">Loading...</span></div></div>';

    const response = await fetch(`${API_BASE}/apps/id/${appId}`);
    const data = await response.json();
    const app = data.data.info || data.data;

    let html = `
            <div class="row">
                <div class="col-md-3 text-center">
                    <img src="${app.icon || "/img/default-app-icon.png"}" class="app-icon img-fluid rounded mb-3" alt="${app.name}">
                    <p class="mb-1">${renderStars(app.rating)}</p>
                    <p class="text-muted">${app.rating_count || 0} 评分</p>
                </div>
                <div class="col-md-9">
                    <h4>${app.name || "Unknown App"}</h4>
                    <p class="text-muted">${app.developer || "Unknown Developer"}</p>
                    <div class="mb-3">
                        <span class="badge badge-primary mr-2">${app.category || "Uncategorized"}</span>
                        <span class="badge badge-secondary mr-2">${formatSize(app.size || 0)}</span>
                        <span class="badge badge-info">${app.version || "Unknown Version"}</span>
                    </div>
                    <p><strong>下载量:</strong> ${formatNumber(app.download_count || 0)}</p>
                    <p><strong>价格:</strong> ${app.price ? `¥${app.price.toFixed(2)}` : "免费"}</p>
                    <p><strong>上次更新:</strong> ${app.last_update ? new Date(app.last_update).toLocaleDateString() : "未知"}</p>
                    <hr>
                    <p>${app.description || "无描述"}</p>
                </div>
            </div>
        `;

    modalBody.innerHTML = html;

    // Initialize and show the modal using Bootstrap's methods
    const modalInstance = new bootstrap.Modal(modal);
    modalInstance.show();
  } catch (error) {
    console.error("Failed to load app details:", error);
  }
}

// Render the apps in the table
function renderApps(apps) {
  const tableBody = document.getElementById("appTableBody");
  tableBody.innerHTML = "";

  if (!apps || apps.length === 0) {
    tableBody.innerHTML =
      '<tr><td colspan="8" class="text-center">No apps found</td></tr>';
    return;
  }

  apps.forEach((app) => {
    const tr = document.createElement("tr");
    tr.style.cursor = "pointer";
    tr.onclick = () => showAppDetail(app.app_id || app.id);

    tr.innerHTML = `
            <td>
                <img src="${app.icon || "/img/default-app-icon.png"}" class="app-icon" alt="${app.name}">
                ${app.name || "Unknown"}
            </td>
            <td>${app.developer || "Unknown"}</td>
            <td>${app.category || "Uncategorized"}</td>
            <td>${renderStars(app.rating)}</td>
            <td>${formatNumber(app.download_count || 0)}</td>
            <td>${app.price ? `¥${app.price.toFixed(2)}` : "免费"}</td>
            <td>${formatSize(app.size || 0)}</td>
            <td>${app.last_update ? new Date(app.last_update).toLocaleDateString() : "Unknown"}</td>
        `;

    tableBody.appendChild(tr);
  });
}

// Update sort icons in table headers
function updateSortIcons() {
  const headers = document.querySelectorAll("th[data-sort]");
  headers.forEach((header) => {
    const field = header.getAttribute("data-sort");
    const icon = header.querySelector("i");
    if (field === currentSort.field) {
      icon.className =
        currentSort.direction === "asc"
          ? "fas fa-sort-up ml-1"
          : "fas fa-sort-down ml-1";
    } else {
      icon.className = "fas fa-sort ml-1 text-muted";
    }
  });
}

// Update last update timestamp
function updateLastUpdate() {
  const now = new Date();
  document.getElementById("lastUpdate").textContent = now.toLocaleString();
}

// Event listeners to be attached when the document is loaded
document.addEventListener("DOMContentLoaded", () => {
  // Initialize the dashboard
  loadOverview();
  loadApps();
  loadCategories();
  loadCharts();
  updateLastUpdate();

  // Set up event listeners for sorting
  document.querySelectorAll("th[data-sort]").forEach((header) => {
    header.addEventListener("click", () => {
      const field = header.getAttribute("data-sort");
      let direction = "desc";

      if (field === currentSort.field) {
        direction = currentSort.direction === "desc" ? "asc" : "desc";
      }

      currentSort = { field, direction };
      updateSortIcons();
      loadApps(1);
    });
  });

  // Search button event listener
  document.getElementById("searchBtn").addEventListener("click", () => {
    searchTerm = document.getElementById("searchInput").value.trim();
    loadApps(1);
  });

  // Search input enter key event
  document.getElementById("searchInput").addEventListener("keypress", (e) => {
    if (e.key === "Enter") {
      searchTerm = e.target.value.trim();
      loadApps(1);
    }
  });

  // Category filter event
  document.getElementById("categoryFilter").addEventListener("change", (e) => {
    categoryFilter = e.target.value;
    loadApps(1);
  });

  // Set up initial sort icons
  updateSortIcons();

  // Refresh button event
  document.getElementById("refreshBtn").addEventListener("click", () => {
    loadOverview();
    loadApps(currentPage);
    loadCharts();
    updateLastUpdate();
  });
});
