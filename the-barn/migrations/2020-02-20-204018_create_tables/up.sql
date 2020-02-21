-- Your SQL goes here
CREATE TABLE markets (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    address JSON NOT NULL,
    geolocation JSON NOT NULL,
    description TEXT
);

CREATE TABLE consumers (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    address JSON NOT NULL,
    type VARCHAR NOT NULL,
    geolocation JSON NOT NULL,
    description TEXT,
    markets UUID[]
);

CREATE TABLE users (
    id UUID PRIMARY KEY,
    first VARCHAR NOT NULL,
    last VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    phone VARCHAR,
    password VARCHAR NOT NULL
);

CREATE TABLE items (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    price MONEY NOT NULL,
    price_unit VARCHAR NOT NULL,
    producer_id UUID REFERENCES producers(id),
    description TEXT,
    post_date DATE,
    tags VARCHAR[]
);

CREATE TABLE user_groups (
    group_id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    organization_id UUID NOT NULL,
    role VARCHAR NOT NULL
);