CREATE TABLE IF NOT EXISTS files (
    group_name INTEGER NOT NULL,
    abs_path TEXT NOT NULL,
    lmtime INTEGER NOT NULL,
    hash TEXT,
    PRIMARY KEY (group_name, abs_path),
    FOREIGN KEY(group_name) REFERENCES groups(name)
      ON UPDATE CASCADE
      ON DELETE CASCADE
);
