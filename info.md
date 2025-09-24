# 项目文件架构

## 📁 根目录
- `Cargo.toml` - Rust项目配置（workspace配置）
- `Cargo.lock` - 依赖锁定文件
- `config.toml` - 主配置文件
- `config.example.toml` - 配置示例文件
- `README.md` - 详细项目文档
- `API.md` - API接口文档
- `runit.py` - Python辅助脚本
- `.gitignore` - Git忽略文件配置

## 📁 src/ - 主项目Rust源代码
- `main.rs` - 程序主入口
- `lib.rs` - 库模块定义
- `config.rs` - 配置管理模块
- `datas.rs` - 数据结构定义
- `db.rs` - 数据库操作模块
- `utils.rs` - 工具函数模块
- `sync.rs` - 数据同步逻辑
- `server.rs` - Web服务器模块
- `guess.rs` - 应用ID猜测（独立二进制）
- `guess_from_db.rs` - 从数据库猜测应用ID（独立二进制）
- `get_nextmax.rs` - 获取下一个最大ID（独立二进制）

### 📁 src/sync/ - 同步相关模块
- `interface_code.rs` - API token管理
- `identy_id.rs` - 身份ID管理

## 📁 read_appgallery/ - 读取应用市场子项目
- `Cargo.toml` - 子项目配置
- `src/` - 子项目源代码目录

## 📁 assets/ - 静态资源文件
### 📁 assets/sql/ - 数据库脚本
- `main.sql` - 主表结构定义
- `top100.sql` - Top 100应用相关脚本
- `migrations/` - 数据库迁移脚本目录

### 📁 assets/html/ - HTML模板文件
- `main.html` - 主页面模板

## 📁 target/ - 编译输出目录
- 编译生成的二进制文件和缓存

## 核心功能
- **多二进制项目**：包含主程序(`get_market`)和多个独立工具(`guess_market`, `guess_from_db`, `get_nextmax`)
- **Workspace管理**：使用Cargo workspace管理主项目和子项目
- **华为应用市场API数据采集**：通过HTTP请求获取应用数据
- **PostgreSQL数据存储**：支持app_info, app_metrics, app_raw等表结构
- **Web服务**：提供REST API接口访问采集的数据
- **配置化管理**：通过TOML文件进行灵活配置
- **数据同步**：支持API token管理和身份验证
- **应用ID猜测**：提供多种方式猜测和验证应用ID

## 项目特点
- 采用Rust 2024 Edition
- 使用Tokio异步运行时
- 基于Axum框架的Web服务
- SQLx数据库操作
- 模块化设计，便于扩展和维护