// 事件监听器模块
import { parseLink, validateInputs, queryApp, clearForm } from './app-query';
import { updateUrlParam } from '../utils/url-params';

export function setupEventListeners(): void {
  // ESC键关闭详情弹窗
  document.addEventListener("keydown", function (event) {
    if (event.key === "Escape") {
      const modalsToClose = [
        "appDetailModal",
        "helpModal",
        "submitModal",
        "contactModal",
        "searchHelpModal",
      ];
      modalsToClose.forEach((modalId) => {
        const modal = document.getElementById(modalId);
        if (modal && !modal.classList.contains("hidden")) {
          modal.classList.add("hidden");

          // 如果关闭的是应用详情模态框，移除URL参数
          if (modalId === "appDetailModal") {
            updateUrlParam("app_id", "");
            updateUrlParam("pkg_name", "");
            (window as any).currentApp = null;
          }
        }
      });
    }
  });

  // 清空按钮
  const clearBtn = document.getElementById("clearBtn");
  if (clearBtn) {
    clearBtn.addEventListener("click", clearForm);
  }

  // 投稿按钮
  const submitBtn = document.getElementById("submitBtn");
  if (submitBtn) {
    submitBtn.addEventListener("click", () => {
      const submitModal = document.getElementById("submitModal");
      if (submitModal) {
        submitModal.classList.remove("hidden");
        // 清空输入和错误
        clearForm();
      }
    });
  }

  // 包名输入事件，隐藏错误
  const pkgInput = document.getElementById("pkgInput");
  if (pkgInput) {
    pkgInput.addEventListener("input", () => {
      const pkgError = document.getElementById("pkgError");
      if (pkgError) {
        pkgError.classList.add("hidden");
      }
    });
  }

  // app_id 输入事件，隐藏错误
  const appIdInput = document.getElementById("appIdInput");
  if (appIdInput) {
    appIdInput.addEventListener("input", () => {
      const appIdError = document.getElementById("appIdError");
      if (appIdError) {
        appIdError.classList.add("hidden");
      }
    });
  }

  // 链接输入事件
  const linkInput = document.getElementById("linkInput");
  if (linkInput) {
    linkInput.addEventListener("input", parseLink);
  }

  // 查询按钮
  const queryBtn = document.getElementById("queryBtn");
  if (queryBtn) {
    queryBtn.addEventListener("click", () => {
      if (validateInputs()) {
        queryApp();
      }
    });
  }

  // 清除用户名存储的按钮事件
  const clearUsernameStorage = document.getElementById("clearUsernameStorage");
  if (clearUsernameStorage) {
    clearUsernameStorage.addEventListener("click", (e) => {
      e.stopPropagation(); // 防止事件冒泡
      localStorage.removeItem("submitUsername");
      const usernameInput = document.getElementById(
        "usernameInput",
      ) as HTMLInputElement;
      if (usernameInput) {
        usernameInput.value = "";
      }
      alert("已清除记忆的用户名");
    });
  }

  // 模态框事件处理器
  const modalHandlers = [
    { id: 'helpBtn', modal: 'helpModal', action: 'show' },
    { id: 'closeHelpModal', modal: 'helpModal', action: 'hide' },
    { id: 'closeHelpModalBtn', modal: 'helpModal', action: 'hide' },
    { id: 'searchHelpBtn', modal: 'searchHelpModal', action: 'show' },
    { id: 'closeSearchHelpModal', modal: 'searchHelpModal', action: 'hide' },
    { id: 'closeSearchHelpBtn', modal: 'searchHelpModal', action: 'hide' },
    { id: 'contactBtn', modal: 'contactModal', action: 'show' },
    { id: 'closeContactModal', modal: 'contactModal', action: 'hide' },
    { id: 'closeContactModalBtn', modal: 'contactModal', action: 'hide' },
    { id: 'closeSubmitModal', modal: 'submitModal', action: 'hide' }
  ];

  modalHandlers.forEach(handler => {
    const element = document.getElementById(handler.id);
    const modal = document.getElementById(handler.modal);

    if (element && modal) {
      element.addEventListener('click', () => {
        if (handler.action === 'show') {
          modal.classList.remove('hidden');
        } else {
          modal.classList.add('hidden');

          // 如果关闭的是应用详情模态框，移除URL参数
          if (handler.modal === 'appDetailModal') {
            updateUrlParam('app_id', '');
            updateUrlParam('pkg_name', '');
            (window as any).currentApp = null;
          }
        }
      });
    }
  });

  // 为应用详情的关闭按钮添加事件，移除URL参数
  document.querySelectorAll("button[onclick]").forEach(btn => {
    const originalOnClick = btn.getAttribute('onclick');
    if (originalOnClick && originalOnClick.includes("appDetailModal") && originalOnClick.includes("classList.add('hidden')")) {
      btn.setAttribute('onclick', `${originalOnClick}; window.updateUrlParam('app_id', ''); window.updateUrlParam('pkg_name', ''); window.currentApp = null;`);
    }
  });
}