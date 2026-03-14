-- DO-178C Level A Compliance: Folder Access Control System
-- §11.13: Database schema initialization
-- §6.3.2: All constraints properly defined

-- Folder access permissions table
CREATE TABLE IF NOT EXISTS folder_permissions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    
    -- Path information
    folder_path TEXT NOT NULL UNIQUE,
    folder_hash TEXT NOT NULL,  -- SHA-256 hash for integrity verification
    
    -- Permission flags (DO-178C §6.3.4: Explicit permissions)
    can_read BOOLEAN NOT NULL DEFAULT 0,
    can_write BOOLEAN NOT NULL DEFAULT 0,
    can_execute BOOLEAN NOT NULL DEFAULT 0,
    can_delete BOOLEAN NOT NULL DEFAULT 0,
    
    -- Metadata
    description TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    created_by TEXT NOT NULL,
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT 1,
    
    -- Audit trail
    last_accessed_at INTEGER,
    access_count INTEGER NOT NULL DEFAULT 0
);

-- Access audit log (DO-178C §11.10: Complete audit trail)
CREATE TABLE IF NOT EXISTS folder_access_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    
    -- Reference
    folder_id INTEGER NOT NULL,
    
    -- Access details
    operation TEXT NOT NULL,  -- read, write, execute, delete
    file_path TEXT,           -- Specific file accessed
    success BOOLEAN NOT NULL,
    
    -- Context
    session_key TEXT,
    user_agent TEXT,
    
    -- Error information
    error_message TEXT,
    
    -- Timestamp
    timestamp INTEGER NOT NULL,
    
    FOREIGN KEY (folder_id) REFERENCES folder_permissions(id) ON DELETE CASCADE
);

-- Folder validation rules (DO-178C Security: Path validation)
CREATE TABLE IF NOT EXISTS folder_validation_rules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    
    -- Rule definition
    rule_type TEXT NOT NULL,  -- blacklist, whitelist, pattern
    pattern TEXT NOT NULL,
    description TEXT,
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT 1,
    priority INTEGER NOT NULL DEFAULT 0,
    
    -- Metadata
    created_at INTEGER NOT NULL,
    created_by TEXT NOT NULL
);

-- Indexes for performance (DO-178C §11.10: Efficient queries)
CREATE INDEX IF NOT EXISTS idx_folder_permissions_path ON folder_permissions(folder_path);
CREATE INDEX IF NOT EXISTS idx_folder_permissions_active ON folder_permissions(is_active);
CREATE INDEX IF NOT EXISTS idx_folder_access_log_folder ON folder_access_log(folder_id);
CREATE INDEX IF NOT EXISTS idx_folder_access_log_timestamp ON folder_access_log(timestamp);
CREATE INDEX IF NOT EXISTS idx_folder_access_log_operation ON folder_access_log(operation);
CREATE INDEX IF NOT EXISTS idx_validation_rules_active ON folder_validation_rules(is_active);

-- Insert default validation rules (DO-178C Security: Prevent access to sensitive paths)
INSERT INTO folder_validation_rules (rule_type, pattern, description, is_active, priority, created_at, created_by)
VALUES 
    ('blacklist', '/etc/*', 'System configuration files', 1, 100, strftime('%s', 'now'), 'system'),
    ('blacklist', '/sys/*', 'System kernel interface', 1, 100, strftime('%s', 'now'), 'system'),
    ('blacklist', '/proc/*', 'Process information', 1, 100, strftime('%s', 'now'), 'system'),
    ('blacklist', '/dev/*', 'Device files', 1, 100, strftime('%s', 'now'), 'system'),
    ('blacklist', '/root/*', 'Root user home', 1, 100, strftime('%s', 'now'), 'system'),
    ('blacklist', '*/.*', 'Hidden files and directories', 1, 50, strftime('%s', 'now'), 'system'),
    ('blacklist', '*/.ssh/*', 'SSH keys', 1, 100, strftime('%s', 'now'), 'system'),
    ('blacklist', '*/.gnupg/*', 'GPG keys', 1, 100, strftime('%s', 'now'), 'system'),
    ('blacklist', '*/id_rsa*', 'SSH private keys', 1, 100, strftime('%s', 'now'), 'system'),
    ('blacklist', '*/id_ed25519*', 'SSH private keys', 1, 100, strftime('%s', 'now'), 'system');
