# 华为应用市场数据采集API文档

## 概述

这是一个基于Rust和Axum框架构建的华为应用市场数据采集服务API。该服务提供应用信息查询功能，支持通过包名获取应用的详细信息、指标数据和评分数据。

## 基础信息

- **服务地址**: `http://{host}:{port}`
- **默认端口**: 3000
- **协议**: HTTP/HTTPS
- **数据格式**: JSON

## API端点

### 1. 查询应用信息

查询指定包名的应用详细信息，包括基本信息、指标数据和评分数据。

**端点**: `GET /query/{pkg_name}` 或 `POST /query/{pkg_name}`

**路径参数**:
- `pkg_name` (string, required): 应用包名，如 `com.huawei.hmsapp.appgallery`

**请求示例**:
```bash
GET http://localhost:3000/query/com.huawei.hmsapp.appgallery
```

或
```bash
POST http://localhost:3000/query/com.huawei.hmsapp.appgallery
Content-Type: application/json
```

**响应格式**:
```json
{
  "info": {
    "app_id": "C1164531384803416384",
    "alliance_app_id": "1164531384803416384",
    "name": "应用市场",
    "pkg_name": "com.huawei.hmsapp.appgallery",
    "dev_id": "260086000000068459",
    "developer_name": "华为软件技术有限公司",
    "dev_en_name": "Huawei Software Technologies Co., Ltd.",
    "supplier": "华为软件技术有限公司",
    "kind_id": 10000000,
    "kind_name": "工具",
    "tag_name": "工具",
    "kind_type_id": 13,
    "kind_type_name": "应用",
    "icon_url": "https://example.com/icon.png",
    "brief_desc": "应用市场，点亮精彩生活",
    "description": "详细的应用描述信息...",
    "privacy_url": "https://example.com/privacy",
    "ctype": 17,
    "detail_id": "app|C1164531384803416384",
    "app_level": 2,
    "jocat_id": 10000000,
    "iap": false,
    "hms": false,
    "tariff_type": "免费",
    "packing_type": 4,
    "order_app": false,
    "denpend_gms": false,
    "denpend_hms": false,
    "force_update": false,
    "img_tag": "1",
    "is_pay": false,
    "is_disciplined": false,
    "is_shelves": true,
    "submit_type": 0,
    "delete_archive": false,
    "charging": false,
    "button_grey": false,
    "app_gift": false,
    "free_days": 0,
    "pay_install_type": 0
  },
  "metric": {
    "id": 123,
    "app_id": "C1164531384803416384",
    "version": "6.3.2.302",
    "version_code": 1460302302,
    "size_bytes": 76591487,
    "sha256": "abc123def456...",
    "info_score": 4.5,
    "info_rate_count": 1031,
    "download_count": 14443706,
    "price": "0.00",
    "release_date": 1704067200000,
    "new_features": "新增功能描述...",
    "upgrade_msg": "建议升级提示...",
    "target_sdk": 18,
    "minsdk": 13,
    "compile_sdk_version": 50100,
    "min_hmos_api_level": 50001,
    "api_release_type": "Release",
    "created_at": "2024-01-15T10:30:00+08:00"
  },
  "rating": {
    "id": 456,
    "app_id": "C1164531384803416384",
    "average_rating": 2.9,
    "star_1_rating_count": 348,
    "star_2_rating_count": 129,
    "star_3_rating_count": 129,
    "star_4_rating_count": 81,
    "star_5_rating_count": 344,
    "my_star_rating": 0,
    "total_star_rating_count": 1031,
    "only_star_count": 511,
    "full_average_rating": 5.0,
    "source_type": "USER_RATING",
    "created_at": "2024-01-15T10:30:00+08:00"
  },
  "is_new": true
}
```

**错误响应**:
```json
{
  "data": "faild to fetch",
  "error": "错误信息描述"
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

## 响应字段说明

- `is_new`: boolean - 表示本次查询是否插入了新数据到数据库

## 错误处理

服务可能返回以下类型的错误：

1. **包名不存在**: 当指定的包名在华为应用市场不存在时
2. **网络超时**: API请求超时（默认30秒）
3. **数据库错误**: 数据库连接或操作失败
4. **解析错误**: API响应数据解析失败

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

### cURL 示例
```bash
curl -X GET "http://localhost:3000/query/com.huawei.hmsapp.appgallery"
```

### Python 示例
```python
import requests

response = requests.get("http://localhost:3000/query/com.huawei.hmsapp.appgallery")
data = response.json()

print(f"应用名称: {data['info']['name']}")
print(f"版本: {data['metric']['version']}")
print(f"评分: {data['rating']['average_rating']}")
```

### JavaScript 示例
```javascript
fetch('http://localhost:3000/query/com.huawei.hmsapp.appgallery')
  .then(response => response.json())
  .then(data => {
    console.log('应用信息:', data.info);
    console.log('指标数据:', data.metric);
    console.log('评分数据:', data.rating);
  });
```

## 注意事项

1. 服务会自动将查询结果保存到PostgreSQL数据库
2. 支持重复数据检测，相同数据不会重复插入
3. 服务会定期自动同步配置中的应用包数据
4. 建议合理控制请求频率，避免过度请求
5. 请遵守华为应用市场的使用条款和隐私政策

## 版本历史

- v0.1.0: 初始版本，支持应用信息查询和数据存储
- v0.2.0: 增加评分数据支持，优化数据结构

## 支持

如有问题或建议，请查看项目README或提交Issue。