# 迁移顺序文档：006_add_comment_to_app_info

## 迁移概述
本迁移为 app_info 表添加 comment 列，类型为 JSONB，允许为 NULL，同时更新 app_latest_info 视图以包含新增字段。

## 执行顺序
1. **001_add_comment_to_app_info.sql** - 主迁移脚本

## 迁移详情

### 影响表
- `app_info` - 应用信息表
- `app_latest_info` - 应用最新信息视图（更新）

### 具体变更

#### app_info 表
- 添加新列 `comment` → JSONB (允许为NULL)
- 定义：用于存储应用相关的评论或注释数据，使用JSON格式以支持复杂的评论结构

#### app_latest_info 视图
- 更新视图定义，包含新增的 comment 字段

## 注意事项
1. 该字段允许为NULL，因此不会影响现有数据
2. 对现有记录，comment字段将默认为NULL
3. 建议在执行前备份数据库
4. 迁移执行后，应更新main.sql文件以反映新的表结构

## 执行命令
```sql
\i sql/migrations/006_add_comment_to_app_info/001_add_comment_to_app_info.sql
```

## 验证
迁移完成后，可以通过以下方式验证：
1. 检查app_info表结构是否包含comment列
2. 确认app_latest_info视图能正常显示comment字段
3. 测试插入和查询操作，确保comment字段可以正确存储和检索JSON数据