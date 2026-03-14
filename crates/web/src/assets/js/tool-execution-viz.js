/**
 * Tool Execution Visualization
 * Real-time display of tool execution chain with tree view
 */

import { html, render } from './vendor/preact-htm.js';
import { useState, useEffect, useRef } from './vendor/preact-hooks.js';
import * as icons from './icons.js';

/**
 * Tool execution tree node
 * @typedef {Object} ToolExecution
 * @property {string} id - Unique execution ID
 * @property {string} tool - Tool name
 * @property {Object} args - Tool arguments
 * @property {string} status - 'pending' | 'running' | 'success' | 'error'
 * @property {any} result - Execution result
 * @property {string} error - Error message if failed
 * @property {number} duration - Execution duration in ms
 * @property {number} startTime - Start timestamp
 * @property {number} endTime - End timestamp
 * @property {ToolExecution[]} children - Child executions
 */

/**
 * Tool execution visualizer component
 */
export function ToolExecutionViz({ sessionId }) {
    const [executions, setExecutions] = useState([]);
    const [expanded, setExpanded] = useState(new Set());
    const [autoScroll, setAutoScroll] = useState(true);
    const containerRef = useRef(null);

    useEffect(() => {
        // Subscribe to tool execution events
        const handleToolStart = (event) => {
            if (event.sessionId !== sessionId) return;
            
            setExecutions(prev => {
                const newExec = {
                    id: event.id,
                    tool: event.tool,
                    args: event.args,
                    status: 'running',
                    startTime: Date.now(),
                    children: []
                };

                // If this is a child execution, add to parent
                if (event.parentId) {
                    return addChildExecution(prev, event.parentId, newExec);
                }

                return [...prev, newExec];
            });

            // Auto-expand new executions
            setExpanded(prev => new Set([...prev, event.id]));
        };

        const handleToolComplete = (event) => {
            if (event.sessionId !== sessionId) return;
            
            setExecutions(prev => updateExecution(prev, event.id, {
                status: 'success',
                result: event.result,
                endTime: Date.now(),
                duration: event.duration
            }));
        };

        const handleToolError = (event) => {
            if (event.sessionId !== sessionId) return;
            
            setExecutions(prev => updateExecution(prev, event.id, {
                status: 'error',
                error: event.error,
                endTime: Date.now(),
                duration: Date.now() - (findExecution(prev, event.id)?.startTime || Date.now())
            }));
        };

        window.addEventListener('tool:start', handleToolStart);
        window.addEventListener('tool:complete', handleToolComplete);
        window.addEventListener('tool:error', handleToolError);

        return () => {
            window.removeEventListener('tool:start', handleToolStart);
            window.removeEventListener('tool:complete', handleToolComplete);
            window.removeEventListener('tool:error', handleToolError);
        };
    }, [sessionId]);

    // Auto-scroll to bottom when new executions are added
    useEffect(() => {
        if (autoScroll && containerRef.current) {
            containerRef.current.scrollTop = containerRef.current.scrollHeight;
        }
    }, [executions, autoScroll]);

    const toggleExpand = (id) => {
        setExpanded(prev => {
            const next = new Set(prev);
            if (next.has(id)) {
                next.delete(id);
            } else {
                next.add(id);
            }
            return next;
        });
    };

    const clearExecutions = () => {
        setExecutions([]);
        setExpanded(new Set());
    };

    return html`
        <div class="tool-execution-viz">
            <div class="viz-header">
                <h3>
                    ${icons.tool}
                    <span>Tool Execution</span>
                </h3>
                <div class="viz-controls">
                    <label class="auto-scroll-toggle">
                        <input
                            type="checkbox"
                            checked=${autoScroll}
                            onChange=${(e) => setAutoScroll(e.target.checked)}
                        />
                        Auto-scroll
                    </label>
                    <button
                        class="btn-clear"
                        onClick=${clearExecutions}
                        disabled=${executions.length === 0}
                    >
                        ${icons.trash} Clear
                    </button>
                </div>
            </div>

            <div class="viz-container" ref=${containerRef}>
                ${executions.length === 0 ? html`
                    <div class="viz-empty">
                        <div class="empty-icon">${icons.tool}</div>
                        <p>No tool executions yet</p>
                        <p class="empty-hint">Tool calls will appear here in real-time</p>
                    </div>
                ` : html`
                    <div class="execution-tree">
                        ${executions.map(exec => html`
                            <${ExecutionNode}
                                key=${exec.id}
                                execution=${exec}
                                expanded=${expanded}
                                onToggle=${toggleExpand}
                                level=${0}
                            />
                        `)}
                    </div>
                `}
            </div>
        </div>
    `;
}

/**
 * Individual execution node component
 */
function ExecutionNode({ execution, expanded, onToggle, level }) {
    const isExpanded = expanded.has(execution.id);
    const hasChildren = execution.children && execution.children.length > 0;
    const indent = level * 20;

    const statusIcon = {
        pending: icons.clock,
        running: icons.spinner,
        success: icons.check,
        error: icons.x
    }[execution.status];

    const statusClass = `status-${execution.status}`;

    return html`
        <div class="execution-node ${statusClass}" style="margin-left: ${indent}px">
            <div class="node-header" onClick=${() => hasChildren && onToggle(execution.id)}>
                <span class="expand-icon">
                    ${hasChildren ? (isExpanded ? '▼' : '▶') : ''}
                </span>
                <span class="status-icon">${statusIcon}</span>
                <span class="tool-name">${execution.tool}</span>
                ${execution.duration && html`
                    <span class="duration">${formatDuration(execution.duration)}</span>
                `}
                ${execution.status === 'running' && html`
                    <span class="running-indicator">Running...</span>
                `}
            </div>

            ${isExpanded && html`
                <div class="node-details">
                    ${execution.args && Object.keys(execution.args).length > 0 && html`
                        <div class="detail-section">
                            <div class="detail-label">Arguments:</div>
                            <pre class="detail-content">${JSON.stringify(execution.args, null, 2)}</pre>
                        </div>
                    `}

                    ${execution.result && html`
                        <div class="detail-section">
                            <div class="detail-label">Result:</div>
                            <pre class="detail-content">${formatResult(execution.result)}</pre>
                        </div>
                    `}

                    ${execution.error && html`
                        <div class="detail-section error">
                            <div class="detail-label">Error:</div>
                            <pre class="detail-content">${execution.error}</pre>
                        </div>
                    `}
                </div>
            `}

            ${hasChildren && isExpanded && html`
                <div class="node-children">
                    ${execution.children.map(child => html`
                        <${ExecutionNode}
                            key=${child.id}
                            execution=${child}
                            expanded=${expanded}
                            onToggle=${onToggle}
                            level=${level + 1}
                        />
                    `)}
                </div>
            `}
        </div>
    `;
}

/**
 * Helper functions
 */

function addChildExecution(executions, parentId, child) {
    return executions.map(exec => {
        if (exec.id === parentId) {
            return {
                ...exec,
                children: [...exec.children, child]
            };
        }
        if (exec.children.length > 0) {
            return {
                ...exec,
                children: addChildExecution(exec.children, parentId, child)
            };
        }
        return exec;
    });
}

function updateExecution(executions, id, updates) {
    return executions.map(exec => {
        if (exec.id === id) {
            return { ...exec, ...updates };
        }
        if (exec.children.length > 0) {
            return {
                ...exec,
                children: updateExecution(exec.children, id, updates)
            };
        }
        return exec;
    });
}

function findExecution(executions, id) {
    for (const exec of executions) {
        if (exec.id === id) return exec;
        if (exec.children.length > 0) {
            const found = findExecution(exec.children, id);
            if (found) return found;
        }
    }
    return null;
}

function formatDuration(ms) {
    if (ms < 1000) return `${ms}ms`;
    if (ms < 60000) return `${(ms / 1000).toFixed(1)}s`;
    return `${Math.floor(ms / 60000)}m ${Math.floor((ms % 60000) / 1000)}s`;
}

function formatResult(result) {
    if (typeof result === 'string') {
        // Truncate long strings
        if (result.length > 500) {
            return result.substring(0, 500) + '... (truncated)';
        }
        return result;
    }
    return JSON.stringify(result, null, 2);
}

/**
 * Mount the tool execution visualizer
 */
export function mountToolExecutionViz(container, sessionId) {
    render(html`<${ToolExecutionViz} sessionId=${sessionId} />`, container);
}

/**
 * Emit tool execution events (to be called from websocket handler)
 */
export function emitToolEvent(type, data) {
    window.dispatchEvent(new CustomEvent(`tool:${type}`, { detail: data }));
}
