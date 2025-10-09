// 通知消息显示工具模块

import { MESSAGE_TYPES } from './constants';

export type MessageType = typeof MESSAGE_TYPES[keyof typeof MESSAGE_TYPES];

export interface NotificationOptions {
  type?: MessageType;
  duration?: number;
  position?: 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left' | 'top-center' | 'bottom-center';
  showClose?: boolean;
  autoClose?: boolean;
}

/**
 * 显示通知消息
 * @param message - 消息内容
 * @param options - 通知选项
 */
export function showNotification(message: string, options: NotificationOptions = {}): void {
  const {
    type = MESSAGE_TYPES.INFO,
    duration = 3000,
    position = 'top-right',
    showClose = true,
    autoClose = true,
  } = options;

  // 创建通知容器（如果不存在）
  let container = document.getElementById('notification-container');
  if (!container) {
    container = document.createElement('div');
    container.id = 'notification-container';
    container.className = `fixed z-50 p-4 space-y-2 ${getPositionClasses(position)}`;
    document.body.appendChild(container);
  }

  // 创建通知元素
  const notification = document.createElement('div');
  notification.className = getNotificationClasses(type);
  
  const content = document.createElement('div');
  content.className = 'flex items-center justify-between';
  
  const messageElement = document.createElement('span');
  messageElement.textContent = message;
  messageElement.className = 'flex-1';
  
  content.appendChild(messageElement);
  
  // 添加关闭按钮
  if (showClose) {
    const closeBtn = document.createElement('button');
    closeBtn.innerHTML = '&times;';
    closeBtn.className = 'ml-4 text-lg leading-none hover:opacity-75';
    closeBtn.onclick = () => removeNotification(notification);
    content.appendChild(closeBtn);
  }
  
  notification.appendChild(content);
  container.appendChild(notification);

  // 动画显示
  requestAnimationFrame(() => {
    notification.style.opacity = '1';
    notification.style.transform = 'translateY(0)';
  });

  // 自动关闭
  if (autoClose && duration > 0) {
    setTimeout(() => removeNotification(notification), duration);
  }
}

/**
 * 显示成功消息
 * @param message - 消息内容
 * @param duration - 显示时长（毫秒）
 */
export function showSuccess(message: string, duration?: number): void {
  showNotification(message, { type: MESSAGE_TYPES.SUCCESS, duration });
}

/**
 * 显示错误消息
 * @param message - 消息内容
 * @param duration - 显示时长（毫秒）
 */
export function showError(message: string, duration?: number): void {
  showNotification(message, { type: MESSAGE_TYPES.ERROR, duration: 5000 });
}

/**
 * 显示警告消息
 * @param message - 消息内容
 * @param duration - 显示时长（毫秒）
 */
export function showWarning(message: string, duration?: number): void {
  showNotification(message, { type: MESSAGE_TYPES.WARNING, duration });
}

/**
 * 显示信息消息
 * @param message - 消息内容
 * @param duration - 显示时长（毫秒）
 */
export function showInfo(message: string, duration?: number): void {
  showNotification(message, { type: MESSAGE_TYPES.INFO, duration });
}

/**
 * 显示复制成功消息
 */
export function showCopySuccess(): void {
  showSuccess('已复制到剪贴板', 2000);
}

/**
 * 显示复制错误消息
 */
export function showCopyError(): void {
  showError('复制失败，请手动复制', 3000);
}

/**
 * 获取位置样式类名
 * @param position - 位置
 * @returns CSS类名字符串
 */
function getPositionClasses(position: string): string {
  const positions: Record<string, string> = {
    'top-right': 'top-0 right-0',
    'top-left': 'top-0 left-0',
    'bottom-right': 'bottom-0 right-0',
    'bottom-left': 'bottom-0 left-0',
    'top-center': 'top-0 left-1/2 transform -translate-x-1/2',
    'bottom-center': 'bottom-0 left-1/2 transform -translate-x-1/2',
  };
  return positions[position] || positions['top-right'];
}

/**
 * 获取通知样式类名
 * @param type - 消息类型
 * @returns CSS类名字符串
 */
function getNotificationClasses(type: MessageType): string {
  const baseClasses = 'p-4 rounded-lg shadow-lg max-w-sm w-full transform transition-all duration-300 ease-in-out opacity-0 -translate-y-2';
  
  const typeClasses: Record<MessageType, string> = {
    [MESSAGE_TYPES.SUCCESS]: 'bg-green-500 text-white',
    [MESSAGE_TYPES.ERROR]: 'bg-red-500 text-white',
    [MESSAGE_TYPES.WARNING]: 'bg-yellow-500 text-white',
    [MESSAGE_TYPES.INFO]: 'bg-blue-500 text-white',
  };
  
  return `${baseClasses} ${typeClasses[type]}`;
}

/**
 * 移除通知元素
 * @param notification - 通知元素
 */
function removeNotification(notification: HTMLElement): void {
  notification.style.opacity = '0';
  notification.style.transform = 'translateY(-10px)';
  
  setTimeout(() => {
    notification.remove();
    
    // 如果容器为空，移除容器
    const container = document.getElementById('notification-container');
    if (container && container.children.length === 0) {
      container.remove();
    }
  }, 300);
}

/**
 * 清除所有通知
 */
export function clearAllNotifications(): void {
  const container = document.getElementById('notification-container');
  if (container) {
    container.remove();
  }
}