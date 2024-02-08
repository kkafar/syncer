CREATE TABLE IF NOT EXISTS files (
    id INTEGER AUTO_INCREMENT,
    group_id INTEGER NOT NULL,
    abs_path TEXT NOT NULL,
    lmtime INTEGER NOT NULL,
    hash TEXT,
    PRIMARY KEY (id),
    FOREIGN KEY(group_id) REFERENCES groups(group_id)
);
