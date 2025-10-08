-- 1. 创建新的 app_data_history 表
CREATE TABLE app_data_history (
    id              BIGSERIAL PRIMARY KEY,
    app_id          TEXT NOT NULL REFERENCES app_info(app_id),
    pkg_name        TEXT NOT NULL REFERENCES app_info(pkg_name) ON DELETE CASCADE,
    raw_json_data   JSONB NOT NULL DEFAULT '{}'::JSONB,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- 添加一个唯一约束，用于在数据迁移后的去重操作中避免重复插入
    UNIQUE (app_id, pkg_name, raw_json_data, created_at)
);

-- 2. 创建新的 app_rating_history 表 (注意：star 已改名为 rating)
CREATE TABLE app_rating_history (
    id              BIGSERIAL PRIMARY KEY,
    app_id          TEXT NOT NULL REFERENCES app_info(app_id),
    pkg_name        TEXT NOT NULL REFERENCES app_info(pkg_name) ON DELETE CASCADE,
    raw_json_rating JSONB NOT NULL DEFAULT '{}'::JSONB, -- star 改名为 rating
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    -- 添加一个唯一约束，用于在数据迁移后的去重操作中避免重复插入
    UNIQUE (app_id, pkg_name, raw_json_rating, created_at)
);

-- 3. 将 app_raw 表的数据迁移到 app_data_history
INSERT INTO app_data_history (app_id, pkg_name, raw_json_data, created_at)
SELECT app_id, pkg_name, raw_json_data, created_at
FROM app_raw
ON CONFLICT (app_id, pkg_name, raw_json_data, created_at) DO NOTHING; -- 利用唯一约束避免完全重复的行

-- 4. 将 app_raw 表的数据迁移到 app_rating_history
INSERT INTO app_rating_history (app_id, pkg_name, raw_json_rating, created_at)
SELECT app_id, pkg_name, raw_json_star, created_at -- 注意这里仍然是从 raw_json_star 列读取
FROM app_raw
ON CONFLICT (app_id, pkg_name, raw_json_rating, created_at) DO NOTHING; -- 利用唯一约束避免完全重复的行

-- 5. 对 app_data_history 进行去重，只保留每个 (app_id, pkg_name, raw_json_data) 组合的最新记录
-- 这一步需要先找到要删除的旧记录，然后删除它们
DELETE FROM app_data_history
WHERE id IN (
    SELECT adh.id
    FROM (
        SELECT
            id,
            app_id,
            pkg_name,
            raw_json_data,
            created_at,
            ROW_NUMBER() OVER (PARTITION BY app_id, pkg_name, raw_json_data ORDER BY created_at DESC) as rn
        FROM app_data_history
    ) adh
    WHERE adh.rn > 1
);

-- 6. 对 app_rating_history 进行去重，只保留每个 (app_id, pkg_name, raw_json_rating) 组合的最新记录
DELETE FROM app_rating_history
WHERE id IN (
    SELECT arh.id
    FROM (
        SELECT
            id,
            app_id,
            pkg_name,
            raw_json_rating,
            created_at,
            ROW_NUMBER() OVER (PARTITION BY app_id, pkg_name, raw_json_rating ORDER BY created_at DESC) as rn
        FROM app_rating_history
    ) arh
    WHERE arh.rn > 1
);

-- 7. 确认数据迁移和去重无误后，删除旧的 app_raw 表
-- 在生产环境中执行此步骤前，请务必备份数据并仔细检查新表数据
DROP TABLE app_raw;

-- 8. (可选) 添加索引以提高查询性能
CREATE INDEX idx_app_data_history_app_pkg_data ON app_data_history (app_id, pkg_name, raw_json_data);
CREATE INDEX idx_app_data_history_created_at ON app_data_history (created_at);

CREATE INDEX idx_app_rating_history_app_pkg_rating ON app_rating_history (app_id, pkg_name, raw_json_rating);
CREATE INDEX idx_app_rating_history_created_at ON app_rating_history (created_at);
