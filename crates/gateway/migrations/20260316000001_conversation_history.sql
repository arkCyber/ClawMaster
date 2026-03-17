-- Conversation History System
-- Records all Q&A pairs with timestamps and resolution status

-- ===========================================================================
-- Conversation Turns Table
-- ===========================================================================
CREATE TABLE IF NOT EXISTS conversation_turns (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    session_key         TEXT    NOT NULL,
    turn_number         INTEGER NOT NULL,
    user_message        TEXT    NOT NULL,
    assistant_response  TEXT    NOT NULL,
    created_at          INTEGER NOT NULL,  -- Unix timestamp
    response_at         INTEGER NOT NULL,  -- Unix timestamp
    duration_ms         INTEGER NOT NULL,  -- Response time in milliseconds
    
    -- Problem resolution tracking
    is_resolved         INTEGER NOT NULL DEFAULT 0,  -- 0=unresolved, 1=resolved, 2=partially
    resolution_status   TEXT,  -- 'resolved', 'unresolved', 'partial', 'escalated'
    user_feedback       TEXT,  -- User's feedback on the response
    feedback_at         INTEGER,  -- When feedback was provided
    
    -- Context and metadata
    model_id            TEXT    NOT NULL,
    provider_name       TEXT    NOT NULL,
    tool_calls_count    INTEGER NOT NULL DEFAULT 0,
    tool_calls_json     TEXT,  -- JSON array of tool calls made
    
    -- Token usage
    input_tokens        INTEGER NOT NULL DEFAULT 0,
    output_tokens       INTEGER NOT NULL DEFAULT 0,
    
    -- Error tracking
    had_error           INTEGER NOT NULL DEFAULT 0,
    error_message       TEXT,
    
    UNIQUE(session_key, turn_number)
);

CREATE INDEX IF NOT EXISTS idx_conversation_turns_session 
    ON conversation_turns(session_key, turn_number DESC);

CREATE INDEX IF NOT EXISTS idx_conversation_turns_created 
    ON conversation_turns(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_conversation_turns_unresolved 
    ON conversation_turns(is_resolved, created_at DESC) 
    WHERE is_resolved = 0;

-- ===========================================================================
-- Unresolved Issues Table
-- ===========================================================================
CREATE TABLE IF NOT EXISTS unresolved_issues (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    conversation_turn_id INTEGER NOT NULL,
    session_key         TEXT    NOT NULL,
    
    -- Issue details
    issue_summary       TEXT    NOT NULL,
    user_query          TEXT    NOT NULL,
    failed_response     TEXT,
    
    -- Timestamps
    created_at          INTEGER NOT NULL,
    last_attempt_at     INTEGER,
    resolved_at         INTEGER,
    
    -- Tracking
    attempt_count       INTEGER NOT NULL DEFAULT 1,
    priority            TEXT    NOT NULL DEFAULT 'normal',  -- 'low', 'normal', 'high', 'critical'
    status              TEXT    NOT NULL DEFAULT 'open',  -- 'open', 'in_progress', 'resolved', 'abandoned'
    
    -- Resolution
    resolution_method   TEXT,  -- 'retry', 'escalation', 'manual', 'alternative_approach'
    resolution_notes    TEXT,
    resolved_by         TEXT,  -- 'system', 'user', 'manual'
    
    FOREIGN KEY (conversation_turn_id) REFERENCES conversation_turns(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_unresolved_issues_session 
    ON unresolved_issues(session_key, status);

CREATE INDEX IF NOT EXISTS idx_unresolved_issues_status 
    ON unresolved_issues(status, priority, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_unresolved_issues_created 
    ON unresolved_issues(created_at DESC);

-- ===========================================================================
-- Conversation Metadata Table
-- ===========================================================================
CREATE TABLE IF NOT EXISTS conversation_metadata (
    session_key         TEXT    PRIMARY KEY,
    
    -- Statistics
    total_turns         INTEGER NOT NULL DEFAULT 0,
    resolved_count      INTEGER NOT NULL DEFAULT 0,
    unresolved_count    INTEGER NOT NULL DEFAULT 0,
    
    -- Timestamps
    first_message_at    INTEGER NOT NULL,
    last_message_at     INTEGER NOT NULL,
    
    -- Quality metrics
    avg_response_time_ms INTEGER NOT NULL DEFAULT 0,
    total_tokens        INTEGER NOT NULL DEFAULT 0,
    error_count         INTEGER NOT NULL DEFAULT 0,
    
    -- User satisfaction
    positive_feedback   INTEGER NOT NULL DEFAULT 0,
    negative_feedback   INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_conversation_metadata_last_message 
    ON conversation_metadata(last_message_at DESC);
