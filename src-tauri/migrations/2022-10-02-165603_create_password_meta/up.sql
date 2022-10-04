-- Your SQL goes here

CREATE TABLE
    passwords (
        id            INT(6) UNSIGNED AUTO_INCREMENT,
        account_name  VARCHAR(255) NOT NULL DEFAULT '',
        secret        VARCHAR(255) NOT NULL,
        institution   VARCHAR(255) NOT NULL,
        industry      VARCHAR(255) NOT NULL,
        is_legacy        BOOLEAN NOT NULL DEFAULT FALSE,
        is_work          BOOLEAN NOT NULL DEFAULT FALSE,
        date_created  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
        PRIMARY KEY (id)
    );