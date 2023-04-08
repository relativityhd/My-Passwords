-- Your SQL goes here

CREATE TABLE
    users (
        id        INT(6) UNSIGNED AUTO_INCREMENT,
        username  VARCHAR(255) NOT NULL UNIQUE,
        pass      VARCHAR(32) NOT NULL,
        PRIMARY KEY (id)
    );

ALTER TABLE passwords ADD user_id INT(6) UNSIGNED NOT NULL DEFAULT 0;
ALTER TABLE passwords ADD CONSTRAINT fk_user_id
FOREIGN KEY (user_id) REFERENCES users(id)
ON DELETE CASCADE ON UPDATE CASCADE;
