// 工具函数模块

// URL参数和剪贴板工具
export { updateUrlParam, getUrlParams } from './url-params.ts';
export { copyToClipboard } from './clipboard.ts';

// 格式化工具
export {
  formatNumber,
  format_size as formatSize,
  formatDate,
  formatLocalDate,
  formatTimeAgo,
  formatPercentage,
  truncateText,
  highlightSearchTerm,
  renderStars,
  formatRating,
  formatAppType,
} from './formatters';

// 常量定义
export * from './constants';

// 通知消息工具
export {
  showNotification,
  showSuccess,
  showError,
  showWarning,
  showInfo,
  showCopySuccess,
  showCopyError,
  clearAllNotifications,
  type NotificationOptions,
  type MessageType,
} from './notifications';

// API数据加载工具
export {
  loadMarketOverview,
  loadAppList,
  getAppDetail,
  loadDownloadRanking,
  loadStarDistribution,
  searchApps,
  getAppCategories,
  getDevelopers,
  batchGetApps,
  exportAppData,
} from './api';

// DOM操作和渲染工具
export {
  updateLastUpdate,
  renderAppTable,
  renderPagination,
  updateSortIcons,
  toggleLoading,
  showTableLoading,
  showTableError,
  updateStatistics,
  clearForm,
  setElementVisibility,
} from './dom';

// 图表工具
export {
  preloadImages,
  renderDownloadChart,
  renderStarChart,
  loadAllCharts,
  destroyAllCharts,
  updateChartTheme,
} from './charts';

// 应用详情工具
export {
  parseDeviceCode,
  showAppDetail,
  closeAppDetail,
  switchDetailTab,
  copyAppId,
  copyPackageName,
  exportAppDetail,
  initAppDetailModal,
} from './app-details';
