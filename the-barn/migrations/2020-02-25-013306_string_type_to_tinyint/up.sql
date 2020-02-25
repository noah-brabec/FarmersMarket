-- Your SQL goes here
ALTER TABLE consumers
DROP COLUMN type;

ALTER TABLE consumers
ADD COLUMN con_type smallint;

ALTER TABLE producers
DROP COLUMN type;

ALTER TABLE producers
ADD COLUMN prod_type smallint;