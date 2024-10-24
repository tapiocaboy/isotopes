-- Add migration script here
CREATE TABLE raw_modules (
    raw_module_id SERIAL PRIMARY KEY,
    module_name VARCHAR(255) NOT NULL,
    module_code TEXT NOT NULL,
    hosted_link VARCHAR(255) NOT NULL
);

CREATE INDEX idx_raw_modules_module_name ON raw_modules(module_name);