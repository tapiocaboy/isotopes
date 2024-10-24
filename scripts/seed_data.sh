#!/bin/bash

# Database connection details
DB_NAME="your_database_name"
DB_USER="your_username"
DB_PASSWORD="your_password"
DB_HOST="localhost"
DB_PORT="5432"

# Function to execute SQL
execute_sql() {
    PGPASSWORD=$DB_PASSWORD psql -h $DB_HOST -p $DB_PORT -U $DB_USER -d $DB_NAME -c "$1"
}

# Seed ai_modules table
execute_sql "
INSERT INTO ai_modules (business_id, module_id, active) VALUES
('business1', 'module1', true),
('business2', 'module2', true),
('business3', 'module3', false);
"

# Seed raw_modules table
execute_sql "
INSERT INTO raw_modules (module_name, module_code, hosted_link) VALUES
('Raw Module 1', 'code1', 'http://example.com/1'),
('Raw Module 2', 'code2', 'http://example.com/2'),
('Raw Module 3', 'code3', 'http://example.com/3');
"

# Seed subscriptions table
execute_sql "
INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES
('11111111-1111-1111-1111-111111111111', 'user1@example.com', 'User One', CURRENT_TIMESTAMP),
('22222222-2222-2222-2222-222222222222', 'user2@example.com', 'User Two', CURRENT_TIMESTAMP),
('33333333-3333-3333-3333-333333333333', 'user3@example.com', 'User Three', CURRENT_TIMESTAMP);
"

# Seed machines table
execute_sql "
INSERT INTO machines (machine_name, owner, active, raw_module_id) VALUES
('Machine 1', 'Owner 1', true, 1),
('Machine 2', 'Owner 2', true, 2),
('Machine 3', 'Owner 3', false, 3);
"

echo "Seed data inserted successfully!"
