// 应用查询模块
import { API_BASE } from '../config';
import { updateUrlParam, getUrlParams } from '../utils/url-params';
import { copyToClipboard } from '../utils/clipboard';

// 链接解析
export function parseLink(): void {
  const linkInput = document.getElementById("linkInput") as HTMLInputElement;
  const pkgInput = document.getElementById("pkgInput") as HTMLInputElement;
  const appIdInput = document.getElementById("appIdInput") as HTMLInputElement;
  
  if (!linkInput || !pkgInput || !appIdInput) return;

  const link = linkInput.value.trim();
  
  if (!link) {
    pkgInput.value = "";
    appIdInput.value = "";
    return;
  }

  // 解析 Google Play 链接
  const playStoreMatch = link.match(/play\.google\.com\/store\/apps\/details\?id=([^&]+)/);
  if (playStoreMatch) {
    pkgInput.value = playStoreMatch[1];
    appIdInput.value = "";
    return;
  }

  // 解析 Apple App Store 链接
  const appStoreMatch = link.match(/apps\.apple\.com\/[a-z]{2}\/app\/[^/]+\/id(\d+)/);
  if (appStoreMatch) {
    appIdInput.value = appStoreMatch[1];
    pkgInput.value = "";
    return;
  }

  // 如果链接不匹配已知格式，清空其他输入
  pkgInput.value = "";
  appIdInput.value = "";
}

// 输入验证
export function validateInputs(): boolean {
  const pkgInput = document.getElementById("pkgInput") as HTMLInputElement;
  const appIdInput = document.getElementById("appIdInput") as HTMLInputElement;
  const linkInput = document.getElementById("linkInput") as HTMLInputElement;
  
  const pkgError = document.getElementById("pkgError");
  const appIdError = document.getElementById("appIdError");
  const linkError = document.getElementById("linkError");

  let isValid = true;

  // 重置错误状态
  if (pkgError) pkgError.classList.add("hidden");
  if (appIdError) appIdError.classList.add("hidden");
  if (linkError) linkError.classList.add("hidden");

  const pkgValue = pkgInput?.value.trim() || "";
  const appIdValue = appIdInput?.value.trim() || "";
  const linkValue = linkInput?.value.trim() || "";

  // 至少需要一个有效输入
  if (!pkgValue && !appIdValue && !linkValue) {
    if (linkError) {
      linkError.textContent = "请至少填写一个查询条件";
      linkError.classList.remove("hidden");
    }
    isValid = false;
  }

  // 包名格式验证（如果填写了包名）
  if (pkgValue && !/^[a-zA-Z][a-zA-Z0-9_]*(\.[a-zA-Z][a-zA-Z0-9_]*)+$/.test(pkgValue)) {
    if (pkgError) {
      pkgError.textContent = "包名格式不正确";
      pkgError.classList.remove("hidden");
    }
    isValid = false;
  }

  // App ID 格式验证（如果填写了App ID）
  if (appIdValue && !/^\d+$/.test(appIdValue)) {
    if (appIdError) {
      appIdError.textContent = "App ID 必须是数字";
      appIdError.classList.remove("hidden");
    }
    isValid = false;
  }

  return isValid;
}

// 应用查询
export async function queryApp(): Promise<void> {
  const pkgInput = document.getElementById("pkgInput") as HTMLInputElement;
  const appIdInput = document.getElementById("appIdInput") as HTMLInputElement;
  const linkInput = document.getElementById("linkInput") as HTMLInputElement;
  const resultArea = document.getElementById("resultArea");
  const loadingIndicator = document.getElementById("loadingIndicator");

  if (!pkgInput || !appIdInput || !linkInput || !resultArea || !loadingIndicator) return;

  const pkgName = pkgInput.value.trim();
  const appId = appIdInput.value.trim();
  const link = linkInput.value.trim();

  // 显示加载状态
  loadingIndicator.classList.remove("hidden");
  resultArea.classList.add("hidden");

  try {
    // 构建查询参数
    const params = new URLSearchParams();
    if (pkgName) params.append("pkg_name", pkgName);
    if (appId) params.append("app_id", appId);
    if (link) params.append("link", link);

    const response = await fetch(`${API_BASE}/query?${params.toString()}`);

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data = await response.json();

    // 渲染结果
    renderResult(data);

    // 隐藏加载状态
    loadingIndicator.classList.add("hidden");
    resultArea.classList.remove("hidden");

  } catch (error) {
    console.error("查询失败:", error);
    
    // 显示错误信息
    const errorMessage = document.getElementById("errorMessage");
    if (errorMessage) {
      errorMessage.textContent = "查询失败，请检查网络连接或稍后重试";
      errorMessage.classList.remove("hidden");
    }
    
    loadingIndicator.classList.add("hidden");
    resultArea.classList.add("hidden");
  }
}

// 渲染查询结果
export function renderResult(data: any): void {
  const resultContent = document.getElementById("resultContent");
  if (!resultContent) return;

  if (data.error) {
    resultContent.innerHTML = `
      <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
        <strong>错误:</strong> ${data.error}
      </div>
    `;
    return;
  }

  const app = data.app;
  if (!app) {
    resultContent.innerHTML = `
      <div class="bg-yellow-100 border border-yellow-400 text-yellow-700 px-4 py-3 rounded">
        未找到匹配的应用
      </div>
    `;
    return;
  }

  // 渲染应用信息
  resultContent.innerHTML = `
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md p-6">
      <div class="flex items-start space-x-4">
        <img src="${app.icon_url || '/static/images/default-app-icon.png'}" 
             alt="${app.name}" 
             class="w-16 h-16 rounded-lg">
        <div class="flex-1">
          <h3 class="text-xl font-bold text-gray-900 dark:text-white">${app.name}</h3>
          <p class="text-gray-600 dark:text-gray-300 mt-1">${app.pkg_name || app.app_id}</p>
          <div class="mt-2 space-y-1">
            ${app.version ? `<p class="text-sm text-gray-500 dark:text-gray-400">版本: ${app.version}</p>` : ''}
            ${app.size ? `<p class="text-sm text-gray-500 dark:text-gray-400">大小: ${app.size}</p>` : ''}
            ${app.update_time ? `<p class="text-sm text-gray-500 dark:text-gray-400">更新时间: ${new Date(app.update_time).toLocaleDateString()}</p>` : ''}
          </div>
        </div>
      </div>
      
      ${app.description ? `
        <div class="mt-4">
          <h4 class="font-semibold text-gray-900 dark:text-white mb-2">应用描述</h4>
          <p class="text-gray-600 dark:text-gray-300 text-sm">${app.description}</p>
        </div>
      ` : ''}
      
      <div class="mt-4 flex space-x-2">
        <button onclick="window.copyToClipboard('${app.pkg_name || app.app_id}', this)" 
                class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-lg text-sm transition-colors">
          <i class="fas fa-copy mr-1"></i>复制标识
        </button>
        ${app.download_url ? `
          <a href="${app.download_url}" 
             target="_blank" 
             class="bg-green-500 hover:bg-green-600 text-white px-4 py-2 rounded-lg text-sm transition-colors">
            <i class="fas fa-download mr-1"></i>下载
          </a>
        ` : ''}
      </div>
    </div>
  `;
}

// 清空表单
export function clearForm(): void {
  const inputs = [
    "linkInput",
    "pkgInput", 
    "appIdInput",
    "usernameInput",
    "remarkInput"
  ];
  
  const errors = [
    "linkError",
    "pkgError",
    "appIdError"
  ];

  // 清空输入框
  inputs.forEach(id => {
    const element = document.getElementById(id) as HTMLInputElement;
    if (element) {
      if (id === "usernameInput") {
        // 从localStorage中重新加载用户名
        const savedUsername = localStorage.getItem('submitUsername');
        element.value = savedUsername || "";
      } else {
        element.value = "";
      }
    }
  });

  // 隐藏错误信息
  errors.forEach(id => {
    const element = document.getElementById(id);
    if (element) {
      element.classList.add("hidden");
    }
  });

  // 隐藏结果区域
  const resultArea = document.getElementById("resultArea");
  if (resultArea) {
    resultArea.classList.add("hidden");
  }
}