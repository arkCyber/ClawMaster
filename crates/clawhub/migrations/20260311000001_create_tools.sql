-- Create tools table
CREATE TABLE IF NOT EXISTS tools (
    -- Primary key
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    
    -- Tool identification
    name TEXT NOT NULL,
    version TEXT NOT NULL,
    
    -- Metadata
    description TEXT NOT NULL,
    readme TEXT,
    author TEXT NOT NULL,
    author_email TEXT,
    license TEXT NOT NULL,
    repository TEXT,
    homepage TEXT,
    
    -- Classification
    tool_type TEXT NOT NULL CHECK(tool_type IN ('pure', 'http')),
    keywords TEXT NOT NULL, -- JSON array
    categories TEXT NOT NULL, -- JSON array
    
    -- Wasm file info
    wasm_hash TEXT NOT NULL,
    wasm_size INTEGER NOT NULL,
    wasm_url TEXT NOT NULL,
    
    -- Security
    signature TEXT NOT NULL,
    public_key TEXT NOT NULL,
    security_status TEXT NOT NULL CHECK(security_status IN ('pending', 'scanning', 'verified', 'failed', 'approved')),
    
    -- Statistics
    downloads INTEGER NOT NULL DEFAULT 0,
    
    -- Timestamps
    published_at TEXT NOT NULL, -- RFC3339 format
    updated_at TEXT NOT NULL, -- RFC3339 format
    
    -- Constraints
    UNIQUE(name, version)
);

-- Index for fast lookups
CREATE INDEX IF NOT EXISTS idx_tools_name ON tools(name);
CREATE INDEX IF NOT EXISTS idx_tools_name_version ON tools(name, version);
CREATE INDEX IF NOT EXISTS idx_tools_downloads ON tools(downloads DESC);
CREATE INDEX IF NOT EXISTS idx_tools_published_at ON tools(published_at DESC);
CREATE INDEX IF NOT EXISTS idx_tools_security_status ON tools(security_status);

-- Full-text search index
CREATE VIRTUAL TABLE IF NOT EXISTS tools_fts USING fts5(
    name,
    description,
    keywords,
    content=tools,
    content_rowid=id
);

-- Triggers to keep FTS in sync
CREATE TRIGGER IF NOT EXISTS tools_fts_insert AFTER INSERT ON tools BEGIN
    INSERT INTO tools_fts(rowid, name, description, keywords)
    VALUES (new.id, new.name, new.description, new.keywords);
END;

CREATE TRIGGER IF NOT EXISTS tools_fts_delete AFTER DELETE ON tools BEGIN
    DELETE FROM tools_fts WHERE rowid = old.id;
END;

CREATE TRIGGER IF NOT EXISTS tools_fts_update AFTER UPDATE ON tools BEGIN
    UPDATE tools_fts SET
        name = new.name,
        description = new.description,
        keywords = new.keywords
    WHERE rowid = new.id;
END;
