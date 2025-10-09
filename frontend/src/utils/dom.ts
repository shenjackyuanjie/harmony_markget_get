// DOM操作和渲染工具模块

import type { App } from './types';
import { formatNumber, format_size, formatDate, formatLocalDate, formatAppType, formatRating } from './formatters';
import { DEFAULT_APP_ICON, PAGE_SIZE } from './constants';

/**
 * 更新最后更新时间
 */
export function updateLastUpdate(): void {
  const lastUpdateElement = document.getElementById("lastUpdate");
  if (lastUpdateElement) {
    const now = new Date();
    const formattedTime = now.toLocaleString("zh-CN", {
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    });
    lastUpdateElement.textContent = formattedTime;
  }
}

/**
 * 渲染应用列表到表格
 * @param apps - 应用数据数组
 * @param currentPage - 当前页码
 * @param onAppClick - 应用点击回调函数
 */
export function renderAppTable(
  apps: App[], 
  currentPage: number,
  onAppClick?: (appId: string) => void
): void {
  const tableBody = document.getElementById("appTableBody");
  if (!tableBody) return;

  tableBody.innerHTML = "";

  if (!apps || apps.length === 0) {
    tableBody.innerHTML =
      '<tr><td colspan="12" class="text-center py-4 text-gray-500">未找到应用</td></tr>';
    return;
  }

  apps.forEach((app, index) => {
    const { info, metric, rating } = app;
    
    const tr = document.createElement("tr");
    tr.className = "hover:bg-gray-50 cursor-pointer transition-colors";
    
    tr.onclick = () => {
      if (onAppClick) {
        onAppClick(info.app_id);
      }
      // 更新URL参数
      if (typeof window.updateUrlParam === 'function') {
        window.updateUrlParam('app_id', info.app_id);
      }
    };
    
    tr.innerHTML = `
      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
        ${(currentPage - 1) * PAGE_SIZE + index + 1}
      </td>
      <td class="px-6 py-4 whitespace-nowrap">
        <div class="flex items-center">
          <img src="${info.icon_url || DEFAULT_APP_ICON}" 
               class="app-icon mr-3 w-8 h-8 rounded" 
               alt="${info.name}"
               onerror="this.src='${DEFAULT_APP_ICON}'">
          <span class="font-medium text-gray-900">${info.name || "未知"}</span>
        </div>
      </td>
      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
        ${info.developer_name || "未知"}
      </td>
      <td class="px-6 py-4 whitespace-nowrap">
        <span class="inline-flex items-center px-2 py-1 text-xs font-medium rounded-full bg-blue-100 text-blue-800">
          ${formatAppType(info.kind_type_name, info.kind_name)}
        </span>
      </td>
      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
        ${formatRating(rating)}
      </td>
      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
        ${formatNumber(rating?.total_star_rating_count || -1)}
      </td>
      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
        ${formatNumber(metric.download_count || 0)}
      </td>
      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
        ${format_size(metric.size_bytes || 0)}
      </td>
      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
        ${formatDate(metric.created_at)}
      </td>
      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
        ${formatLocalDate(info.created_at)}
      </td>
      <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
        ${formatLocalDate(info.listed_at)}
      </td>
    `;

    tableBody.appendChild(tr);
  });
}

/**
 * 渲染分页控件
 * @param currentPage - 当前页码
 * @param totalPages - 总页数
 * @param onPageChange - 页面变化回调函数
 */
export function renderPagination(
  currentPage: number,
  totalPages: number,
  onPageChange?: (page: number) => void
): void {
  const paginationEl = document.getElementById("pagination");
  if (!paginationEl) return;

  paginationEl.innerHTML = "";

  if (totalPages <= 1) return;

  const ul = document.createElement("ul");
  ul.className = "flex items-center space-x-1";

  // 第一页按钮
  const firstLi = createPaginationItem(
    "1",
    currentPage === 1,
    () => onPageChange?.(1)
  );
  ul.appendChild(firstLi);

  // 上一页按钮
  const prevLi = createPaginationItem(
    "上一页",
    currentPage === 1,
    () => onPageChange?.(currentPage - 1)
  );
  ul.appendChild(prevLi);

  // 页码数字
  const startPage = Math.max(1, currentPage - 2);
  const endPage = Math.min(totalPages, currentPage + 2);
  
  for (let i = startPage; i <= endPage; i++) {
    const li = createPaginationItem(
      i.toString(),
      i === currentPage,
      () => onPageChange?.(i),
      i === currentPage
    );
    ul.appendChild(li);
  }

  // 下一页按钮
  const nextLi = createPaginationItem(
    "下一页",
    currentPage === totalPages,
    () => onPageChange?.(currentPage + 1)
  );
  ul.appendChild(nextLi);

  // 最后一页按钮
  const lastLi = createPaginationItem(
    totalPages.toString(),
    currentPage === totalPages,
    () => onPageChange?.(totalPages)
  );
  ul.appendChild(lastLi);

  // 页码输入和跳转按钮
  const inputLi = createPageJumpInput(currentPage, totalPages, onPageChange);
  ul.appendChild(inputLi);

  paginationEl.appendChild(ul);
}

/**
 * 创建分页项
 * @param text - 显示文本
 * @param disabled - 是否禁用
 * @param onClick - 点击回调
 * @param isActive - 是否为当前页
 */
function createPaginationItem(
  text: string,
  disabled: boolean,
  onClick: () => void,
  isActive: boolean = false
): HTMLLIElement {
  const li = document.createElement("li");
  li.className = `flex ${disabled ? "opacity-50 cursor-not-allowed" : "cursor-pointer"}`;
  
  const a = document.createElement("a");
  a.textContent = text;
  
  if (isActive) {
    a.className = "px-3 py-2 text-sm font-medium rounded-md border-blue-500 bg-blue-100 text-blue-800";
  } else {
    a.className = "px-3 py-2 text-sm font-medium rounded-md border-blue-300 bg-blue-50 text-blue-700 hover:bg-blue-100";
  }
  
  if (!disabled) {
    a.onclick = (e) => {
      e.preventDefault();
      onClick();
    };
  }
  
  li.appendChild(a);
  return li;
}

/**
 * 创建页码跳转输入框
 * @param currentPage - 当前页码
 * @param totalPages - 总页数
 * @param onPageChange - 页面变化回调
 */
function createPageJumpInput(
  currentPage: number,
  totalPages: number,
  onPageChange?: (page: number) => void
): HTMLLIElement {
  const li = document.createElement("li");
  li.className = "flex items-center space-x-1";
  
  const input = document.createElement("input");
  input.type = "number";
  input.min = "1";
  input.max = totalPages.toString();
  input.value = currentPage.toString();
  input.className = "px-2 py-2 text-sm border-blue-300 bg-blue-50 text-blue-900 rounded-md w-16 text-center";
  
  const jumpBtn = document.createElement("button");
  jumpBtn.textContent = "跳转";
  jumpBtn.className = "px-3 py-2 text-sm font-medium rounded-md bg-blue-500 text-white border-blue-500 hover:bg-blue-600";
  
  jumpBtn.onclick = (e) => {
    e.preventDefault();
    const page = parseInt(input.value);
    if (!isNaN(page) && page >= 1 && page <= totalPages) {
      onPageChange?.(page);
    } else {
      input.value = currentPage.toString(); // 重置无效输入
    }
  };
  
  li.appendChild(input);
  li.appendChild(jumpBtn);
  return li;
}

/**
 * 更新表格排序图标状态
 * @param currentSort - 当前排序配置
 */
export function updateSortIcons(currentSort: { field: string; desc: boolean }): void {
  const headers = document.querySelectorAll("th[data-sort]");
  headers.forEach((header) => {
    const field = header.getAttribute("data-sort");
    const span = header.querySelector(".sort-icon");
    if (!span || !field) return;

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
 * 显示加载状态
 * @param elementId - 元素ID
 * @param show - 是否显示加载状态
 */
export function toggleLoading(elementId: string, show: boolean): void {
  const element = document.getElementById(elementId);
  if (element) {
    element.style.display = show ? "inline-block" : "none";
  }
}

/**
 * 显示表格加载状态
 * @param colspan - 列数
 */
export function showTableLoading(colspan: number = 12): void {
  const tableBody = document.getElementById("appTableBody");
  if (tableBody) {
    tableBody.innerHTML = `
      <tr>
        <td colspan="${colspan}" class="text-center py-12">
          <div class="inline-block w-8 h-8 border-2 border-blue-600 border-t-transparent rounded-full animate-spin"></div>
        </td>
      </tr>
    `;
  }
}

/**
 * 显示表格错误状态
 * @param message - 错误消息
 * @param colspan - 列数
 */
export function showTableError(message: string = "加载数据失败", colspan: number = 8): void {
  const tableBody = document.getElementById("appTableBody");
  if (tableBody) {
    tableBody.innerHTML = `
      <tr>
        <td colspan="${colspan}" class="text-center py-4 text-gray-500">
          ${message}
        </td>
      </tr>
    `;
  }
}

/**
 * 更新统计数据
 * @param totalCount - 总数
 * @param appCount - 应用数量
 * @param atomicServiceCount - 原子化服务数量
 * @param developerCount - 开发者数量
 */
export function updateStatistics({
  totalCount,
  appCount,
  atomicServiceCount,
  developerCount,
}: {
  totalCount: number;
  appCount: number;
  atomicServiceCount: number;
  developerCount: number;
}): void {
  const elements = {
    totalCount: "totalCount",
    appCount: "appCount",
    atomicServiceCount: "atomicServiceCount",
    developerCount: "developerCount",
  };

  Object.entries(elements).forEach(([key, elementId]) => {
    const element = document.getElementById(elementId);
    if (element) {
      element.textContent = formatNumber(eval(key));
    }
  });
}

/**
 * 清空表单
 * @param formId - 表单ID
 */
export function clearForm(formId?: string): void {
  const form = formId 
    ? document.getElementById(formId) as HTMLFormElement
    : document.querySelector('form') as HTMLFormElement;
    
  if (form) {
    form.reset();
  }
}

/**
 * 设置元素的显示状态
 * @param elementId - 元素ID
 * @param visible - 是否可见
 */
export function setElementVisibility(elementId: string, visible: boolean): void {
  const element = document.getElementById(elementId);
  if (element) {
    element.style.display = visible ? "" : "none";
  }
}