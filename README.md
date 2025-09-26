# 华为应用市场数据采集工具

这是一个用于从华为应用市场 API 获取应用信息并存储到 PostgreSQL 数据库的 Rust 工具。

## 🚀 功能特性

- ✅ 从华为应用市场 API 获取应用详细信息和评分数据
- ✅ 支持批量处理多个应用包和应用ID猜测
- ✅ 数据存储到 PostgreSQL 数据库，支持多表结构和视图查询
- ✅ 支持应用信息、指标数据、评分数据和原始 JSON 数据的存储
- ✅ 智能重复数据检测，避免重复插入相同数据
- ✅ 配置化管理数据库连接、API 参数和服务器设置
- ✅ 集成 tracing 日志系统和字符串清理工具
- ✅ 错误处理机制，单个包失败不影响整体流程
- ✅ Web 服务器提供 REST API 接口查询数据
- ✅ 多二进制工具支持数据同步、爬取和猜测应用ID
- ✅ API token 自动管理和刷新机制
- ✅ 数据库迁移脚本支持表结构演进

## 📋 数据库表结构

项目包含以下表结构：

### app_info - 应用基本信息表
- 应用ID、名称、包名、开发者信息
- 分类信息、描述、图标URL
- 应用属性（是否付费、是否上架等）
- created_at 时间戳跟踪记录创建

### app_metrics - 应用指标数据表
- 版本信息、应用大小、SHA256校验值
- 下载次数、价格、热度评分
- SDK版本信息、发布时间

### app_rating - 应用评分数据表
- average_rating 平均评分
- 星级计数（1-5星）
- 与 app_id 关联

### app_raw - 原始 JSON 数据表
- 完整的原始API响应数据（JSONB格式）
- 时间戳记录，便于数据追溯
- 去重机制避免重复存储

### app_latest_info - 视图
- 最新应用信息视图，聚合 app_info、app_metrics 和 app_rating
- 支持 Top 100 查询脚本

## 🛠️ 安装步骤

### 1. 前置要求

- Rust 1.70+ 和 Cargo
- PostgreSQL 12+ 数据库
- 网络连接（可访问华为应用市场API）

### 2. 克隆项目

```bash
git clone <repository-url>
cd get_market
```

### 3. 安装依赖

```bash
cargo build
```

### 4. 数据库配置

#### 创建数据库
```sql
CREATE DATABASE market_db;
```

#### 创建数据库用户（可选）
```sql
CREATE USER market_user WITH PASSWORD 'your_password';
GRANT ALL PRIVILEGES ON DATABASE market_db TO market_user;
```

#### 执行数据库迁移
```bash
# 执行主表结构脚本
psql -d market_db -f assets/sql/main.sql

# 应用迁移脚本（按顺序执行）
for dir in assets/sql/migrations/*; do
    psql -d market_db -f "$dir/up.sql"  # 假设每个迁移有 up.sql
done
```

### 5. 配置文件设置

复制示例配置文件并修改：

```bash
cp config.example.toml config.toml
```

编辑 `config.toml` 文件：

```toml
[database]
# PostgreSQL 数据库连接字符串
# 格式: postgresql://用户名:密码@主机:端口/数据库名
url = "postgresql://market_user:password@localhost:5432/market_db"

[app]
# 要查询的应用包名列表
packages = [
    "com.huawei.hmsapp.appgallery",  # 华为应用市场
    "com.tencent.mm",                # 微信
    "com.eg.android.AlipayGphone",   # 支付宝
    "com.chuckfang.meow"             # 示例应用
]
# 语言地区设置（所有包共用）
locale = "zh_CN"

[api]
# 华为应用市场 API 基础 URL
base_url = "https://web-drcn.hispace.dbankcloud.com/edge/webedge/appinfo"
# API 请求超时时间（秒）
timeout_seconds = 30

[logging]
# 日志级别: debug, info, warn, error
level = "info"

[server]
# Web 服务器配置
host = "0.0.0.0"
port = 8080
```

## 🎯 使用方法

### 基本运行

```bash
cargo run
```

### 使用命令行参数指定包名

```bash
# 处理单个应用包
cargo run com.example.app1

# 处理多个应用包
cargo run com.example.app1 com.example.app2 com.example.app3

# 直接运行编译后的可执行文件
./target/debug/get_market.exe com.huawei.app1 com.tencent.app2
```

### 构建发布版本

```bash
cargo build --release
./target/release/get_market
```

### 使用特定配置文件
```bash
cargo run -- --config custom_config.toml
```

### 启动 Web 服务器
```bash
# 启动服务器，提供 API 接口
cargo run --bin get_market server

# 或直接运行发布版本
./target/release/get_market server
```

服务器默认监听 http://localhost:8080，支持端点如：
- GET /query/pkg_name/{pkg} - 查询指定包名应用信息
- GET /query/app_id/{id} - 查询指定应用ID信息
- GET /top100 - 获取 Top 100 热门应用

### 其他二进制工具
```bash
# 猜测应用ID
cargo run --bin guess_market

# 从数据库猜测应用ID
cargo run --bin guess_from_db

# 从 nextmax.cn 爬取应用ID并保存为 apps.json
cargo run --bin get_nextmax

# 读取华为应用市场数据
cargo run --bin read_appgallery
```

## 📊 数据采集流程

1. **参数解析** - 解析命令行参数或加载配置文件中的包名
2. **初始化配置** - 加载配置文件，建立数据库连接
3. **API请求** - 对每个应用包发送POST请求到华为API
4. **数据解析** - 解析API返回的JSON数据
5. **重复检查** - 检查数据是否与最后一条记录相同
6. **数据存储** - 将解析后的数据插入到相应表中
7. **进度显示** - 实时显示处理进度和状态

## 🔧 配置说明

### 数据库配置
```toml
[database]
url = "postgresql://username:password@localhost:5432/market_db"
```

### 服务器配置
```toml
[server]
host = "0.0.0.0"
port = 8080
```

### 应用配置
```toml
[app]
packages = ["com.example.app1", "com.example.app2"]
locale = "zh_CN"  # 支持: zh_CN, en_US 等
```

### API 配置
```toml
[api]
base_url = "https://web-drcn.hispace.dbankcloud.com/edge/webedge/appinfo"
timeout_seconds = 30
```

### 日志配置
```toml
[logging]
level = "info"  # debug, info, warn, error
```

## 🗃️ 数据结构示例

### AppInfo 结构
```rust
pub struct AppInfo {
    pub app_id: String,           // 应用唯一ID
    pub name: String,             // 应用名称
    pub pkg_name: String,         // 包名
    pub developer_name: String,   // 开发者名称
    pub kind_name: String,        // 分类名称
    pub brief_desc: String,       // 简短描述
    // ... 其他字段
}
```

### AppMetric 结构
```rust
pub struct AppMetric {
    pub app_id: String,           // 应用ID
    pub version: String,          // 版本号
    pub size_bytes: i64,          // 应用大小
    pub download_count: i64,      // 下载次数
    pub hot_score: f64,           // 热度评分
    // ... 其他字段
}
```

## 🚨 故障排除

### CLI参数使用问题
- 命令行参数会覆盖配置文件中的包名设置
- 如果没有提供命令行参数，则使用配置文件中的包名
- 命令行参数支持多个包名，用空格分隔

### 数据库连接问题
- 检查 PostgreSQL 服务是否运行：`sudo systemctl status postgresql`
- 验证连接字符串格式是否正确
- 确认数据库用户有足够的权限

### API 访问问题
- 检查网络连接是否正常
- 验证 API URL 是否可访问
- 查看防火墙设置

### 编译问题
```bash
# 更新 Rust 工具链
rustup update

# 清理并重新构建
cargo clean
cargo build
```

## 📈 性能优化

- 使用 `--release` 标志构建以获得最佳性能
- 调整数据库连接池大小
- 合理设置请求超时时间
- 批量处理应用包时适当添加延迟

## 🤝 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

<!--## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情-->

## 🙏 致谢

- 华为应用市场提供的开放API
- Rust 生态系统和相关库开发者
- PostgreSQL 数据库团队

## 📞 支持

如果您遇到问题或有建议，请：
1. 查看现有的 [Issues](../../issues)
2. 创建新的 Issue 并描述详细问题
3. 提供相关的错误日志和配置信息

---

**注意**: 请确保遵守华为应用市场的使用条款和隐私政策，合理使用API接口。
