-- Add migration script here
CREATE TABLE groups (
    uuid TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    -- should be something like '["titles:id", "titles:id"]',
    -- json array or something similar
    titles TEXT
);
