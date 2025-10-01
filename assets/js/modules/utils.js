// 工具函数模块
var DashboardUtils = (function() {
    /**
     * 格式化数字，四位一分隔（中文习惯）
     * @param {number} num - 要格式化的数字
     * @returns {string} 格式化后的字符串
     */
    function formatNumber(num) {
        return num.toString().replace(/\B(?=(\d{4})+(?!\d))/g, ",");
    }

    /**
     * 格式化文件大小，自动选择合适的单位
     * @param {number} size - 文件大小（字节）
     * @returns {string} 格式化后的大小字符串
     */
    function formatSize(size) {
        if (size < 1024) return size + " B";
        if (size < 1024 * 1024) return (size / 1024).toFixed(2) + " KB";
        if (size < 1024 * 1024 * 1024)
            return (size / (1024 * 1024)).toFixed(2) + " MB";
        return (size / (1024 * 1024 * 1024)).toFixed(2) + " GB";
    }

    /**
     * 格式化日期时间，输出为YYYY-MM-DD HH:mm格式
     * @param {string|Date} dateInput - 日期时间字符串或Date对象
     * @returns {string} 格式化后的日期时间字符串
     */
    function formatDate(dateInput) {
        const date = new Date(dateInput);
        const pad = (n) => (n < 10 ? "0" + n : n);
        return (
            date.getFullYear() +
            "-" +
            pad(date.getMonth() + 1) +
            "-" +
            pad(date.getDate()) +
            " " +
            pad(date.getHours()) +
            ":" +
            pad(date.getMinutes())
        );
    }

    /**
     * 使用Unicode字符渲染星级评分显示
     * @param {number} rating - 评分值，范围0-5
     * @returns {string|null} 星级字符串，如果无评分返回null
     */
    function renderStars(rating) {
        if (rating === undefined || rating === null) return "无评分";
        const fullStars = Math.floor(rating);
        const hasHalf = rating % 1 >= 0.5;
        let stars = "";
        for (let i = 0; i < 5; i++) {
            if (i < fullStars) {
                stars += "★"; // 满星
            } else if (i === fullStars && hasHalf) {
                stars += "☆"; // 半星（简化表示，可替换为更好符号）
            } else {
                stars += "☆"; // 空星
            }
        }
        return stars + ` ${rating.toFixed(1)}`;
    }

    return {
        formatNumber: formatNumber,
        formatSize: formatSize,
        formatDate: formatDate,
        renderStars: renderStars
    };
})();
