CREATE TABLE order_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_code VARCHAR(50) NOT NULL REFERENCES orders(code) ON DELETE CASCADE,
    product_code VARCHAR(50) NOT NULL REFERENCES products(code),
    product_name VARCHAR(100) NOT NULL,
    unit_price NUMERIC(18,2) NOT NULL,
    qty NUMERIC(10,2) NOT NULL CHECK (qty > 0),
    total_amount NUMERIC(18,2) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
