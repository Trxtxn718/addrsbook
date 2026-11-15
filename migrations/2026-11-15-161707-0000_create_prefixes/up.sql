-- Your SQL goes here
CREATE TABLE prefixes (
    id          SERIAL PRIMARY KEY,
    network     CIDR NOT NULL UNIQUE,
    vlan_id     INTEGER REFERENCES vlans(id),
    status      TEXT NOT NULL DEFAULT 'active'
                CHECK (status IN ('active', 'reserved', 'deprecated')),
    description TEXT
);