// 仪表板状态管理工具

import type { 
  App, 
  MarketInfo, 
  SortConfig, 
  SearchConfig, 
  StarDistribution,
  RankingApp 
} from '../types';
import { 
  loadMarketOverview, 
  loadAppList, 
  loadDownloadRanking, 
  loadStarDistribution,
  searchApps 
} from '../utils/api';
import { 
  renderAppTable, 
  renderPagination, 
  updateSortIcons, 
  updateStatistics,
  showTableLoading,
  showTableError,
  toggleLoading 
} from '../utils/dom';
import { 
  showSuccess, 
  showError 
} from '../utils/notifications';
import { 
  PAGE_SIZE, 
  LOADING_STATES 
} from '../utils/constants';

export interface DashboardState {
  // 加载状态
  loadingState: string;
  
  // 数据
  apps: App[];
  marketInfo: MarketInfo | null;
  starDistribution: StarDistribution | null;
  downloadRanking: RankingApp[];
  downloadRankingExcludingHuawei: RankingApp[];
  
  // 分页和排序
  currentPage: number;
  totalPages: number;
  sortConfig: SortConfig;
  searchConfig: SearchConfig;
  
  // 统计数据
  totalCount: number;
  appCount: number;
  atomicServiceCount: number;
  developerCount: number;
}

class DashboardStore {
  private state: DashboardState;
  private listeners: Set<() => void> = new Set();
  
  constructor() {
    this.state = {
      loadingState: LOADING_STATES.IDLE,
      apps: [],
      marketInfo: null,
      starDistribution: null,
      downloadRanking: [],
      downloadRankingExcludingHuawei: [],
      currentPage: 1,
      totalPages: 1,
      sortConfig: { field: 'download_count', desc: true },
      searchConfig: { term: '', key: 'name', exact: false },
      totalCount: 0,
      appCount: 0,
      atomicServiceCount: 0,
      developerCount: 0,
    };
  }
  
  // 获取当前状态
  getState(): DashboardState {
    return { ...this.state };
  }
  
  // 订阅状态变化
  subscribe(listener: () => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }
  
  // 通知状态变化
  private notify(): void {
    this.listeners.forEach(listener => listener());
  }
  
  // 更新状态
  private setState(updates: Partial<DashboardState>): void {
    this.state = { ...this.state, ...updates };
    this.notify();
  }
  
  // 设置加载状态
  setLoadingState(loadingState: string): void {
    this.setState({ loadingState });
  }
  
  // 加载市场概览
  async loadMarketOverview(): Promise<void> {
    try {
      toggleLoading('loadingOverview', true);
      toggleLoading('loadingDeveloperCount', true);
      toggleLoading('loadingAtomicServiceCount', true);
      toggleLoading('loadingTotalCount', true);
      
      const marketInfo = await loadMarketOverview();
      
      this.setState({
        marketInfo,
        totalCount: marketInfo.app_count.total,
        appCount: marketInfo.app_count.apps,
        atomicServiceCount: marketInfo.app_count.atomic_services,
        developerCount: marketInfo.developer_count,
      });
      
      updateStatistics({
        totalCount: marketInfo.app_count.total,
        appCount: marketInfo.app_count.apps,
        atomicServiceCount: marketInfo.app_count.atomic_services,
        developerCount: marketInfo.developer_count,
      });
      
    } catch (error) {
      console.error('加载市场概览失败:', error);
      showError('加载市场概览失败');
    } finally {
      toggleLoading('loadingOverview', false);
      toggleLoading('loadingDeveloperCount', false);
      toggleLoading('loadingAtomicServiceCount', false);
      toggleLoading('loadingTotalCount', false);
    }
  }
  
  // 加载应用列表
  async loadApps(page: number = this.state.currentPage): Promise<void> {
    try {
      this.setLoadingState(LOADING_STATES.LOADING);
      showTableLoading();
      
      const response = await loadAppList(page, this.state.sortConfig, this.state.searchConfig);
      
      const apps = response.data.data || [];
      const totalPages = Math.ceil(response.data.total_count / PAGE_SIZE);
      
      this.setState({
        apps,
        currentPage: page,
        totalPages,
        loadingState: LOADING_STATES.SUCCESS,
      });
      
      renderAppTable(apps, page, (appId: string) => {
        // 应用点击处理
        if (typeof window.showAppDetail === 'function') {
          window.showAppDetail(appId);
        }
      });
      
      renderPagination(page, totalPages, (newPage: number) => {
        this.loadApps(newPage);
      });
      
    } catch (error) {
      console.error('加载应用列表失败:', error);
      this.setLoadingState(LOADING_STATES.ERROR);
      showTableError();
      showError('加载应用列表失败');
    }
  }
  
  // 搜索应用
  async searchApps(searchTerm: string, searchKey: string = 'name', exact: boolean = false): Promise<void> {
    this.setState({
      searchConfig: { term: searchTerm, key: searchKey, exact },
      currentPage: 1,
    });
    
    await this.loadApps(1);
  }
  
  // 排序应用
  async sortApps(field: string, desc?: boolean): Promise<void> {
    const currentDesc = desc !== undefined ? desc : 
      this.state.sortConfig.field === field ? !this.state.sortConfig.desc : true;
    
    this.setState({
      sortConfig: { field, desc: currentDesc },
    });
    
    updateSortIcons({ field, desc: currentDesc });
    await this.loadApps(this.state.currentPage);
  }
  
  // 刷新当前页面
  async refresh(): Promise<void> {
    await Promise.all([
      this.loadMarketOverview(),
      this.loadApps(this.state.currentPage),
    ]);
    
    showSuccess('数据已刷新');
  }
  
  // 加载图表数据
  async loadChartData(): Promise<void> {
    try {
      const [downloadRanking, downloadRankingExcludingHuawei, starDistribution] = await Promise.all([
        loadDownloadRanking(20),
        loadDownloadRanking(30, 'huawei'),
        loadStarDistribution(),
      ]);
      
      this.setState({
        downloadRanking,
        downloadRankingExcludingHuawei,
        starDistribution,
      });
      
      // 加载图表
      if (typeof window.loadAllCharts === 'function') {
        window.loadAllCharts();
      }
      
    } catch (error) {
      console.error('加载图表数据失败:', error);
      showError('加载图表数据失败');
    }
  }
  
  // 清空搜索
  clearSearch(): void {
    this.setState({
      searchConfig: { term: '', key: 'name', exact: false },
      currentPage: 1,
    });
    this.loadApps(1);
  }
  
  // 转到指定页面
  goToPage(page: number): void {
    if (page >= 1 && page <= this.state.totalPages) {
      this.loadApps(page);
    }
  }
  
  // 上一页
  previousPage(): void {
    if (this.state.currentPage > 1) {
      this.goToPage(this.state.currentPage - 1);
    }
  }
  
  // 下一页
  nextPage(): void {
    if (this.state.currentPage < this.state.totalPages) {
      this.goToPage(this.state.currentPage + 1);
    }
  }
  
  // 获取当前应用的详细信息
  getAppById(appId: string): App | undefined {
    return this.state.apps.find(app => app.info.app_id === appId);
  }
  
  // 获取搜索建议
  async getSearchSuggestions(query: string, field: string): Promise<string[]> {
    if (!query.trim()) return [];
    
    try {
      // 这里可以调用API获取搜索建议
      // const response = await apiRequest(`/api/suggestions?query=${encodeURIComponent(query)}&field=${field}`);
      // return response.data;
      
      // 临时实现：从现有数据中提取建议
      const suggestions = new Set<string>();
      this.state.apps.forEach(app => {
        let value = '';
        switch (field) {
          case 'name':
            value = app.info.name;
            break;
          case 'developer_name':
            value = app.info.developer_name;
            break;
          case 'pkg_name':
            value = app.info.pkg_name || '';
            break;
          case 'kind_name':
            value = app.info.kind_name || '';
            break;
        }
        
        if (value.toLowerCase().includes(query.toLowerCase())) {
          suggestions.add(value);
        }
      });
      
      return Array.from(suggestions).slice(0, 10);
    } catch (error) {
      console.error('获取搜索建议失败:', error);
      return [];
    }
  }
  
  // 导出当前数据
  async exportCurrentData(format: 'csv' | 'json' | 'xlsx' = 'csv'): Promise<void> {
    try {
      this.setLoadingState(LOADING_STATES.LOADING);
      
      // 这里可以调用导出API
      // const blob = await exportAppData(format, this.state.searchConfig, this.state.sortConfig);
      // downloadBlob(blob, `apps_export_${Date.now()}.${format}`);
      
      showSuccess(`数据已导出为 ${format.toUpperCase()} 格式`);
    } catch (error) {
      console.error('导出数据失败:', error);
      showError('导出数据失败');
    } finally {
      this.setLoadingState(LOADING_STATES.SUCCESS);
    }
  }
}

// 创建单例实例
export const dashboardStore = new DashboardStore();

// 全局类型声明
declare global {
  interface Window {
    showAppDetail?: (appId: string) => void;
    loadAllCharts?: () => Promise<void>;
  }
}
