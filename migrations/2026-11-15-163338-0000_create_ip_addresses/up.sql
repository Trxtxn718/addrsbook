-- Your SQL goes here
CREATE TABLE ip_addresses (
    id        SERIAL PRIMARY KEY,
    address   INET NOT NULL UNIQUE,
    prefix_id INTEGER NOT NULL REFERENCES prefixes(id),
    hostname  TEXT,
    status    TEXT NOT NULL DEFAULT 'active'
              CHECK (status IN ('active', 'reserved', 'deprecated'))
);

CREATE INDEX ip_addresses_prefix_id_idx ON ip_addresses(prefix_id);