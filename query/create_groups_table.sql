CREATE TABLE IF NOT EXISTS groups (
    id INTEGER AUTO_INCREMENT,
    name TEXT NOT NULL UNIQUE,
    prefix TEXT NOT NULL UNIQUE,
    PRIMARY KEY (id)
);