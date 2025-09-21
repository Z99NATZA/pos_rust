CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) UNIQUE NOT NULL, 
    name VARCHAR(50) NOT NULL,
    description TEXT DEFAULT '',
    price NUMERIC(18, 2) NOT NULL DEFAULT 0.00,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    image_name VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT products_price_nonneg CHECK (price >= 0) -- price: ป้องกันค่าติดลบ
)

CREATE INDEX IF NOT EXISTS idx_products_name_trgm
    ON products USING GIN (name gin_trgm_ops);

CREATE INDEX IF NOT EXISTS idx_products_code_trgm
    ON products USING GIN (code gin_trgm_ops);