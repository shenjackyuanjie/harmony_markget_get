-- 确保没有 json null
UPDATE app_raw
SET raw_json_star = '{}'::JSONB
WHERE raw_json_star::text = 'null';

UPDATE app_raw
SET raw_json_data = raw_json_data - 'AG-TraceId'
WHERE raw_json_data ? 'AG-TraceId';
