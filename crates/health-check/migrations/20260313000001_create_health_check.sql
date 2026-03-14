-- Health Check System Tables
-- DO-178C Level A Compliant
-- 
-- This migration creates tables for storing health check history
-- for analysis and compliance reporting

-- Health check history table
CREATE TABLE IF NOT EXISTS health_check_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    overall_status TEXT NOT NULL CHECK(overall_status IN ('healthy', 'degraded', 'unhealthy')),
    total_duration_ms INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Individual check results
CREATE TABLE IF NOT EXISTS health_check_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    history_id INTEGER NOT NULL,
    check_name TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('healthy', 'degraded', 'unhealthy')),
    criticality TEXT NOT NULL CHECK(criticality IN ('critical', 'important', 'optional')),
    duration_ms INTEGER NOT NULL,
    reason TEXT,
    metadata TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (history_id) REFERENCES health_check_history(id) ON DELETE CASCADE
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_health_history_timestamp ON health_check_history(timestamp);
CREATE INDEX IF NOT EXISTS idx_health_history_status ON health_check_history(overall_status);
CREATE INDEX IF NOT EXISTS idx_health_results_history ON health_check_results(history_id);
CREATE INDEX IF NOT EXISTS idx_health_results_name ON health_check_results(check_name);
CREATE INDEX IF NOT EXISTS idx_health_results_status ON health_check_results(status);

-- Resource metrics history (for trending analysis)
CREATE TABLE IF NOT EXISTS resource_metrics_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TEXT NOT NULL,
    cpu_usage_percent REAL NOT NULL,
    memory_used_bytes INTEGER NOT NULL,
    memory_total_bytes INTEGER NOT NULL,
    memory_usage_percent REAL NOT NULL,
    disk_used_bytes INTEGER NOT NULL,
    disk_total_bytes INTEGER NOT NULL,
    disk_usage_percent REAL NOT NULL,
    active_connections INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_resource_metrics_timestamp ON resource_metrics_history(timestamp);
