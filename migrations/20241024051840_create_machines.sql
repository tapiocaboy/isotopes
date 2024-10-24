-- Add migration script here
CREATE TABLE machines (
    machine_id SERIAL PRIMARY KEY,
    machine_name VARCHAR(255) NOT NULL,
    owner VARCHAR(255) NOT NULL,
    created TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    raw_module_id INTEGER NOT NULL,
    FOREIGN KEY (raw_module_id) REFERENCES raw_modules(raw_module_id)
);

CREATE INDEX idx_machines_owner ON machines(owner);
CREATE INDEX idx_machines_raw_module_id ON machines(raw_module_id);