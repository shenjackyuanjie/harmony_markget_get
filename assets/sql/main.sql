-- 优化后的 SQL 表定义，使用更合适的数据类型

CREATE TABLE app_info (
    app_id                  TEXT PRIMARY KEY,   -- 应用唯一ID（如 C1164531384803416384）
    alliance_app_id         TEXT,               -- 联盟应用ID（如 1164531384803416384）
    name                    TEXT NOT NULL,      -- 应用名称（如 应用市场）
    pkg_name                TEXT NOT NULL,      -- 应用包名（如 com.huawei.hmsapp.appgallery）
    dev_id                  TEXT NOT NULL,      -- 开发者ID（如 260086000000068459）
    developer_name          TEXT NOT NULL,      -- 开发者名称（如 华为软件技术有限公司）
    dev_en_name             TEXT,               -- 开发者英文名称（如 Huawei Software Technologies Co., Ltd.）
    supplier                TEXT,               -- 供应商名称（如 华为软件技术有限公司）
    kind_id                 INTEGER NOT NULL,   -- 应用分类ID（如 10000000）
    kind_name               TEXT NOT NULL,      -- 应用分类名称（如 工具）
    tag_name                TEXT,               -- 标签名称（如 工具）
    kind_type_id            INTEGER NOT NULL,   -- 类型ID（如 13）
    kind_type_name          TEXT NOT NULL,      -- 类型名称（如 应用）
    icon_url                TEXT NOT NULL,      -- 应用图标URL
    brief_desc              TEXT NOT NULL,      -- 简短描述（如 应用市场，点亮精彩生活）
    description             TEXT NOT NULL,      -- 应用详细描述
    privacy_url             TEXT NOT NULL,      -- 隐私政策链接
    ctype                   INTEGER NOT NULL,   -- 客户端类型（如 17）
    detail_id               TEXT NOT NULL,      -- 详情页ID（如 app|C1164531384803416384）
    app_level               INTEGER NOT NULL,   -- 应用等级（如 2）
    jocat_id                INTEGER NOT NULL,   -- 分类ID（如 10000000）
    iap                     BOOLEAN NOT NULL,   -- 是否含应用内购买（false=否）
    hms                     BOOLEAN NOT NULL,   -- 是否依赖HMS（false=否）
    tariff_type             TEXT NOT NULL,      -- 资费类型（如 免费/付费）
    packing_type            INTEGER NOT NULL,   -- 打包类型（如 4）
    order_app               BOOLEAN NOT NULL,   -- 是否预装应用（false）
    denpend_gms             BOOLEAN NOT NULL,   -- 是否依赖GMS（false=否）
    denpend_hms             BOOLEAN NOT NULL,   -- 是否依赖HMS（false=否）
    force_update            BOOLEAN NOT NULL,   -- 是否强制更新（false=否）
    img_tag                 TEXT NOT NULL,      -- 图片标签（如 1）
    is_pay                  BOOLEAN NOT NULL,   -- 是否付费（false=否）
    is_disciplined          BOOLEAN NOT NULL,   -- 是否合规（false=否）
    is_shelves              BOOLEAN NOT NULL,   -- 是否上架（true=是）
    submit_type             INTEGER NOT NULL,   -- 提交类型（0）
    delete_archive          BOOLEAN NOT NULL,   -- 是否删除归档（false=否）
    charging                BOOLEAN NOT NULL,   -- 是否收费（false=否）
    button_grey             BOOLEAN NOT NULL,   -- 按钮是否置灰（false=否）
    app_gift                BOOLEAN NOT NULL,   -- 是否有礼包（false=否）
    free_days               INTEGER NOT NULL,   -- 免费天数（0）
    pay_install_type        INTEGER NOT NULL,   -- 付费安装类型（0）
    created_at              TIMESTAMPTZ NOT NULL DEFAULT now() -- 创建时间
);

CREATE TABLE app_raw (
    id              BIGSERIAL PRIMARY KEY,                     -- 主键ID
    app_id          TEXT NOT NULL REFERENCES app_info(app_id), -- 对应 app_info 的 app_id
    raw_json_data   JSONB NOT NULL DEFAULT '{}'::JSONB,        -- 原始应用数据JSON
    raw_json_star   JSONB NOT NULL DEFAULT '{}'::JSONB,        -- 原始评分数据JSON
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()           -- 创建时间
);

CREATE TABLE app_metrics (
    id                  BIGSERIAL PRIMARY KEY,             -- 主键ID
    app_id              TEXT NOT NULL REFERENCES app_info(app_id),  -- 对应 app_info 的 app_id
    version             TEXT NOT NULL,                    -- 版本号（如 6.3.2.302）
    version_code        BIGINT NOT NULL,                  -- 版本代码（如 1460302302）
    size_bytes          BIGINT NOT NULL,                  -- 应用大小（字节）（如 76591487）
    sha256              TEXT NOT NULL,                    -- 安装包SHA256校验值
    info_score          NUMERIC(3,1) NOT NULL,            -- 信息评分（重命名自 hot_score）
    info_rate_count     BIGINT NOT NULL,                  -- 信息评分人数（重命名自 rate_num）
    download_count      BIGINT NOT NULL,                  -- 下载次数（如 14443706）
    price               NUMERIC(10,2) NOT NULL,           -- 价格（如 0 表示免费）
    release_date        BIGINT NOT NULL,                  -- 发布时间（时间戳毫秒）
    new_features        TEXT NOT NULL,                    -- 新功能描述
    upgrade_msg         TEXT NOT NULL,                    -- 升级提示
    target_sdk          INTEGER NOT NULL,                 -- 目标SDK版本（如 18）
    minsdk              INTEGER NOT NULL,                 -- 最低SDK版本（如 13）
    compile_sdk_version INTEGER NOT NULL,                 -- 编译SDK版本（如 50100）
    min_hmos_api_level  INTEGER NOT NULL,                 -- 最低鸿蒙API等级（如 50001）
    api_release_type    TEXT NOT NULL,                    -- API发布类型（如 Release）
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now()  -- 创建时间
);

CREATE TABLE app_rating (
    id                          BIGSERIAL PRIMARY KEY,             -- 主键ID
    app_id                      TEXT NOT NULL REFERENCES app_info(app_id),  -- 对应 app_info 的 app_id
    average_rating              NUMERIC(3,1) NOT NULL,            -- 平均评分
    star_1_rating_count         INTEGER NOT NULL,                 -- 1星评分数量
    star_2_rating_count         INTEGER NOT NULL,                 -- 2星评分数量
    star_3_rating_count         INTEGER NOT NULL,                 -- 3星评分数量
    star_4_rating_count         INTEGER NOT NULL,                 -- 4星评分数量
    star_5_rating_count         INTEGER NOT NULL,                 -- 5星评分数量
    my_star_rating              INTEGER NOT NULL,                 -- 我的评分
    total_star_rating_count     INTEGER NOT NULL,                 -- 总评分数量
    only_star_count             INTEGER NOT NULL,                 -- 仅评分数量
    full_average_rating         NUMERIC(3,1) NOT NULL,            -- 完整平均评分
    source_type                 TEXT NOT NULL,                    -- 评分来源类型
    created_at                  TIMESTAMPTZ NOT NULL DEFAULT now()  -- 创建时间
);

CREATE OR REPLACE VIEW app_latest_info AS
SELECT ai.app_id,
   ai.alliance_app_id,
   ai.name,
   ai.pkg_name,
   ai.dev_id,
   ai.developer_name,
   ai.dev_en_name,
   ai.supplier,
   ai.kind_id,
   ai.kind_name,
   ai.tag_name,
   ai.kind_type_id,
   ai.kind_type_name,
   ai.icon_url,
   ai.brief_desc,
   ai.description,
   ai.privacy_url,
   ai.ctype,
   ai.detail_id,
   ai.app_level,
   ai.jocat_id,
   ai.iap,
   ai.hms,
   ai.tariff_type,
   ai.packing_type,
   ai.order_app,
   ai.denpend_gms,
   ai.denpend_hms,
   ai.force_update,
   ai.img_tag,
   ai.is_pay,
   ai.is_disciplined,
   ai.is_shelves,
   ai.submit_type,
   ai.delete_archive,
   ai.charging,
   ai.button_grey,
   ai.app_gift,
   ai.free_days,
   ai.pay_install_type,
   ai.created_at,  -- 创建时间
   am.version,
   am.version_code,
   am.size_bytes,
   am.sha256,
   am.info_score,
   am.info_rate_count,
   am.download_count,
   am.price,
   am.release_date,
   am.new_features,
   am.upgrade_msg,
   am.target_sdk,
   am.minsdk,
   am.compile_sdk_version,
   am.min_hmos_api_level,
   am.api_release_type,
   ar.average_rating,
   ar.star_1_rating_count,
   ar.star_2_rating_count,
   ar.star_3_rating_count,
   ar.star_4_rating_count,
   ar.star_5_rating_count,
   ar.my_star_rating,
   ar.total_star_rating_count,
   ar.only_star_count,
   ar.full_average_rating,
   ar.source_type,
   am.created_at AS metrics_created_at,
   ar.created_at AS rating_created_at
  FROM app_info ai
    LEFT JOIN ( SELECT DISTINCT ON (app_metrics.app_id) app_metrics.id,
           app_metrics.app_id,
           app_metrics.version,
           app_metrics.version_code,
           app_metrics.size_bytes,
           app_metrics.sha256,
           app_metrics.download_count,
           app_metrics.price,
           app_metrics.release_date,
           app_metrics.new_features,
           app_metrics.upgrade_msg,
           app_metrics.target_sdk,
           app_metrics.minsdk,
           app_metrics.compile_sdk_version,
           app_metrics.min_hmos_api_level,
           app_metrics.api_release_type,
           app_metrics.created_at,
           app_metrics.info_score,
           app_metrics.info_rate_count
          FROM app_metrics
         ORDER BY app_metrics.app_id, app_metrics.created_at DESC NULLS LAST) am ON ai.app_id = am.app_id
    LEFT JOIN ( SELECT DISTINCT ON (app_rating.app_id) app_rating.id,
           app_rating.app_id,
           app_rating.average_rating,
           app_rating.star_1_rating_count,
           app_rating.star_2_rating_count,
           app_rating.star_3_rating_count,
           app_rating.star_4_rating_count,
           app_rating.star_5_rating_count,
           app_rating.my_star_rating,
           app_rating.total_star_rating_count,
           app_rating.only_star_count,
           app_rating.full_average_rating,
           app_rating.source_type,
           app_rating.created_at
          FROM app_rating
         ORDER BY app_rating.app_id, app_rating.created_at DESC NULLS LAST) ar ON ai.app_id = ar.app_id;

-- 创建索引以提高查询性能
CREATE INDEX idx_app_info_app_id ON app_info(app_id);
CREATE INDEX idx_app_raw_app_id ON app_raw(app_id);
CREATE INDEX idx_app_metrics_app_id ON app_metrics(app_id);
CREATE INDEX idx_app_metrics_version ON app_metrics(version);
CREATE INDEX idx_app_metrics_download_count ON app_metrics(download_count);
CREATE INDEX idx_app_rating_app_id ON app_rating(app_id);
CREATE INDEX idx_app_rating_version ON app_rating(version);