CREATE TABLE IF NOT EXISTS groups (
    name TEXT NOT NULL UNIQUE,
    prefix TEXT NOT NULL UNIQUE,
    PRIMARY KEY (name) 
);
