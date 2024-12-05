-- Add migration script here
CREATE TABLE IF NOT EXISTS item
(
    id              TEXT        PRIMARY KEY NOT NULL,
    display_name    TEXT                    NOT NULL,
    title           TEXT                    NOT NULL,
    description     TEXT                    NOT NULL,
    state           INTEGER                 NOT NULL,
    etag            TEXT                    NOT NULL,
    uid             TEXT                    NOT NULL,
    create_time     TEXT                    NOT NULL,
    update_time     TEXT                    NOT NULL
) STRICT;
