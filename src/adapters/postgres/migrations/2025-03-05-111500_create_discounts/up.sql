CREATE TABLE discounts (
    id BIGSERIAL PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    discount_type TEXT NOT NULL,
    value DOUBLE PRECISION NOT NULL,
    min_purchase_amount DOUBLE PRECISION,
    max_discount_amount DOUBLE PRECISION,
    starts_at TIMESTAMP NOT NULL,
    expires_at TIMESTAMP,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    usage_limit INTEGER,
    usage_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);