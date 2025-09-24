CREATE TYPE payment_method AS ENUM ('CASH', 'TRANSFER');

CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    note TEXT DEFAULT '',
    grand_total NUMERIC(18, 2) NOT NULL DEFAULT 0.00,
    payment_method payment_method NOT NULL DEFAULT 'CASH',
    paid_amount NUMERIC(18, 2) NOT NULL DEFAULT 0.00,
    change_amount NUMERIC(18, 2) NOT NULL DEFAULT 0.00,
    user_id UUID NOT NULL REFERENCES users(id),
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
)