-- Add migration script here
CREATE TABLE ai_modules (
    ai_id SERIAL PRIMARY KEY,
    business_id VARCHAR(255) NOT NULL,
    module_id VARCHAR(255) NOT NULL,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    active BOOLEAN NOT NULL DEFAULT TRUE
);

CREATE INDEX idx_ai_modules_business_id ON ai_modules(business_id);
CREATE INDEX idx_ai_modules_module_id ON ai_modules(module_id);