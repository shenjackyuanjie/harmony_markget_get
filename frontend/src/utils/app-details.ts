// 应用详情显示和处理工具模块

import type { App } from '../types';
import { formatNumber, format_size, formatDate, formatLocalDate, formatAppType, formatRating } from './formatters';
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
    // 显示模态框
    const modal = document.getElementById('appDetailModal');
    if (!modal) return;

    // 显示加载状态
    showLoadingState();

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
    showErrorState();
  }
}

/**
 * 显示加载状态
 */
function showLoadingState(): void {
  const content = document.getElementById('appDetailContent');
  if (!content) return;

  const loader = content.querySelector('.loader');
  const detailContent = content.querySelector('.app-detail-content');
  const errorContent = content.querySelector('.error-content');

  if (loader) loader.classList.remove('hidden');
  if (detailContent) detailContent.classList.add('hidden');
  if (errorContent) errorContent.classList.add('hidden');
}

/**
 * 显示错误状态
 */
function showErrorState(): void {
  const content = document.getElementById('appDetailContent');
  if (!content) return;

  const loader = content.querySelector('.loader');
  const detailContent = content.querySelector('.app-detail-content');
  const errorContent = content.querySelector('.error-content');

  if (loader) loader.classList.add('hidden');
  if (detailContent) detailContent.classList.add('hidden');
  if (errorContent) errorContent.classList.remove('hidden');
}

/**
 * 显示详情内容
 */
function showDetailContent(): void {
  const content = document.getElementById('appDetailContent');
  if (!content) return;

  const loader = content.querySelector('.loader');
  const detailContent = content.querySelector('.app-detail-content');
  const errorContent = content.querySelector('.error-content');

  if (loader) loader.classList.add('hidden');
  if (detailContent) detailContent.classList.remove('hidden');
  if (errorContent) errorContent.classList.add('hidden');
}

/**
 * 渲染应用详情内容
 * @param app - 应用数据
 */
function renderAppDetail(app: App): void {
  const { info, metric, rating } = app;

  // 更新应用头部信息
  updateAppHeader(info, rating);

  // 更新基本信息标签页
  updateBasicInfoTab(info, rating);

  // 更新数据指标标签页
  updateMetricsTab(metric);

  // 更新技术信息标签页
  updateTechnicalInfoTab(info);

  // 更新操作按钮
  updateActionButtons(info.app_id);

  // 显示详情内容
  showDetailContent();

  // 默认显示基本信息标签页
  switchDetailTab('basic');
}

/**
 * 更新应用头部信息
 */
function updateAppHeader(info: App['info'], rating?: App['rating']): void {
  const icon = document.getElementById('appDetailIcon') as HTMLImageElement;
  const name = document.getElementById('appDetailName');
  const developer = document.getElementById('appDetailDeveloper');
  const appId = document.getElementById('appDetailAppId');
  const type = document.getElementById('appDetailType');
  const ratingElement = document.getElementById('appDetailRating');

  if (icon) icon.src = info.icon_url || '/img/default-app-icon.png';
  if (name) name.textContent = info.name || '未知应用';
  if (developer) developer.textContent = `开发者: ${info.developer_name || '未知'}`;
  if (appId) appId.textContent = `应用ID: ${info.app_id}`;
  if (type) type.textContent = formatAppType(info.kind_type_name, info.kind_name);
  if (ratingElement) {
    ratingElement.textContent = rating ? formatRating(rating) : '';
  }
}

/**
 * 更新基本信息标签页
 */
function updateBasicInfoTab(info: App['info'], rating?: App['rating']): void {
  const packageName = document.getElementById('appDetailPackageName');
  const createdAt = document.getElementById('appDetailCreatedAt');
  const listedAt = document.getElementById('appDetailListedAt');
  const averageRating = document.getElementById('appDetailAverageRating');
  const ratingCount = document.getElementById('appDetailRatingCount');

  if (packageName) packageName.textContent = info.pkg_name || '未知';
  if (createdAt) createdAt.textContent = formatLocalDate(info.created_at);
  if (listedAt) listedAt.textContent = formatLocalDate(info.listed_at);
  if (averageRating) {
    averageRating.textContent = rating ? rating.average_rating.toFixed(1) : '无评分';
  }
  if (ratingCount) {
    ratingCount.textContent = formatNumber(rating?.total_star_rating_count || 0);
  }
}

/**
 * 更新数据指标标签页
 */
function updateMetricsTab(metric: App['metric']): void {
  const downloadCount = document.getElementById('appDetailDownloadCount');
  const size = document.getElementById('appDetailSize');
  const updatedAt = document.getElementById('appDetailUpdatedAt');

  if (downloadCount) downloadCount.textContent = formatNumber(metric.download_count || 0);
  if (size) size.textContent = format_size(metric.size_bytes || 0);
  if (updatedAt) updatedAt.textContent = formatDate(metric.created_at);
}

/**
 * 更新技术信息标签页
 */
function updateTechnicalInfoTab(info: App['info']): void {
  const technicalAppId = document.getElementById('appDetailTechnicalAppId');
  const technicalPackageName = document.getElementById('appDetailTechnicalPackageName');
  const copyAppIdButton = document.querySelector('[onclick^="copyAppId"]') as HTMLButtonElement;
  const copyPackageNameButton = document.querySelector('[onclick^="copyPackageName"]') as HTMLButtonElement;

  if (technicalAppId) technicalAppId.textContent = info.app_id;
  if (technicalPackageName) technicalPackageName.textContent = info.pkg_name || '未知';

  // 更新复制按钮的onclick事件
  if (copyAppIdButton) {
    copyAppIdButton.onclick = () => copyAppId(info.app_id);
  }

  if (copyPackageNameButton && info.pkg_name) {
    copyPackageNameButton.onclick = () => copyPackageName(info.pkg_name!);
  }
}

/**
 * 更新操作按钮
 */
function updateActionButtons(appId: string): void {
  const exportButton = document.querySelector('[onclick^="exportAppDetail"]') as HTMLButtonElement;

  if (exportButton) {
    exportButton.onclick = () => exportAppDetail(appId);
  }
}

/**
 * 关闭应用详情模态框
 */
export function closeAppDetail(): void {
  const modal = document.getElementById('appDetailModal');
  if (modal) {
    modal.classList.add('hidden');
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
    copyToClipboard(appId);
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
    copyToClipboard(packageName);
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
    showAppDetail?: (appId: string) => void;
    closeAppDetail?: () => void;
    switchDetailTab?: (tabName: string) => void;
    copyAppId?: (appId: string) => void;
    copyPackageName?: (packageName: string) => void;
    exportAppDetail?: (appId: string) => void;
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
