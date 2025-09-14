CREATE EXTENSION IF NOT EXISTS pgcrypto;  -- สำหรับ gen_random_uuid()
CREATE EXTENSION IF NOT EXISTS citext;    -- สำหรับ CITEXT
CREATE EXTENSION IF NOT EXISTS pg_trgm;   -- สำหรับค้นหาข้อความไวขึ้น