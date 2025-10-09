// 剪贴板工具
export function copyToClipboard(text: string, button?: HTMLElement): void {
  if (!text) {
    showCopyError(button);
    return;
  }

  // 使用新的 Clipboard API
  if (navigator.clipboard && window.isSecureContext) {
    navigator.clipboard
      .writeText(text)
      .then(() => {
        showCopySuccess(button);
      })
      .catch(() => {
        // 如果新的API失败，回退到旧的execCommand方法
        fallbackCopyTextToClipboard(text, button);
      });
  } else {
    // 使用旧的execCommand方法
    fallbackCopyTextToClipboard(text, button);
  }
}

function fallbackCopyTextToClipboard(text: string, button?: HTMLElement): void {
  const textArea = document.createElement("textarea");
  textArea.value = text;

  // 避免滚动到视图中
  textArea.style.top = "0";
  textArea.style.left = "0";
  textArea.style.position = "fixed";

  document.body.appendChild(textArea);
  textArea.focus();
  textArea.select();

  try {
    const successful = document.execCommand("copy");
    if (successful) {
      showCopySuccess(button);
    } else {
      showCopyError(button);
    }
  } catch (err) {
    showCopyError(button);
  }

  document.body.removeChild(textArea);
}

function showCopySuccess(button?: HTMLElement): void {
  if (button) {
    const originalHTML = button.innerHTML;

    // 临时更改按钮文本和样式
    button.innerHTML = '<i class="fas fa-check"></i> 已复制';
    button.classList.add("bg-green-500", "hover:bg-green-600");
    button.classList.remove("bg-blue-500", "hover:bg-blue-600");

    // 3秒后恢复原始状态
    setTimeout(() => {
      button.innerHTML = originalHTML;
      button.classList.remove("bg-green-500", "hover:bg-green-600");
      button.classList.add("bg-blue-500", "hover:bg-blue-600");
    }, 3000);
  } else {
    // 如果没有提供按钮，显示一个简单的提示
    const notification = document.createElement("div");
    notification.textContent = "已复制到剪贴板";
    notification.className =
      "fixed top-4 right-4 bg-green-500 text-white px-4 py-2 rounded-lg shadow-lg z-50";

    document.body.appendChild(notification);

    setTimeout(() => {
      document.body.removeChild(notification);
    }, 3000);
  }
}

function showCopyError(button?: HTMLElement): void {
  if (button) {
    const originalHTML = button.innerHTML;

    // 临时更改按钮文本和样式
    button.innerHTML = '<i class="fas fa-times"></i> 复制失败';
    button.classList.add("bg-red-500", "hover:bg-red-600");
    button.classList.remove("bg-blue-500", "hover:bg-blue-600");

    // 3秒后恢复原始状态
    setTimeout(() => {
      button.innerHTML = originalHTML;
      button.classList.remove("bg-red-500", "hover:bg-red-600");
      button.classList.add("bg-blue-500", "hover:bg-blue-600");
    }, 3000);
  } else {
    // 如果没有提供按钮，显示一个简单的提示
    const notification = document.createElement("div");
    notification.textContent = "复制失败，请手动复制";
    notification.className =
      "fixed top-4 right-4 bg-red-500 text-white px-4 py-2 rounded-lg shadow-lg z-50";

    document.body.appendChild(notification);

    setTimeout(() => {
      document.body.removeChild(notification);
    }, 3000);
  }
}
