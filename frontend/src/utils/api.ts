// API数据加载工具模块

import type { 
  App, 
  AppInfo, 
  MarketInfo, 
  ApiResponse, 
  PaginatedResponse, 
  StarDistribution, 
  RankingApp,
  SortConfig,
  SearchConfig 
} from '../types';
import { API_BASE, PAGE_SIZE } from './constants';
import { showError } from './notifications';

/**
 * 通用API请求函数
 * @param url - 请求URL
 * @param options - 请求选项
 * @returns Promise<T>
 */
async function apiRequest<T>(url: string, options: RequestInit = {}): Promise<T> {
  try {
    const response = await fetch(url, {
      headers: {
        'Content-Type': 'application/json',
        ...options.headers,
      },
      ...options,
    });

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }

    return await response.json();
  } catch (error) {
    console.error('API请求失败:', error);
    throw error;
  }
}

/**
 * 加载应用市场概览统计信息
 * @returns Promise<MarketInfo>
 */
export async function loadMarketOverview(): Promise<MarketInfo> {
  try {
    const response = await apiRequest<ApiResponse<MarketInfo>>(`${API_BASE}/market_info`);
    return response.data;
  } catch (error) {
    showError('加载市场概览失败');
    throw error;
  }
}

/**
 * 加载应用列表
 * @param page - 页码
 * @param sortConfig - 排序配置
 * @param searchConfig - 搜索配置
 * @returns Promise<PaginatedResponse<App>>
 */
export async function loadAppList(
  page: number = 1,
  sortConfig: SortConfig = { field: 'download_count', desc: true },
  searchConfig: SearchConfig = { term: '', key: 'name', exact: false }
): Promise<PaginatedResponse<App>> {
  try {
    const params = new URLSearchParams({
      page: page.toString(),
      sort: sortConfig.field,
      desc: sortConfig.desc.toString(),
      page_size: PAGE_SIZE.toString(),
    });

    if (searchConfig.term.trim()) {
      params.append('search_key', searchConfig.key);
      params.append('search_value', searchConfig.term);
      params.append('search_exact', searchConfig.exact.toString());
    }

    const url = `${API_BASE}/apps/list/${page}?${params.toString()}`;
    const response = await apiRequest<PaginatedResponse<App>>(url);
    return response;
  } catch (error) {
    showError('加载应用列表失败');
    throw error;
  }
}

/**
 * 获取应用详情
 * @param appId - 应用ID
 * @returns Promise<App>
 */
export async function getAppDetail(appId: string): Promise<App> {
  try {
    const response = await apiRequest<ApiResponse<App>>(`${API_BASE}/apps/${appId}`);
    return response.data;
  } catch (error) {
    showError('加载应用详情失败');
    throw error;
  }
}

/**
 * 加载下载量排行榜
 * @param limit - 限制数量
 * @param excludePattern - 排除模式
 * @returns Promise<RankingApp[]>
 */
export async function loadDownloadRanking(
  limit: number = 20,
  excludePattern?: string
): Promise<RankingApp[]> {
  try {
    const params = new URLSearchParams({ limit: limit.toString() });
    if (excludePattern) {
      params.append('exclude_pattern', excludePattern);
    }

    const url = `${API_BASE}/rankings/top-downloads?${params.toString()}`;
    const response = await apiRequest<ApiResponse<[AppInfo, { download_count: number }][]>>(url);
    
    return response.data.map(([info, metric]) => ({
      name: info.name,
      download_count: metric.download_count,
      icon_url: info.icon_url,
      app_id: info.app_id,
      pkg_name: info.pkg_name,
    }));
  } catch (error) {
    showError('加载下载排行榜失败');
    throw error;
  }
}

/**
 * 加载星级分布数据
 * @returns Promise<StarDistribution>
 */
export async function loadStarDistribution(): Promise<StarDistribution> {
  try {
    const response = await apiRequest<ApiResponse<StarDistribution>>(`${API_BASE}/charts/star-distribution`);
    return response.data;
  } catch (error) {
    showError('加载星级分布失败');
    throw error;
  }
}

/**
 * 搜索应用
 * @param searchTerm - 搜索关键词
 * @param searchKey - 搜索字段
 * @param exact - 是否精确匹配
 * @param page - 页码
 * @param sortConfig - 排序配置
 * @returns Promise<PaginatedResponse<App>>
 */
export async function searchApps(
  searchTerm: string,
  searchKey: string = 'name',
  exact: boolean = false,
  page: number = 1,
  sortConfig: SortConfig = { field: 'download_count', desc: true }
): Promise<PaginatedResponse<App>> {
  return loadAppList(page, sortConfig, { term: searchTerm, key: searchKey, exact });
}

/**
 * 获取应用分类列表
 * @returns Promise<string[]>
 */
export async function getAppCategories(): Promise<string[]> {
  try {
    const response = await apiRequest<ApiResponse<string[]>>(`${API_BASE}/categories`);
    return response.data;
  } catch (error) {
    showError('加载应用分类失败');
    throw error;
  }
}

/**
 * 获取开发者列表
 * @returns Promise<string[]>
 */
export async function getDevelopers(): Promise<string[]> {
  try {
    const response = await apiRequest<ApiResponse<string[]>>(`${API_BASE}/developers`);
    return response.data;
  } catch (error) {
    showError('加载开发者列表失败');
    throw error;
  }
}

/**
 * 批量获取应用信息
 * @param appIds - 应用ID数组
 * @returns Promise<App[]>
 */
export async function batchGetApps(appIds: string[]): Promise<App[]> {
  try {
    const response = await apiRequest<ApiResponse<App[]>>(`${API_BASE}/apps/batch`, {
      method: 'POST',
      body: JSON.stringify({ app_ids: appIds }),
    });
    return response.data;
  } catch (error) {
    showError('批量获取应用信息失败');
    throw error;
  }
}

/**
 * 导出应用数据
 * @param format - 导出格式 (csv, json, xlsx)
 * @param searchConfig - 搜索配置
 * @param sortConfig - 排序配置
 * @returns Promise<Blob>
 */
export async function exportAppData(
  format: 'csv' | 'json' | 'xlsx' = 'csv',
  searchConfig?: SearchConfig,
  sortConfig?: SortConfig
): Promise<Blob> {
  try {
    const params = new URLSearchParams({ format });
    
    if (searchConfig?.term.trim()) {
      params.append('search_key', searchConfig.key);
      params.append('search_value', searchConfig.term);
      params.append('search_exact', searchConfig.exact.toString());
    }
    
    if (sortConfig) {
      params.append('sort', sortConfig.field);
      params.append('desc', sortConfig.desc.toString());
    }

    const response = await fetch(`${API_BASE}/export?${params.toString()}`);
    
    if (!response.ok) {
      throw new Error(`导出失败: ${response.statusText}`);
    }
    
    return await response.blob();
  } catch (error) {
    showError('导出数据失败');
    throw error;
  }
}