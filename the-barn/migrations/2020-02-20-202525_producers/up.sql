-- Your SQL goes here
CREATE TABLE producers (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    address JSON NOT NULL,
    type VARCHAR NOT NULL,
    geolocation JSON NOT NULL,
    markets UUID[]
)