-- 确保没有 json null
UPDATE app_raw
SET raw_json_star = '{}'::JSONB
WHERE raw_json_star IS NULL
   OR raw_json_star::text = 'null';
