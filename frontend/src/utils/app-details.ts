// 应用详情显示和处理工具模块

import type { App } from '../types';
import { formatNumber, formatSize, formatDate, formatLocalDate, formatAppType, formatRating } from './formatters';
import { showCopySuccess, showCopyError } from './notifications';
import { copyToClipboard } from './clipboard';

/**
 * 解析设备代码信息
 * @param deviceCode - 设备代码
 * @returns 解析后的设备信息对象
 */
export function parseDeviceCode(deviceCode: string): {
  model: string;
  brand: string;
  version?: string;
} | null {
  try {
    // 基本的设备代码解析逻辑
    const parts = deviceCode.split('-');
    if (parts.length < 2) return null;
    
    return {
      model: parts[0] || '未知',
      brand: parts[1] || '未知',
      version: parts[2],
    };
  } catch (error) {
    console.error('解析设备代码失败:', error);
    return null;
  }
}

/**
 * 显示应用详情模态框
 * @param appId - 应用ID
 */
export async function showAppDetail(appId: string): Promise<void> {
  try {
  // 显示加载状态
  const modal = document.getElementById('appDetailModal');
  if (!modal) return;
    
  // 显示加载动画
  const loader = modal.querySelector('.loader');
  if (loader) {
    loader.classList.remove('hidden');
  }
    
  // 显示模态框
  modal.classList.remove('hidden');
    
    // 加载应用详情数据
    const response = await fetch(`/api/apps/${appId}`);
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    
    const result = await response.json();
    const app: App = result.data;
    
    // 渲染应用详情
    renderAppDetail(app);
    
  } catch (error) {
    console.error('加载应用详情失败:', error);
    
    const content = document.getElementById('appDetailContent');
    if (content) {
      content.innerHTML = `
        <div class="text-center py-8">
          <div class="text-red-500 mb-4">加载应用详情失败</div>
          <button onclick="closeAppDetail()" class="px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600">
            关闭
          </button>
        </div>
      `;
    }
  }
}

/**
 * 渲染应用详情内容
 * @param app - 应用数据
 */
function renderAppDetail(app: App): void {
  const { info, metric, rating } = app;
  const content = document.getElementById('appDetailContent');
  if (!content) return;
  
  content.innerHTML = `
    <div class="bg-white rounded-lg shadow-xl max-w-4xl max-h-[90vh] overflow-y-auto">
      <!-- 应用头部信息 -->
      <div class="p-6 border-b">
        <div class="flex items-start space-x-4">
          <img src="${info.icon_url || '/img/default-app-icon.png'}" 
               class="w-20 h-20 rounded-lg shadow-md"
               alt="${info.name}"
               onerror="this.src='/img/default-app-icon.png'">
          <div class="flex-1">
            <h2 class="text-2xl font-bold text-gray-900 mb-2">${info.name || '未知应用'}</h2>
            <div class="space-y-1">
              <p class="text-gray-600">开发者: ${info.developer_name || '未知'}</p>
              <p class="text-gray-600">应用ID: ${info.app_id}</p>
              <div class="flex items-center space-x-4">
                <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800">
                  ${formatAppType(info.kind_type_name, info.kind_name)}
                </span>
                ${rating ? `
                  <span class="text-gray-600">
                    ${formatRating(rating)}
                  </span>
                ` : ''}
              </div>
            </div>
          </div>
          <button onclick="closeAppDetail()" class="text-gray-400 hover:text-gray-600">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
            </svg>
          </button>
        </div>
      </div>
      
      <!-- 详细信息标签页 -->
      <div class="border-b">
        <nav class="flex space-x-8 px-6" aria-label="Tabs">
          <button onclick="switchDetailTab('basic')" class="detail-tab active py-4 px-1 border-b-2 font-medium text-sm" data-tab="basic">
            基本信息
          </button>
          <button onclick="switchDetailTab('metrics')" class="detail-tab py-4 px-1 border-b-2 font-medium text-sm" data-tab="metrics">
            数据指标
          </button>
          <button onclick="switchDetailTab('technical')" class="detail-tab py-4 px-1 border-b-2 font-medium text-sm" data-tab="technical">
            技术信息
          </button>
        </nav>
      </div>
      
      <!-- 标签页内容 -->
      <div class="p-6">
        <!-- 基本信息标签页 -->
        <div id="basic-tab" class="tab-content">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="space-y-4">
              <h3 class="text-lg font-semibold text-gray-900">应用信息</h3>
              <div class="space-y-2">
                <div class="flex justify-between">
                  <span class="text-gray-600">包名:</span>
                  <span class="font-mono text-sm">${info.pkg_name || '未知'}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-600">创建时间:</span>
                  <span>${formatLocalDate(info.created_at)}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-600">上架时间:</span>
                  <span>${formatLocalDate(info.listed_at)}</span>
                </div>
              </div>
            </div>
            
            <div class="space-y-4">
              <h3 class="text-lg font-semibold text-gray-900">评分信息</h3>
              <div class="space-y-2">
                <div class="flex justify-between">
                  <span class="text-gray-600">平均评分:</span>
                  <span>${rating ? rating.average_rating.toFixed(1) : '无评分'}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-600">评分人数:</span>
                  <span>${formatNumber(rating?.total_star_rating_count || 0)}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <!-- 数据指标标签页 -->
        <div id="metrics-tab" class="tab-content hidden">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="space-y-4">
              <h3 class="text-lg font-semibold text-gray-900">下载统计</h3>
              <div class="space-y-2">
                <div class="flex justify-between">
                  <span class="text-gray-600">总下载量:</span>
                  <span class="font-semibold">${formatNumber(metric.download_count || 0)}</span>
                </div>
              </div>
            </div>
            
            <div class="space-y-4">
              <h3 class="text-lg font-semibold text-gray-900">应用大小</h3>
              <div class="space-y-2">
                <div class="flex justify-between">
                  <span class="text-gray-600">安装包大小:</span>
                  <span>${formatSize(metric.size_bytes || 0)}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-600">最后更新:</span>
                  <span>${formatDate(metric.created_at)}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <!-- 技术信息标签页 -->
        <div id="technical-tab" class="tab-content hidden">
          <div class="space-y-6">
            <div>
              <h3 class="text-lg font-semibold text-gray-900 mb-4">技术信息</h3>
              <div class="bg-gray-50 rounded-lg p-4">
                <div class="space-y-2">
                  <div class="flex justify-between">
                    <span class="text-gray-600">应用ID:</span>
                    <div class="flex items-center space-x-2">
                      <span class="font-mono text-sm">${info.app_id}</span>
                      <button onclick="copyAppId('${info.app_id}')" class="text-blue-600 hover:text-blue-800">
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                        </svg>
                      </button>
                    </div>
                  </div>
                  <div class="flex justify-between">
                    <span class="text-gray-600">包名:</span>
                    <div class="flex items-center space-x-2">
                      <span class="font-mono text-sm">${info.pkg_name || '未知'}</span>
                      ${info.pkg_name ? `
                        <button onclick="copyPackageName('${info.pkg_name}')" class="text-blue-600 hover:text-blue-800">
                          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                          </svg>
                        </button>
                      ` : ''}
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 操作按钮 -->
      <div class="px-6 py-4 bg-gray-50 border-t flex justify-end space-x-3">
        <button onclick="exportAppDetail('${info.app_id}')" class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700">
          导出详情
        </button>
        <button onclick="closeAppDetail()" class="px-4 py-2 bg-gray-600 text-white rounded hover:bg-gray-700">
          关闭
        </button>
      </div>
    </div>
  `;
}

/**
 * 关闭应用详情模态框
 */
export function closeAppDetail(): void {
  const modal = document.getElementById('appDetailModal');
  if (modal) {
    modal.classList.add('hidden');
    
    // 隐藏加载动画
    const loader = modal.querySelector('.loader');
    if (loader) {
      loader.classList.add('hidden');
    }
  }
}

/**
 * 切换详情标签页
 * @param tabName - 标签页名称
 */
export function switchDetailTab(tabName: string): void {
  // 更新标签按钮状态
  document.querySelectorAll('.detail-tab').forEach(tab => {
    if (tab instanceof HTMLElement && tab.dataset.tab === tabName) {
      tab.classList.add('border-blue-500', 'text-blue-600');
      tab.classList.remove('border-transparent', 'text-gray-500');
    } else {
      tab.classList.remove('border-blue-500', 'text-blue-600');
      tab.classList.add('border-transparent', 'text-gray-500');
    }
  });
  
  // 显示对应的内容
  document.querySelectorAll('.tab-content').forEach(content => {
    content.classList.add('hidden');
  });
  
  const targetContent = document.getElementById(`${tabName}-tab`);
  if (targetContent) {
    targetContent.classList.remove('hidden');
  }
}

/**
 * 复制应用ID到剪贴板
 * @param appId - 应用ID
 */
export async function copyAppId(appId: string): Promise<void> {
  try {
    await copyToClipboard(appId);
    showCopySuccess();
  } catch (error) {
    showCopyError();
  }
}

/**
 * 复制包名到剪贴板
 * @param packageName - 包名
 */
export async function copyPackageName(packageName: string): Promise<void> {
  try {
    await copyToClipboard(packageName);
    showCopySuccess();
  } catch (error) {
    showCopyError();
  }
}

/**
 * 导出应用详情
 * @param appId - 应用ID
 */
export async function exportAppDetail(appId: string): Promise<void> {
  try {
    const response = await fetch(`/api/apps/${appId}/export`);
    if (!response.ok) {
      throw new Error(`导出失败: ${response.statusText}`);
    }
    
    const blob = await response.blob();
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `app_detail_${appId}.json`;
    document.body.appendChild(a);
    a.click();
    window.URL.revokeObjectURL(url);
    document.body.removeChild(a);
    
    showCopySuccess(); // 使用成功提示代替导出成功提示
  } catch (error) {
    console.error('导出应用详情失败:', error);
    showCopyError(); // 使用错误提示
  }
}

/**
 * 初始化应用详情模态框事件监听
 */
export function initAppDetailModal(): void {
  // 点击模态框背景关闭
  const modal = document.getElementById('appDetailModal');
  if (modal) {
    modal.addEventListener('click', (e) => {
      if (e.target === modal) {
        closeAppDetail();
      }
    });
  }
  
  // ESC键关闭模态框
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
      closeAppDetail();
    }
  });
}

// 全局函数声明，供HTML调用
declare global {
  interface Window {
    showAppDetail: (appId: string) => void;
    closeAppDetail: () => void;
    switchDetailTab: (tabName: string) => void;
    copyAppId: (appId: string) => void;
    copyPackageName: (packageName: string) => void;
    exportAppDetail: (appId: string) => void;
  }
}

// 注册全局函数
if (typeof window !== 'undefined') {
  window.showAppDetail = showAppDetail;
  window.closeAppDetail = closeAppDetail;
  window.switchDetailTab = switchDetailTab;
  window.copyAppId = copyAppId;
  window.copyPackageName = copyPackageName;
  window.exportAppDetail = exportAppDetail;
}