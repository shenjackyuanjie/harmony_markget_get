# 项目文件架构

## 📁 根目录
- Cargo.toml - Rust项目配置
- Cargo.lock - 依赖锁定
- config.toml - 主配置文件
- config.example.toml - 配置示例
- README.md - 详细文档
- runit.py - Python辅助脚本

## 📁 src/ - Rust源代码
- main.rs - 程序入口
- config.rs - 配置管理
- datas.rs - 数据结构定义
- db.rs - 数据库操作
- utils.rs - 工具函数
- sync.rs - 数据同步逻辑
- server.rs - Web服务器
- guess.rs - 应用ID猜测（独立二进制）

### 📁 src/sync/
- interface_code.rs - API token管理

## 📁 sql/ - 数据库
- main.sql - 表结构定义
- migrations/ - 迁移脚本

## 📁 target/ - 编译输出
- 编译生成的二进制文件

## 核心功能
- 华为应用市场API数据采集
- PostgreSQL数据存储（app_info, app_metrics, app_raw）
- Web服务提供REST API接口
- 配置化管理