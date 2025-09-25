# 华为应用市场数据采集API文档

## 概述

这是一个基于Rust和Axum框架构建的华为应用市场数据采集服务API。该服务提供应用信息查询、列表浏览和排行榜功能，支持通过包名或应用ID获取应用的详细信息、指标数据和评分数据。

## 基础信息

- **服务地址**: `http://{host}:{port}`
- **默认端口**: 3000
- **协议**: HTTP
- **数据格式**: JSON

## API端点

所有端点返回 JSON 格式响应。成功响应结构：
```json
{
  "data": {实际数据},
  "total_count": 数字 (可选，分页或排行时),
  "page_size": 数字 (可选，分页或排行时)
}
```

错误响应：
```json
{
  "error": "错误描述"
}
```

### 1. 查询应用信息（按包名）

查询指定包名的应用详细信息，包括基本信息、指标数据和评分数据。

**端点**: `GET /api/apps/by-pkg-name/{pkg_name}`

**路径参数**:
- `pkg_name` (string, required): 应用包名，如 `com.huawei.hmsapp.appgallery`

**响应示例**:
```json
{
  "data": {
    "info": {AppInfo对象},
    "metric": {AppMetric对象},
    "rating": {AppRating对象},
    "is_new": true
  }
}
```

### 2. 查询应用信息（按应用ID）

查询指定应用ID的详细信息。

**端点**: `GET /api/apps/by-app-id/{app_id}` 或 `GET /api/apps/{app_id}`

**路径参数**:
- `app_id` (string, required): 应用ID，如 `C1164531384803416384`

**响应示例**: 同上，按包名查询。

### 3. 获取应用列表统计信息

获取应用总数和原子服务总数。

**端点**: `GET /api/apps/list/info`

**响应示例**:
```json
{
  "data": {
    "app_count": 12345,
    "atomic_services_count": 6789
  }
}
```

### 4. 分页获取应用列表（简略信息）

分页获取应用的基本信息。

**端点**: `GET /api/apps/list/{page_count}`

**路径参数**:
- `page_count` (string, required): 页码，如 `1`

**响应示例**:
```json
{
  "data": {
    "apps": [{简略AppInfo数组}],
    "total_count": 12345,
    "page_size": 100
  }
}
```

### 5. 分页获取应用列表（详细信息）

分页获取应用的完整信息，包括指标和评分。

**端点**: `GET /api/apps/list/{page_count}/detail`

**路径参数**:
- `page_count` (string, required): 页码，如 `1`

**响应示例**:
```json
{
  "data": {
    "apps": [{完整AppInfo, Metric, Rating数组}],
    "total_count": 12345,
    "page_size": 100
  }
}
```

### 6. 排行榜API

所有排行榜支持查询参数 `?limit=N` (默认10) 和部分支持 `?time_range=7d` (如增长排行)。

#### 6.1 下载量排行
**端点**: `GET /api/rankings/downloads`

**响应**: Top apps by download_count。

#### 6.2 评分排行
**端点**: `GET /api/rankings/ratings`

**响应**: Top apps by average_rating。

#### 6.3 最近更新排行
**端点**: `GET /api/rankings/recent`

**响应**: Recently updated apps。

#### 6.4 价格排行
**端点**: `GET /api/rankings/prices`

**响应**: Top priced apps。

#### 6.5 评分人数排行
**端点**: `GET /api/rankings/rating-counts`

**响应**: Top apps by rating count。

#### 6.6 下载量增长排行
**端点**: `GET /api/rankings/download-growth?time_range=7d`

**响应**: Apps with download growth in time range。

#### 6.7 评分增长排行
**端点**: `GET /api/rankings/rating-growth?time_range=7d`

**响应**: Apps with rating growth。

#### 6.8 开发者排行
**端点**: `GET /api/rankings/developers`

**响应**: Top developers by app count or metrics。

#### 6.9 应用大小排行
**端点**: `GET /api/rankings/sizes`

**响应**: Largest apps by size。

**排行响应示例** (通用):
```json
{
  "data": [{AppInfo with metrics数组}],
  "total_count": 10,
  "page_size": 10
}
```

## 数据类型说明

### AppInfo (应用基本信息)

| 字段 | 类型 | 描述 |
|------|------|------|
| app_id | string | 应用唯一ID |
| alliance_app_id | string | 联盟应用ID |
| name | string | 应用名称 |
| pkg_name | string | 应用包名 |
| dev_id | string | 开发者ID |
| developer_name | string | 开发者名称 |
| dev_en_name | string | 开发者英文名称 |
| supplier | string | 供应商名称 |
| kind_id | integer | 应用分类ID |
| kind_name | string | 应用分类名称 |
| tag_name | string | 标签名称 |
| kind_type_id | integer | 类型ID |
| kind_type_name | string | 类型名称 |
| icon_url | string | 应用图标URL |
| brief_desc | string | 简短描述 |
| description | string | 应用详细描述 |
| privacy_url | string | 隐私政策链接 |
| ctype | integer | 客户端类型 |
| detail_id | string | 详情页ID |
| app_level | integer | 应用等级 |
| jocat_id | integer | 分类ID |
| iap | boolean | 是否含应用内购买 |
| hms | boolean | 是否依赖HMS |
| tariff_type | string | 资费类型 |
| packing_type | integer | 打包类型 |
| order_app | boolean | 是否预装应用 |
| denpend_gms | boolean | 是否依赖GMS |
| denpend_hms | boolean | 是否依赖HMS |
| force_update | boolean | 是否强制更新 |
| img_tag | string | 图片标签 |
| is_pay | boolean | 是否付费 |
| is_disciplined | boolean | 是否合规 |
| is_shelves | boolean | 是否上架 |
| submit_type | integer | 提交类型 |
| delete_archive | boolean | 是否删除归档 |
| charging | boolean | 是否收费 |
| button_grey | boolean | 按钮是否置灰 |
| app_gift | boolean | 是否有礼包 |
| free_days | integer | 免费天数 |
| pay_install_type | integer | 付费安装类型 |

### AppMetric (应用指标数据)

| 字段 | 类型 | 描述 |
|------|------|------|
| id | integer | 主键ID |
| app_id | string | 应用ID |
| version | string | 版本号 |
| version_code | integer | 版本代码 |
| size_bytes | integer | 应用大小(字节) |
| sha256 | string | 安装包SHA256校验值 |
| info_score | number | 信息评分 |
| info_rate_count | integer | 信息评分人数 |
| download_count | integer | 下载次数 |
| price | string | 价格 |
| release_date | integer | 发布时间(时间戳毫秒) |
| new_features | string | 新功能描述 |
| upgrade_msg | string | 升级提示 |
| target_sdk | integer | 目标SDK版本 |
| minsdk | integer | 最低SDK版本 |
| compile_sdk_version | integer | 编译SDK版本 |
| min_hmos_api_level | integer | 最低鸿蒙API等级 |
| api_release_type | string | API发布类型 |
| created_at | string | 创建时间(ISO 8601) |

### AppRating (应用评分数据)

| 字段 | 类型 | 描述 |
|------|------|------|
| id | integer | 主键ID |
| app_id | string | 应用ID |
| average_rating | number | 平均评分 |
| star_1_rating_count | integer | 1星评分数量 |
| star_2_rating_count | integer | 2星评分数量 |
| star_3_rating_count | integer | 3星评分数量 |
| star_4_rating_count | integer | 4星评分数量 |
| star_5_rating_count | integer | 5星评分数量 |
| my_star_rating | integer | 我的评分 |
| total_star_rating_count | integer | 总评分数量 |
| only_star_count | integer | 仅评分数量 |
| full_average_rating | number | 完整平均评分 |
| source_type | string | 评分来源类型 |
| created_at | string | 创建时间(ISO 8601) |

## 配置说明

服务配置通过 `config.toml` 文件管理，主要配置项：

```toml
[database]
url = "postgresql://username:password@localhost:5432/market_db"
max_connect = 3

[app]
packages = ["com.huawei.hmsapp.appgallery"]
locale = "zh_CN"

[api]
info_url = "https://web-drcn.hispace.dbankcloud.com/edge/webedge/appinfo"
detail_url = "https://web-drcn.hispace.dbankcloud.com/edge/webedge/appinfo"
timeout_seconds = 30
interval_seconds = 1800

[serve]
url = "localhost"
port = 3000
```

## 使用示例

### cURL 示例（查询包名）
```bash
curl -X GET "http://localhost:3000/api/apps/by-pkg-name/com.huawei.hmsapp.appgallery"
```

### Python 示例
```python
import requests

response = requests.get("http://localhost:3000/api/apps/by-pkg-name/com.huawei.hmsapp.appgallery")
data = response.json()
print(f"应用名称: {data['data']['info']['name']}")
```

### JavaScript 示例
```javascript
fetch('http://localhost:3000/api/apps/by-pkg-name/com.huawei.hmsapp.appgallery')
  .then(response => response.json())
  .then(data => {
    console.log('应用信息:', data.data.info);
    console.log('指标数据:', data.data.metric);
    console.log('评分数据:', data.data.rating);
  });
```

## 注意事项

1. 服务会自动将查询结果保存到PostgreSQL数据库
2. 支持重复数据检测，相同数据不会重复插入
3. 服务会定期自动同步配置中的应用包数据
4. 建议合理控制请求频率，避免过度请求
5. 请遵守华为应用市场的使用条款和隐私政策

## 版本历史

- v1.0.0: 更新端点到 /api/apps/... 结构，新增排行榜API。
- v0.1.0: 初始版本，支持应用信息查询和数据存储
- v0.2.0: 增加评分数据支持，优化数据结构

## 支持

如有问题或建议，请查看项目README或提交Issue。