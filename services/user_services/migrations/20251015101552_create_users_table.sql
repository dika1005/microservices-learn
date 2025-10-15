-- Add migration script here
-- services/user_services/migrations/20251015100000_create_users_table.sql

CREATE TABLE users (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE
);