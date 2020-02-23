-- This file should undo anything in `up.sql`
ALTER TABLE consumers
ADD COLUMN geolocation JSON;

ALTER TABLE producers
ADD COLUMN geolocation JSON;

ALTER TABLE markets
ADD COLUMN geolocation JSON;