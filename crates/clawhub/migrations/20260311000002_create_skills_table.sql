-- Create skills table for SKILL.md and other skill formats
CREATE TABLE IF NOT EXISTS skills (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    description TEXT NOT NULL,
    readme TEXT,
    author TEXT NOT NULL,
    author_email TEXT,
    license TEXT NOT NULL,
    repository TEXT,
    homepage TEXT,
    keywords TEXT NOT NULL,  -- JSON array
    categories TEXT NOT NULL,  -- JSON array
    skill_format TEXT NOT NULL,  -- 'skill_md', 'claude_code', etc.
    github_repo TEXT,  -- owner/repo format
    commit_sha TEXT,
    downloads INTEGER NOT NULL DEFAULT 0,
    stars INTEGER NOT NULL DEFAULT 0,
    security_status TEXT NOT NULL,
    published_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(name, version)
);

-- Full-text search index for skills
CREATE VIRTUAL TABLE IF NOT EXISTS skills_fts USING fts5(
    name,
    description,
    keywords,
    content=skills,
    content_rowid=id
);

-- Triggers to keep FTS index in sync
CREATE TRIGGER IF NOT EXISTS skills_ai AFTER INSERT ON skills BEGIN
    INSERT INTO skills_fts(rowid, name, description, keywords)
    VALUES (new.id, new.name, new.description, new.keywords);
END;

CREATE TRIGGER IF NOT EXISTS skills_ad AFTER DELETE ON skills BEGIN
    DELETE FROM skills_fts WHERE rowid = old.id;
END;

CREATE TRIGGER IF NOT EXISTS skills_au AFTER UPDATE ON skills BEGIN
    UPDATE skills_fts SET
        name = new.name,
        description = new.description,
        keywords = new.keywords
    WHERE rowid = new.id;
END;

-- Index for common queries
CREATE INDEX IF NOT EXISTS idx_skills_name ON skills(name);
CREATE INDEX IF NOT EXISTS idx_skills_format ON skills(skill_format);
CREATE INDEX IF NOT EXISTS idx_skills_downloads ON skills(downloads DESC);
CREATE INDEX IF NOT EXISTS idx_skills_published ON skills(published_at DESC);
