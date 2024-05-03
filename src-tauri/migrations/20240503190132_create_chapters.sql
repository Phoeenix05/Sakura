CREATE TABLE chapters (
    uuid TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    chapter INTEGER NOT NULL,
    author_id TEXT,
    publisher_id TEXT,
    FOREIGN KEY (author_id) REFERENCES authors(uuid),
    FOREIGN KEY (publisher_id) REFERENCES publishers(uuid)
);
