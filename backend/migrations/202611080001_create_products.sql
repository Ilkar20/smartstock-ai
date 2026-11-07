CREATE TABLE products (
    id UUID PRIMARY KEY,
    sku VARCHAR(100) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price DOUBLE PRECISION NOT NULL DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT products_name_not_empty
        CHECK (length(trim(name)) > 0),

    CONSTRAINT products_sku_not_empty
        CHECK (length(trim(sku)) > 0),

    CONSTRAINT products_price_valid
        CHECK (
            price >= 0
            AND price != 'Infinity'::float8
            AND price != '-Infinity'::float8
            AND price != 'NaN'::float8
        )
);