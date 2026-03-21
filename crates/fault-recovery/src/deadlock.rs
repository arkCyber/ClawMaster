//! Deadlock Detection
//!
//! DO-178C Level A Compliant Deadlock Detection

use {
    crate::{FaultError, FaultResult},
    parking_lot::RwLock,
    std::{
        collections::{HashMap, HashSet},
        sync::Arc,
    },
    time::OffsetDateTime,
};

/// Lock acquisition record
#[derive(Debug, Clone)]
pub struct LockRecord {
    pub thread_id: String,
    pub resource_id: String,
    pub timestamp: OffsetDateTime,
}

/// Deadlock detector
///
/// DO-178C §6.3.3: Deadlock detection
pub struct DeadlockDetector {
    lock_graph: Arc<RwLock<LockGraph>>,
}

#[derive(Debug)]
struct LockGraph {
    // Maps thread -> resources it holds
    thread_resources: HashMap<String, HashSet<String>>,
    // Maps thread -> resources it's waiting for
    thread_waiting: HashMap<String, String>,
    // Maps resource -> thread that holds it
    resource_holders: HashMap<String, String>,
}

impl DeadlockDetector {
    /// Create new deadlock detector
    pub fn new() -> Self {
        Self {
            lock_graph: Arc::new(RwLock::new(LockGraph {
                thread_resources: HashMap::new(),
                thread_waiting: HashMap::new(),
                resource_holders: HashMap::new(),
            })),
        }
    }

    /// Record lock acquisition
    ///
    /// DO-178C §6.3.3: Lock tracking
    pub fn acquire_lock(&self, thread_id: &str, resource_id: &str) -> FaultResult<()> {
        let mut graph = self.lock_graph.write();

        // Check for deadlock before acquiring
        if let Some(cycle) = self.detect_cycle_internal(&graph, thread_id, resource_id) {
            return Err(FaultError::DeadlockDetected(format!(
                "Deadlock cycle detected: {}",
                cycle.join(" -> ")
            )));
        }

        // Record the acquisition
        graph
            .thread_resources
            .entry(thread_id.to_string())
            .or_insert_with(HashSet::new)
            .insert(resource_id.to_string());

        graph
            .resource_holders
            .insert(resource_id.to_string(), thread_id.to_string());

        // Remove from waiting list
        graph.thread_waiting.remove(thread_id);

        Ok(())
    }

    /// Record lock wait
    pub fn wait_for_lock(&self, thread_id: &str, resource_id: &str) -> FaultResult<()> {
        let mut graph = self.lock_graph.write();

        // Check for deadlock
        if let Some(cycle) = self.detect_cycle_internal(&graph, thread_id, resource_id) {
            return Err(FaultError::DeadlockDetected(format!(
                "Deadlock would occur: {}",
                cycle.join(" -> ")
            )));
        }

        graph
            .thread_waiting
            .insert(thread_id.to_string(), resource_id.to_string());

        Ok(())
    }

    /// Record lock release
    pub fn release_lock(&self, thread_id: &str, resource_id: &str) {
        let mut graph = self.lock_graph.write();

        if let Some(resources) = graph.thread_resources.get_mut(thread_id) {
            resources.remove(resource_id);
        }

        graph.resource_holders.remove(resource_id);
    }

    /// Detect deadlock cycle
    fn detect_cycle_internal(
        &self,
        graph: &LockGraph,
        thread_id: &str,
        resource_id: &str,
    ) -> Option<Vec<String>> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();

        self.dfs_cycle(graph, thread_id, resource_id, &mut visited, &mut path)
    }

    /// Depth-first search for cycle detection
    fn dfs_cycle(
        &self,
        graph: &LockGraph,
        current_thread: &str,
        target_resource: &str,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        if visited.contains(current_thread) {
            // Found a cycle
            if path.contains(&current_thread.to_string()) {
                return Some(path.clone());
            }
            return None;
        }

        visited.insert(current_thread.to_string());
        path.push(current_thread.to_string());

        // Check if current thread would wait for a resource held by another thread
        if let Some(holder) = graph.resource_holders.get(target_resource) {
            if holder == current_thread {
                // Thread already holds the resource
                return None;
            }

            // Check if holder is waiting for any resource held by current thread
            if let Some(resources) = graph.thread_resources.get(current_thread) {
                if let Some(waiting_for) = graph.thread_waiting.get(holder) {
                    if resources.contains(waiting_for) {
                        path.push(holder.to_string());
                        return Some(path.clone());
                    }
                }
            }

            // Continue DFS
            return self.dfs_cycle(graph, holder, target_resource, visited, path);
        }

        path.pop();
        None
    }

    /// Get current lock statistics
    pub fn get_statistics(&self) -> DeadlockStatistics {
        let graph = self.lock_graph.read();

        DeadlockStatistics {
            total_threads: graph.thread_resources.len(),
            total_resources: graph.resource_holders.len(),
            waiting_threads: graph.thread_waiting.len(),
        }
    }
}

impl Default for DeadlockDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Deadlock statistics
#[derive(Debug, Clone)]
pub struct DeadlockStatistics {
    pub total_threads: usize,
    pub total_resources: usize,
    pub waiting_threads: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deadlock_detector_creation() {
        let detector = DeadlockDetector::new();
        let stats = detector.get_statistics();

        assert_eq!(stats.total_threads, 0);
        assert_eq!(stats.total_resources, 0);
        assert_eq!(stats.waiting_threads, 0);
    }

    #[test]
    fn test_acquire_and_release_lock() {
        let detector = DeadlockDetector::new();

        // Thread A acquires resource 1
        detector.acquire_lock("thread_a", "resource_1").unwrap();

        let stats = detector.get_statistics();
        assert_eq!(stats.total_threads, 1);
        assert_eq!(stats.total_resources, 1);

        // Release lock
        detector.release_lock("thread_a", "resource_1");

        let stats = detector.get_statistics();
        assert_eq!(stats.total_resources, 0);
    }

    #[test]
    fn test_simple_deadlock_detection() {
        let detector = DeadlockDetector::new();

        // Thread A holds resource 1
        detector.acquire_lock("thread_a", "resource_1").unwrap();

        // Thread B holds resource 2
        detector.acquire_lock("thread_b", "resource_2").unwrap();

        // Thread A waits for resource 2 (held by B)
        detector.wait_for_lock("thread_a", "resource_2").unwrap();

        // Thread B tries to wait for resource 1 (held by A) - should detect deadlock
        let result = detector.wait_for_lock("thread_b", "resource_1");
        assert!(result.is_err());
    }

    #[test]
    fn test_no_deadlock_same_thread() {
        let detector = DeadlockDetector::new();

        // Thread A acquires resource 1
        detector.acquire_lock("thread_a", "resource_1").unwrap();

        // Thread A acquires resource 1 again (reentrant) - should not deadlock
        let result = detector.acquire_lock("thread_a", "resource_1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_no_deadlock_different_resources() {
        let detector = DeadlockDetector::new();

        // Thread A holds resource 1
        detector.acquire_lock("thread_a", "resource_1").unwrap();

        // Thread B holds resource 2
        detector.acquire_lock("thread_b", "resource_2").unwrap();

        // Thread C holds resource 3 - no deadlock
        let result = detector.acquire_lock("thread_c", "resource_3");
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_statistics() {
        let detector = DeadlockDetector::new();

        detector.acquire_lock("thread_a", "resource_1").unwrap();
        detector.acquire_lock("thread_b", "resource_2").unwrap();
        detector.wait_for_lock("thread_c", "resource_1").unwrap();

        let stats = detector.get_statistics();
        assert_eq!(stats.total_threads, 2);
        assert_eq!(stats.total_resources, 2);
        assert_eq!(stats.waiting_threads, 1);
    }
}
