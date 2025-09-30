-- ตาราง users
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(), -- ใช้ UUID เป็น PK
    username CITEXT NOT NULL,
    email CITEXT UNIQUE NOT NULL, -- email ห้ามซ้ำ
    password_hash TEXT NOT NULL,
    role VARCHAR(20) NOT NULL DEFAULT 'staff', -- สิทธิ์ เช่น staff/admin
    is_active BOOLEAN NOT NULL DEFAULT TRUE, -- ใช้ปิดบัญชีได้
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);