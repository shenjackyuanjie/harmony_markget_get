# 华为应用市场数据采集工具

这是一个用于从华为应用市场 API 获取应用信息并存储到 PostgreSQL 数据库的工具。

## 功能特性

- 从华为应用市场 API 获取应用详细信息
- 将数据存储到 PostgreSQL 数据库
- 支持应用信息、权限、指标和原始 JSON 数据的存储
- 配置化管理数据库连接和 API 参数

## 数据库配置

### 1. 创建数据库

首先需要创建一个 PostgreSQL 数据库：

```sql
CREATE DATABASE market_db;
```

### 2. 配置连接字符串

编辑 `config.toml` 文件，设置正确的数据库连接信息：

```toml
[database]
url = "postgresql://username:password@localhost:5432/market_db"
```

连接字符串格式：`postgresql://用户名:密码@主机:端口/数据库名`

### 3. 数据库表结构

项目包含以下表结构：

- `app_info` - 应用基本信息表
- `app_permissions` - 应用权限信息表  
- `app_metrics` - 应用指标数据表
- `app_raw` - 原始 JSON 数据表

表结构定义在 `sql/main.sql` 中。

### 4. 运行数据库迁移

手动执行 SQL 文件创建表：

```bash
psql -d market_db -f sql/main.sql
```

## 安装依赖

```bash
cargo build
```

## 使用方法

1. 配置 `config.toml` 文件
2. 确保 PostgreSQL 数据库运行并可访问
3. 运行程序：

```bash
cargo run
```

## 配置说明

### 数据库配置
```toml
[database]
url = "postgresql://username:password@localhost:5432/market_db"
```

### 应用配置
```toml
[app]
default_package = "com.huawei.hmsapp.appgallery"  # 默认查询的应用包名
locale = "zh_CN"                                  # 语言地区
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
level = "info"
```

## 数据结构

程序会将华为 API 返回的数据解析为以下结构：

1. **AppInfo** - 应用基本信息
2. **AppPermission** - 应用权限信息  
3. **AppMetric** - 应用指标数据
4. **RawJsonData** - 原始 API 响应数据

## 注意事项

1. 确保 PostgreSQL 数据库服务正在运行
2. 数据库用户需要有创建表和插入数据的权限
3. 网络需要能够访问华为应用市场 API
4. 程序使用 ON CONFLICT 更新机制，重复运行会更新现有数据

## 故障排除

如果遇到数据库连接问题，检查：
- 数据库服务是否运行
- 连接字符串是否正确
- 数据库用户权限是否足够

如果遇到 API 访问问题，检查：
- 网络连接是否正常
- API URL 是否可访问