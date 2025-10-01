# 项目文件架构 (华为应用市场数据采集工具，使用Rust构建，支持API查询和数据库存储)

## 📁 根目录
- `Cargo.toml` - Rust项目配置（workspace配置，包含8个二进制程序）
- `Cargo.lock` - 依赖锁定文件
- `config.toml` - 主配置文件
- `config.example.toml` - 配置示例文件 (数据库、API和应用包配置模板)
- `README.md` - 详细项目文档 (安装、运行和数据库表结构说明)
- `API.md` - API接口文档 (REST端点如/query/pkg_name/{pkg}用于应用信息查询)
- `info.md` - 项目结构文档 (当前文件，描述目录和文件功能)
- `runit.py` - Python定时运行脚本 (每小时执行 cargo run --release 以自动化数据采集和更新)
- `.gitignore` - Git忽略文件配置 (忽略target/、.env等)
- `.idea/` - IDE配置文件目录
- `.ruff_cache/` - Python代码检查缓存
- `.venv/` - Python虚拟环境目录
- `target/` - Rust编译输出目录

## 📁 src/ - 主项目Rust源代码
- `main.rs` - 程序主入口 (`get_market`)
- `config.rs` - 配置管理模块
- `utils.rs` - 工具函数模块 (日志初始化、UTF-8字符串清理和无效字符处理)

### 📁 src/db/ - 数据库操作模块
- `mod.rs` - 模块定义 (数据库连接池管理和操作入口)
- `insert.rs` - 数据插入实现 (应用信息、指标和原始数据的存储)
- `query.rs` - 数据查询实现 (应用信息检索和视图查询)

### 📁 src/server/ - Web服务器模块
- `mod.rs` - 模块定义 (服务器启动和任务管理)
- `handlers.rs` - 请求处理函数 (API端点逻辑实现)
- `routes.rs` - API路由配置 (定义查询路径和参数处理)
- `state.rs` - 应用状态管理 (共享配置和数据库连接状态)

### 📁 src/sync/ - 同步相关模块
- `mod.rs` - 模块定义 (同步API调用，如get_app_info获取应用数据、get_star_by_app_id获取评分)
- `code.rs` - API token管理 (全局CodeManager处理identity_id和interface_code刷新，每10分钟更新)

### 📁 src/model/ - 模型相关模块
- `mod.rs` - 模块定义 (导出AppQuery、RawJsonData和AppInfo等结构)
- `query.rs` - 查询结构定义 (AppQuery枚举支持pkg_name和app_id查询类型)
- `raw.rs` - 原始数据模型 (RawJsonData和RawRatingData从API JSON反序列化，AppInfo/Metric/Rating转换)

### 📁 独立二进制程序
- `guess_market.rs` - 应用ID猜测（独立二进制 `guess_market`）
- `guess_rand.rs` - 随机应用ID猜测（独立二进制 `guess_rand`）
- `guess_from_db.rs` - 从数据库猜测应用ID（独立二进制 `guess_from_db`）
- `guess_large.rs` - 大规模应用ID猜测（独立二进制 `guess_large`）
- `get_nextmax.rs` - 从nextmax.cn爬取华为应用市场应用ID并保存为apps.json（独立二进制 `get_nextmax`）
- `read_appgallery.rs` - 读取华为应用市场数据（独立二进制 `read_appgallery`）
- `read_pkg_name.rs` - 读取包名数据（独立二进制 `read_pkg_name`）

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
- `main.html` - 主页面模板 (Web UI基础模板)

### 📁 assets/js/ - JavaScript前端资源
- `dashboard.js` - 仪表板JavaScript逻辑
- `chart.js` - 图表库
- `chartjs-adapter-date-fns.js` - Chart.js日期适配器
- `chartjs-plugin-datalabels.js` - Chart.js数据标签插件
- `modules/` - JavaScript模块目录

### 📁 assets/icon/ - 图标资源文件
- `favicon.ico` - 网站 Favicon 图标

## 📁 import_data/ - 数据导入相关
- `gen.py` - 数据生成Python脚本
- `gen.sql` - 数据生成SQL脚本
- `what.csv` - 示例数据CSV文件

## 📁 scripts/ - 辅助脚本目录
- `guess.py` - Python应用ID猜测脚本
- `guess_pkg_name.py` - Python包名猜测脚本

## 核心功能
- **多二进制项目**：包含主程序(`get_market`)和7个独立工具(`guess_market`, `guess_rand`, `guess_from_db`, `guess_large`, `get_nextmax`, `read_appgallery`, `read_pkg_name`)
- **项目管理**：使用Cargo.toml配置多二进制和依赖，支持`no_sync`和`no_db_sync`特性
- **华为应用市场API数据采集**：通过HTTP请求获取应用数据，支持身份验证和token管理
- **PostgreSQL数据存储**：支持app_info, app_metrics, app_raw, app_rating等表结构，包含完整的数据模型转换
- **Web服务**：提供REST API接口访问采集的数据，支持分页查询和多种查询条件
- **配置化管理**：通过TOML文件进行灵活配置，支持数据库连接池配置
- **数据同步**：支持API token管理和身份验证，每10分钟自动刷新
- **应用ID猜测**：提供多种方式猜测和验证应用ID，包括随机猜测、数据库辅助和大规模猜测
- **工具支持**：集成tracing日志系统和字符串清理工具，确保数据完整性
- **自动化运行**：通过runit.py脚本实现定时任务，支持持续数据同步
- **前端支持**：包含HTML模板和JavaScript资源，支持数据可视化

## 技术栈
- **后端**: Rust (Tokio异步运行时, Axum Web框架, SQLx数据库操作)
- **数据库**: PostgreSQL
- **前端**: HTML + JavaScript + Chart.js
- **配置**: TOML
- **工具**: Python辅助脚本

## 数据模型
- `AppInfo`: 应用基本信息
- `AppMetric`: 应用指标数据
- `AppRating`: 应用评分数据
- `AppRaw`: 原始JSON数据存储
- `FullAppInfo`: 完整应用信息组合
- `ShortAppInfo`: 简化版应用信息
- `ShortAppRating`: 简化版评分排行