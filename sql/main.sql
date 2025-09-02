-- 修改后的 SQL 表定义，与 Rust 数据结构保持一致

CREATE TABLE app_info (
    app_id                  TEXT PRIMARY KEY,   -- 应用唯一ID（如 C1164531384803416384）
    alliance_app_id         TEXT,               -- 联盟应用ID（如 1164531384803416384）
    name                    TEXT,               -- 应用名称（如 应用市场）
    pkg_name                TEXT,               -- 应用包名（如 com.huawei.hmsapp.appgallery）
    dev_id                  TEXT,               -- 开发者ID（如 260086000000068459）
    developer_name          TEXT,               -- 开发者名称（如 华为软件技术有限公司）
    dev_en_name             TEXT,               -- 开发者英文名称（如 Huawei Software Technologies Co., Ltd.）
    supplier                TEXT,               -- 供应商名称（如 华为软件技术有限公司）
    kind_id                 TEXT,               -- 应用分类ID（如 10000000）
    kind_name               TEXT,               -- 应用分类名称（如 工具）
    tag_name                TEXT,               -- 标签名称（如 工具）
    kind_type_id            TEXT,               -- 类型ID（如 13）
    kind_type_name          TEXT,               -- 类型名称（如 应用）
    icon_url                TEXT,               -- 应用图标URL
    brief_desc              TEXT,               -- 简短描述（如 应用市场，点亮精彩生活）
    description             TEXT,               -- 应用详细描述
    privacy_url             TEXT,               -- 隐私政策链接
    ctype                   INTEGER,            -- 客户端类型（如 17）
    detail_id               TEXT,               -- 详情页ID（如 app|C1164531384803416384）
    app_level               INTEGER,            -- 应用等级（如 2）
    jocat_id                INTEGER,            -- 分类ID（如 10000000）
    iap                     INTEGER,            -- 是否含应用内购买（0=否）
    hms                     INTEGER,            -- 是否依赖HMS（0=否）
    tariff_type             TEXT,               -- 资费类型（如 免费/付费）
    packing_type            INTEGER,            -- 打包类型（如 4）
    order_app               BOOLEAN,            -- 是否预装应用（false）
    denpend_gms             INTEGER,            -- 是否依赖GMS（0=否）
    denpend_hms             INTEGER,            -- 是否依赖HMS（0=否）
    force_update            INTEGER,            -- 是否强制更新（0=否）
    img_tag                 TEXT,               -- 图片标签（如 1）
    is_pay                  TEXT,               -- 是否付费（0=否）
    is_disciplined          INTEGER,            -- 是否合规（0=否）
    is_shelves              INTEGER,            -- 是否上架（1=是）
    submit_type             INTEGER,            -- 提交类型（0）
    delete_archive          INTEGER,            -- 是否删除归档（0=否）
    charging                INTEGER,            -- 是否收费（0=否）
    button_grey             INTEGER,            -- 按钮是否置灰（0=否）
    app_gift                INTEGER,            -- 是否有礼包（0=否）
    free_days               INTEGER,            -- 免费天数（0）
    pay_install_type        INTEGER            -- 付费安装类型（0）
);

CREATE TABLE app_raw (
    id          BIGSERIAL PRIMARY KEY,                     -- 主键ID
    app_id      TEXT REFERENCES app_info(app_id),          -- 对应 app_info 的 app_id
    raw_json    JSONB,                                     -- 原始JSON数据（完整应用信息）
    created_at  TIMESTAMPTZ DEFAULT now()                    -- 创建时间
);

CREATE TABLE app_permissions (
    id                  BIGSERIAL PRIMARY KEY,             -- 主键ID
    app_id              TEXT REFERENCES app_info(app_id),  -- 对应 app_info 的 app_id
    group_desc          TEXT,                              -- 权限分组描述（如 其他权限）
    permission_label    TEXT,                              -- 权限名称（如 完全的网络访问权限）
    permission_desc     TEXT,                              -- 权限详细说明
    hide                TEXT                               -- 是否隐藏（n=否）
);

CREATE TABLE app_metrics (
    id                  BIGSERIAL PRIMARY KEY,             -- 主键ID
    app_id              TEXT REFERENCES app_info(app_id),  -- 对应 app_info 的 app_id
    version             TEXT,                              -- 版本号（如 6.3.2.302）
    version_code        BIGINT,                            -- 版本代码（如 1460302302）
    size_bytes          BIGINT,                            -- 应用大小（字节）（如 76591487）
    sha256              TEXT,                              -- 安装包SHA256校验值
    hot_score           TEXT,                              -- 热度评分（如 4.5）
    rate_num            TEXT,                              -- 评分人数（如 350）
    download_count      TEXT,                              -- 下载次数（如 14443706）
    price               TEXT,                              -- 价格（如 0 表示免费）
    release_date        BIGINT,                            -- 发布时间（时间戳毫秒）
    new_features        TEXT,                              -- 新功能描述
    upgrade_msg         TEXT,                              -- 升级提示
    target_sdk          TEXT,                              -- 目标SDK版本（如 18）
    minsdk              TEXT,                              -- 最低SDK版本（如 13）
    compile_sdk_version INTEGER,                           -- 编译SDK版本（如 50100）
    min_hmos_api_level  INTEGER,                           -- 最低鸿蒙API等级（如 50001）
    api_release_type    TEXT,                              -- API发布类型（如 Release）
    created_at          TIMESTAMPTZ DEFAULT now()            -- 创建时间
);

-- 创建索引以提高查询性能
CREATE INDEX idx_app_info_app_id ON app_info(app_id);
CREATE INDEX idx_app_raw_app_id ON app_raw(app_id);
CREATE INDEX idx_app_permissions_app_id ON app_permissions(app_id);
CREATE INDEX idx_app_metrics_app_id ON app_metrics(app_id);
CREATE INDEX idx_app_metrics_version ON app_metrics(version);
