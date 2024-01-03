-- This file should undo anything in `up.sql`
ALTER TABLE todo DROP COLUMN user_id;
DROP TABLE users