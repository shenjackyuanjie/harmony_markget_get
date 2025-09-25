# 项目文件架构 (华为应用市场数据采集工具，使用Rust构建，支持API查询和数据库存储)

## 📁 根目录
- `Cargo.toml` - Rust项目配置（workspace配置）
- `Cargo.lock` - 依赖锁定文件
- `config.toml` - 主配置文件
- `config.example.toml` - 配置示例文件 (数据库、API和应用包配置模板)
- `README.md` - 详细项目文档 (安装、运行和数据库表结构说明)
- `API.md` - API接口文档 (REST端点如/query/pkg_name/{pkg}用于应用信息查询)
- `info.md` - 项目结构文档 (当前文件，描述目录和文件功能)
- `runit.py` - Python定时运行脚本 (每小时执行 cargo run --release 以自动化数据采集和更新)
- `.gitignore` - Git忽略文件配置 (忽略target/、.env等)

## 📁 src/ - 主项目Rust源代码
- `main.rs` - 程序主入口 (`get_market`)
- `config.rs` - 配置管理模块
### 📁 src/db/ - 数据库操作模块
- `mod.rs` - 模块定义 (数据库连接池管理和操作入口)
- `insert.rs` - 数据插入实现 (应用信息、指标和原始数据的存储)
- `query.rs` - 数据查询实现 (应用信息检索和视图查询)
- `utils.rs` - 工具函数模块 (日志初始化、UTF-8字符串清理和无效字符处理)
### 📁 src/server/ - Web服务器模块
- `mod.rs` - 模块定义 (服务器启动和任务管理)
- `handlers.rs` - 请求处理函数 (API端点逻辑实现)
- `routes.rs` - API路由配置 (定义查询路径和参数处理)
- `state.rs` - 应用状态管理 (共享配置和数据库连接状态)

- `guess.rs` - 应用ID猜测（独立二进制 `guess_market`）
- `guess_from_db.rs` - 从数据库猜测应用ID（独立二进制 `guess_from_db`）
- `get_nextmax.rs` - 从nextmax.cn爬取华为应用市场应用ID并保存为apps.json（独立二进制 `get_nextmax`）
- `read_appgallery.rs` - 读取华为应用市场数据（独立二进制 `read_appgallery`）

### 📁 src/sync/ - 同步相关模块
- `code.rs` - API token管理 (全局CodeManager处理identity_id和interface_code刷新，每10分钟更新)
- `mod.rs` - 模块定义 (同步API调用，如get_app_info获取应用数据、get_star_by_app_id获取评分)

### 📁 src/model/ - 模型相关模块
- `mod.rs` - 模块定义 (导出AppQuery、RawJsonData和AppInfo等结构)
- `query.rs` - 查询结构定义 (AppQuery枚举支持pkg_name和app_id查询类型)
- `raw.rs` - 原始数据模型 (RawJsonData和RawRatingData从API JSON反序列化，AppInfo/Metric/Rating转换)

## 📁 assets/ - 静态资源文件
### 📁 assets/sql/ - 数据库脚本
- `main.sql` - 主表结构定义 (创建app_info基本信息、app_metrics指标、app_raw原始JSON、app_rating评分表和app_latest_info视图，加上索引)
- `top100.sql` - Top 100应用相关脚本 (从app_latest_info按download_count降序查询前100应用)
- `migrations/` - 数据库迁移脚本目录
 - `001_add_star/` - 添加星级相关迁移 (引入app_rating表存储average_rating和星级计数)
 - `002_split_tables/` - 分割表结构迁移 (分离app_metrics和app_rating以优化存储)
 - `003_remove_duplicate_app_raw/` - 移除重复 app_raw 记录迁移 (清理多余的原始JSON数据)
 - `004_add_created_at_to_app_info/` - 为 app_info 添加 created_at 字段迁移 (添加时间戳跟踪记录创建)
 - `005_set_not_null_constraints/` - 设置非空约束迁移 (强化app_id、name等关键字段的NOT NULL)

### 📁 assets/html/ - HTML模板文件
- `main.html` - 主页面模板 (未使用，可能为Web UI基础模板)

## 核心功能
- **多二进制项目**：包含主程序(`get_market`)和多个独立工具(`guess_market`, `guess_from_db`, `get_nextmax`, `read_appgallery`)
- **项目管理**：使用Cargo.toml配置多二进制和依赖
- **华为应用市场API数据采集**：通过HTTP请求获取应用数据
- **PostgreSQL数据存储**：支持app_info, app_metrics, app_raw等表结构
- **Web服务**：提供REST API接口访问采集的数据
- **配置化管理**：通过TOML文件进行灵活配置
- **数据同步**：支持API token管理和身份验证
- **应用ID猜测**：提供多种方式猜测和验证应用ID
- **工具支持**：集成tracing日志系统和字符串清理工具，确保数据完整性
- **自动化运行**：通过runit.py脚本实现定时任务，支持持续数据同步
