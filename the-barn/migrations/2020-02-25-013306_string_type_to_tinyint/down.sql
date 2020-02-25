-- This file should undo anything in `up.sql`
ALTER TABLE consumers
DROP COLUMN con_type;

ALTER TABLE producers
DROP COLUMN prod_type;