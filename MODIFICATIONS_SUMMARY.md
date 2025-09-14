# 数据库和代码修改总结

## 修改概述

本次修改主要完成了以下任务：
1. 重命名 `app_metrics` 表中的 `hot_score` 和 `rate_num` 字段
2. 添加了页面评分相关的数据字段
3. 修改了 `app_raw` 表结构以支持两个JSON字段
4. 更新了相应的Rust代码

## 具体修改内容

### 1. 数据库表结构修改

#### app_metrics 表
- **重命名字段**:
  - `hot_score` → `info_score` (NUMERIC(3,1))
  - `rate_num` → `info_rate_count` (BIGINT)

- **新增页面评分字段**:
  - `page_average_rating` (NUMERIC(3,1)) - 页面平均评分
  - `page_star_1_rating_count` (INTEGER) - 页面1星评分数量
  - `page_star_2_rating_count` (INTEGER) - 页面2星评分数量
  - `page_star_3_rating_count` (INTEGER) - 页面3星评分数量
  - `page_star_4_rating_count` (INTEGER) - 页面4星评分数量
  - `page_star_5_rating_count` (INTEGER) - 页面5星评分数量
  - `page_my_star_rating` (INTEGER) - 页面我的评分
  - `page_total_star_rating_count` (INTEGER) - 页面总评分数量
  - `page_only_star_count` (INTEGER) - 页面仅评分数量
  - `page_full_average_rating` (NUMERIC(3,1)) - 页面完整平均评分
  - `page_source_type` (TEXT) - 页面评分来源类型

#### app_raw 表
- **结构调整**:
  - 删除 `raw_json` 字段
  - 新增 `raw_json_data` (JSONB) - 原始应用数据JSON
  - 新增 `raw_json_star` (JSONB) - 原始评分数据JSON

### 2. Rust代码修改

#### datas.rs
- **AppMetric 结构体**:
  - 重命名字段: `hot_score` → `info_score`, `rate_num` → `info_rate_count`
  - 新增页面评分相关字段
  - 添加 `from_raw_data_and_star` 方法，支持从 `RawJsonData` 和 `RawStarData` 创建 `AppMetric`

#### db.rs
- **insert_app_metric 方法**:
  - 更新SQL查询语句以使用新字段名
  - 添加页面评分字段的绑定

- **insert_raw_data 方法**:
  - 修改为插入两个JSON字段: `raw_json_data` 和 `raw_json_star`

- **get_last_raw_json 方法**:
  - 修改为查询 `raw_json_star` 字段

- **save_app_data 方法**:
  - 使用新的 `from_raw_data_and_star` 方法创建 `AppMetric`

### 3. 迁移脚本

创建了两个迁移脚本：

#### 001_rename_metrics_fields_and_add_page_ratings.sql
- 重命名 `app_metrics` 表中的字段
- 添加页面评分相关字段
- 更新视图 `app_latest_info`

#### 002_modify_app_raw_table.sql
- 修改 `app_raw` 表结构
- 添加 `raw_json_data` 和 `raw_json_star` 字段
- 迁移现有数据
- 删除旧的 `raw_json` 字段

## 数据流变化

### 修改前:
```
RawJsonData → AppMetric (仅包含应用数据)
RawStarData → AppRaw.raw_json_star (仅存储在原始数据中)
```

### 修改后:
```
RawJsonData + RawStarData → AppMetric (包含应用数据和页面评分数据)
RawJsonData → AppRaw.raw_json_data
RawStarData → AppRaw.raw_json_star
```

## 影响范围

1. **数据库层面**: 需要执行迁移脚本更新表结构
2. **代码层面**: 所有使用 `AppMetric` 结构体的地方都需要更新字段名
3. **数据存储**: 页面评分数据现在会持久化到数据库中，便于查询和分析

## 迁移步骤

1. 执行 `001_rename_metrics_fields_and_add_page_ratings.sql`
2. 执行 `002_modify_app_raw_table.sql`
3. 部署更新后的Rust代码

## 注意事项

- 迁移脚本需要按顺序执行
- 现有数据会自动迁移到新字段
- 代码更新后需要重新编译
- 确保数据库备份后再执行迁移操作