CREATE TABLE variations (
    id BIGSERIAL PRIMARY KEY,
    category_id BIGINT NOT NULL,
    name TEXT NOT NULL,
    CONSTRAINT fk_category FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
);

CREATE TABLE variation_options (
    id BIGSERIAL PRIMARY KEY,
    variation_id BIGINT NOT NULL,
    value TEXT NOT NULL,
    CONSTRAINT fk_variation FOREIGN KEY (variation_id) REFERENCES variations(id) ON DELETE CASCADE
);