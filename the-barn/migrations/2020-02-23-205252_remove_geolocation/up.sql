-- Your SQL goes here
ALTER TABLE consumers
DROP COLUMN geolocation;

ALTER TABLE producers
DROP COLUMN geolocation;

ALTER TABLE markets
DROP COLUMN geolocation;