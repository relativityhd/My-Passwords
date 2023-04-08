-- This file should undo anything in `up.sql`-- This file should undo anything in `up.sql`
ALTER TABLE passwords DROP FOREIGN KEY fk_user_id;
ALTER TABLE passwords DROP user_id;
DROP TABLE users;
