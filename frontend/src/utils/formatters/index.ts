// 数据格式化工具函数模块

import type { AppRating } from '../../types';

/**
 * 格式化数字，四位一分隔（中文习惯）
 * @param num - 要格式化的数字
 * @returns 格式化后的字符串
 */
export function formatNumber(num: number): string {
  if (num === -1) return '未知';
  return num.toString().replace(/\B(?=(\d{4})+(?!\d))/g, ',');
}

/**
 * 格式化文件大小，自动选择合适的单位
 * @param size - 文件大小（字节）
 * @returns 格式化后的大小字符串
 */
export function format_size(size: number): string {
  if (size < 1024) return `${size} B`;
  if (size < 1024 * 1024) return `${(size / 1024).toFixed(2)} KB`;
  if (size < 1024 * 1024 * 1024) return `${(size / (1024 * 1024)).toFixed(2)} MB`;
  return `${(size / (1024 * 1024 * 1024)).toFixed(2)} GB`;
}

/**
 * 格式化日期时间，输出为YYYY-MM-DD HH:mm格式
 * @param dateInput - 日期时间字符串或Date对象
 * @returns 格式化后的日期时间字符串
 */
export function formatDate(dateInput: string | Date): string {
  const date = new Date(dateInput);
  const pad = (n: number) => (n < 10 ? `0${n}` : n);
  
  return (
    `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())} ` +
    `${pad(date.getHours())}:${pad(date.getMinutes())}`
  );
}

/**
 * 格式化日期为本地日期字符串
 * @param dateInput - 日期时间字符串或Date对象
 * @returns 格式化后的本地日期字符串
 */
export function formatLocalDate(dateInput: string | Date): string {
  const date = new Date(dateInput);
  return date.toLocaleDateString('zh-CN');
}

/**
 * 使用Unicode字符渲染星级评分显示
 * @param rating - 评分值，范围0-5
 * @returns 星级字符串，如果无评分返回'无评分'
 */
export function renderStars(rating?: number | null): string {
  if (rating === undefined || rating === null) return '无评分';
  
  const fullStars = Math.floor(rating);
  const hasHalf = rating % 1 >= 0.5;
  let stars = '';
  
  for (let i = 0; i < 5; i++) {
    if (i < fullStars) {
      stars += '★'; // 满星
    } else if (i === fullStars && hasHalf) {
      stars += '☆'; // 半星（简化表示，可替换为更好符号）
    } else {
      stars += '☆'; // 空星
    }
  }
  
  return `${stars} ${rating.toFixed(1)}`;
}

/**
 * 格式化评分信息
 * @param rating - 评分对象
 * @returns 格式化后的评分字符串
 */
export function formatRating(rating?: AppRating): string {
  if (!rating) return '无评分';
  return `${renderStars(rating.average_rating)} (${formatNumber(rating.total_star_rating_count)})`;
}

/**
 * 格式化应用类型标签
 * @param kindTypeName - 应用类型名称
 * @param kindName - 应用分类名称
 * @returns 格式化后的类型标签
 */
export function formatAppType(kindTypeName?: string, kindName?: string): string {
  if (!kindTypeName && !kindName) return '未知';
  return `${kindTypeName || '未知'}-${kindName || '未知'}`;
}

/**
 * 格式化百分比
 * @param value - 数值
 * @param total - 总数
 * @param decimals - 小数位数，默认2位
 * @returns 格式化后的百分比字符串
 */
export function formatPercentage(value: number, total: number, decimals: number = 2): string {
  if (total === 0) return '0%';
  return `${((value / total) * 100).toFixed(decimals)}%`;
}

/**
 * 格式化时间间隔
 * @param dateInput - 日期时间字符串或Date对象
 * @returns 格式化后的时间间隔描述
 */
export function formatTimeAgo(dateInput: string | Date): string {
  const date = new Date(dateInput);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  
  const diffSeconds = Math.floor(diffMs / 1000);
  const diffMinutes = Math.floor(diffSeconds / 60);
  const diffHours = Math.floor(diffMinutes / 60);
  const diffDays = Math.floor(diffHours / 24);
  
  if (diffDays > 0) return `${diffDays}天前`;
  if (diffHours > 0) return `${diffHours}小时前`;
  if (diffMinutes > 0) return `${diffMinutes}分钟前`;
  return '刚刚';
}

/**
 * 截断文本并添加省略号
 * @param text - 原始文本
 * @param maxLength - 最大长度
 * @returns 截断后的文本
 */
export function truncateText(text: string, maxLength: number): string {
  if (text.length <= maxLength) return text;
  return `${text.slice(0, maxLength)}...`;
}

/**
 * 格式化搜索关键词高亮
 * @param text - 原始文本
 * @param searchTerm - 搜索关键词
 * @returns 高亮后的HTML字符串
 */
export function highlightSearchTerm(text: string, searchTerm: string): string {
  if (!searchTerm.trim()) return text;
  
  const regex = new RegExp(`(${searchTerm.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi');
  return text.replace(regex, '<mark class="bg-yellow-200">$1</mark>');
}