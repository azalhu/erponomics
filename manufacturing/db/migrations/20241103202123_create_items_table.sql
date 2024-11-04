-- Add migration script here
CREATE TABLE IF NOT EXISTS items
(
    id              TEXT    PRIMARY KEY NOT NULL,
    code            TEXT                NOT NULL,
    name            TEXT                NOT NULL,
    description     TEXT                NOT NULL,
    created_at      TEXT                NOT NULL
) STRICT;
