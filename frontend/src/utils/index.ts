// 工具函数模块

export { updateUrlParam, getUrlParams } from './url-params.ts';
export { copyToClipboard } from './clipboard.ts';

// 更新最后更新时间
export function updateLastUpdate(): void {
  const lastUpdateElement = document.getElementById("lastUpdate");
  if (lastUpdateElement) {
    const now = new Date();
    const formattedTime = now.toLocaleString("zh-CN", {
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    });
    lastUpdateElement.textContent = formattedTime;
  }
}
