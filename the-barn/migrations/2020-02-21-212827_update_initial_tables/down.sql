-- This file should undo anything in `up.sql`
ALTER TABLE consumers
DROP COLUMN email,
DROP COLUMN phone,
DROP COLUMN website,

ALTER TABLE markets
DROP COLUMN email,
DROP COLUMN phone,
DROP COLUMN website,


ALTER TABLE producers
DROP COLUMN email,
DROP COLUMN phone,
DROP COLUMN website,
DROP COlUMN description;