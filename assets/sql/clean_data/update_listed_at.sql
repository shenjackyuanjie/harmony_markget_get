
-- 更新 listed at, 尽量贴近真实日期
UPDATE app_info ai
SET listed_at = LEAST(
    ai.listed_at,
    (SELECT MIN(TO_TIMESTAMP(am.release_date / 1000)) FROM app_metrics am WHERE am.app_id = ai.app_id)
)
WHERE EXISTS (SELECT 1 FROM app_metrics am WHERE am.app_id = ai.app_id);
