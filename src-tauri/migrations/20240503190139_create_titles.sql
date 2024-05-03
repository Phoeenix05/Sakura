-- Add migration script here
CREATE TABLE titles (
    uuid TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    -- should be something like '["chapters:id", "chapters:id"]',
    -- json array or something similar
    chapters TEXT,
    chapter_count INTEGER NOT NULL,
    latest_chapter TEXT,
    author_id TEXT,
    publisher_id TEXT,
    FOREIGN KEY (latest_chapter) REFERENCES chapters(uuid),
    FOREIGN KEY (author_id) REFERENCES authors(uuid),
    FOREIGN KEY (publisher_id) REFERENCES publishers(uuid)
);
