-- Migration 0001: Initial schema for hackathon boilerplate
CREATE TABLE IF NOT EXISTS items (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL DEFAULT (DATETIME('now'))
);

-- Seed an initial record for quick smoke tests
INSERT OR IGNORE INTO items (id, title, description)
VALUES ('00000000-0000-0000-0000-000000000001', 'Welcome Hackathon Item', 'Pre-seeded database entry');
