// 全局类型声明
declare global {
  interface Window {
    updateUrlParam: (key: string, value: string) => void;
    getUrlParams: () => Record<string, string>;
    copyToClipboard: (text: string, button?: HTMLElement) => void;
    currentApp: { appId?: string; pkgName?: string } | null;
  }
}

// 全局变量定义
export let currentPage: number = 1;
export let totalPages: number = 1;
export let currentApp: { appId?: string; pkgName?: string } = {};
export let currentSort: { field: string; desc: boolean } = {
  field: "download_count",
  desc: true,
};
export let searchTerm: string = "";
export let searchKey: string = "name";
export let searchExact: boolean = false;

// 常量定义
export const PAGE_SIZE: number = 20;
export const API_BASE: string = "/api";