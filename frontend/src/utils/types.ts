// 应用信息类型
export interface AppInfo {
  app_id: string;
  name: string;
  developer_name: string;
  icon_url?: string;
  kind_type_name?: string;
  kind_name?: string;
  created_at: string;
  listed_at: string;
  pkg_name?: string;
}

// 应用指标类型
export interface AppMetric {
  download_count: number;
  size_bytes: number;
  created_at: string;
}

// 应用评分类型
export interface AppRating {
  average_rating: number;
  total_star_rating_count: number;
}

// 应用数据类型
export interface App {
  info: AppInfo;
  metric: AppMetric;
  rating?: AppRating;
}

// 应用统计类型
export interface AppCount {
  total: number;
  apps: number;
  atomic_services: number;
}

// 市场信息类型
export interface MarketInfo {
  app_count: AppCount;
  developer_count: number;
}

// API响应基础类型
export interface ApiResponse<T> {
  success: boolean;
  data: T;
  message?: string;
}

// 分页响应类型
export interface PaginatedResponse<T> {
  data: {
    data: T[];
    total_count: number;
    current_page: number;
    total_pages: number;
  };
}

// 排序配置类型
export interface SortConfig {
  field: string;
  desc: boolean;
}

// 搜索配置类型
export interface SearchConfig {
  term: string;
  key: string;
  exact: boolean;
}

// 星级分布数据类型
export interface StarDistribution {
  star_1: number;
  star_2: number;
  star_3: number;
  star_4: number;
  star_5: number;
}

// 排行榜应用类型
export interface RankingApp {
  name: string;
  download_count: number;
  icon_url?: string;
  app_id: string;
  pkg_name?: string;
}