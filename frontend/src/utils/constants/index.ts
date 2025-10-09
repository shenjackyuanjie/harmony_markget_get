// API相关常量
export const API_BASE = '/api';

// 分页相关常量
export const PAGE_SIZE = 50;

// 搜索字段选项
export const SEARCH_FIELDS = [
  { value: 'name', label: '应用名称' },
  { value: 'developer_name', label: '开发者' },
  { value: 'pkg_name', label: '包名' },
  { value: 'app_id', label: '应用ID' },
  { value: 'kind_name', label: '分类' },
] as const;

// 排序字段选项
export const SORT_FIELDS = [
  { value: 'download_count', label: '下载量' },
  { value: 'created_at', label: '创建时间' },
  { value: 'listed_at', label: '上架时间' },
  { value: 'name', label: '应用名称' },
  { value: 'developer_name', label: '开发者' },
  { value: 'average_rating', label: '评分' },
  { value: 'total_star_rating_count', label: '评分数量' },
  { value: 'size_bytes', label: '应用大小' },
] as const;

// 应用类型
export const APP_TYPES = {
  APP: '应用',
  ATOMIC_SERVICE: '原子化服务',
} as const;

// 默认应用图标
export const DEFAULT_APP_ICON = '/img/default-app-icon.png';

// 图表相关常量
export const CHART_COLORS = {
  STAR_RATING: [
    '#ef4444', // 无评分 - 红色
    '#f97316', // [1-2)星 - 橙色
    '#eab308', // [2-3)星 - 黄色
    '#22c55e', // [3-4)星 - 绿色
    '#0ea5e9', // [4-5]星 - 蓝色
  ],
  DOWNLOAD_BAR: 'rgba(59, 130, 246, 0.6)',
  DOWNLOAD_BORDER: 'rgba(59, 130, 246, 1)',
} as const;

// 加载状态
export const LOADING_STATES = {
  IDLE: 'idle',
  LOADING: 'loading',
  SUCCESS: 'success',
  ERROR: 'error',
} as const;

// 消息类型
export const MESSAGE_TYPES = {
  SUCCESS: 'success',
  ERROR: 'error',
  WARNING: 'warning',
  INFO: 'info',
} as const;