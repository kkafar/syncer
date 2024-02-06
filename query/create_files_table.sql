CREATE TABLE IF NOT EXISTS files (
    id INTEGER PRIMARY KEY,
    group_id INTEGER NOT NULL,
    abs_path TEXT NOT NULL,
    lmtime INTEGER NOT NULL,
    hash TEXT,
    FOREIGN KEY(group_id) REFERENCES groups(group_id)
);
