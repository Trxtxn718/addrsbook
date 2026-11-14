-- Your SQL goes here
CREATE TABLE vlans (
    id    SERIAL PRIMARY KEY,
    vid   INTEGER NOT NULL UNIQUE CHECK (vid BETWEEN 1 AND 4094),
    name  TEXT NOT NULL
);