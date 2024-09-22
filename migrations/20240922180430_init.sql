-- Add migration script here
CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY,
    serial_number VARCHAR(255) NOT NULL UNIQUE,
    token TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);