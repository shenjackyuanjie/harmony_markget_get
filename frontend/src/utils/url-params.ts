// URL参数处理工具

// 获取URL参数
export function getUrlParams(): Record<string, string> {
  const params = new URLSearchParams(window.location.search);
  const result: Record<string, string> = {};

  for (const [key, value] of params) {
    result[key] = value;
  }

  return result;
}

// 更新URL参数
export function updateUrlParam(key: string, value: string): void {
  const url = new URL(window.location.href);

  if (value) {
    url.searchParams.set(key, value);
  } else {
    url.searchParams.delete(key);
  }

  window.history.replaceState({}, "", url.toString());
}
