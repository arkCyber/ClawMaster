//! Plugin dependency resolution

use crate::plugin::PluginDependency;
use std::collections::HashMap;
use anyhow::Result;

/// Dependency resolver
pub struct DependencyResolver {
    // Plugin ID -> Version mapping
    installed_plugins: HashMap<String, String>,
}

impl DependencyResolver {
    /// Create a new dependency resolver
    pub fn new() -> Self {
        Self {
            installed_plugins: HashMap::new(),
        }
    }

    /// Register an installed plugin
    pub fn register_plugin(&mut self, plugin_id: String, version: String) {
        self.installed_plugins.insert(plugin_id, version);
    }

    /// Unregister a plugin
    pub fn unregister_plugin(&mut self, plugin_id: &str) {
        self.installed_plugins.remove(plugin_id);
    }

    /// Resolve dependencies for a plugin
    pub async fn resolve(&self, dependencies: &[PluginDependency]) -> Result<()> {
        for dep in dependencies {
            self.check_dependency(dep)?;
        }
        Ok(())
    }

    /// Check a single dependency
    fn check_dependency(&self, dep: &PluginDependency) -> Result<()> {
        // Skip optional dependencies if not installed
        if dep.optional && !self.installed_plugins.contains_key(&dep.plugin_id) {
            tracing::debug!(
                plugin_id = %dep.plugin_id,
                "optional dependency not installed, skipping"
            );
            return Ok(());
        }

        // Check if dependency is installed
        let installed_version = self.installed_plugins
            .get(&dep.plugin_id)
            .ok_or_else(|| anyhow::anyhow!(
                "required dependency not installed: {}",
                dep.plugin_id
            ))?;

        // Parse version requirement
        let version_req = semver::VersionReq::parse(&dep.version)?;
        let installed_ver = semver::Version::parse(installed_version)?;

        // Check version compatibility
        if !version_req.matches(&installed_ver) {
            anyhow::bail!(
                "dependency version mismatch: {} requires {}, but {} is installed",
                dep.plugin_id,
                dep.version,
                installed_version
            );
        }

        tracing::debug!(
            plugin_id = %dep.plugin_id,
            version = %installed_version,
            "dependency satisfied"
        );

        Ok(())
    }

    /// Topological sort of plugins based on dependencies
    pub fn topological_sort(
        &self,
        plugins: &[(String, Vec<PluginDependency>)],
    ) -> Result<Vec<String>> {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        let mut in_degree: HashMap<String, usize> = HashMap::new();

        // Build dependency graph
        for (plugin_id, dependencies) in plugins {
            in_degree.entry(plugin_id.clone()).or_insert(0);
            
            for dep in dependencies {
                if !dep.optional {
                    graph
                        .entry(dep.plugin_id.clone())
                        .or_insert_with(Vec::new)
                        .push(plugin_id.clone());
                    
                    *in_degree.entry(plugin_id.clone()).or_insert(0) += 1;
                }
            }
        }

        // Kahn's algorithm for topological sort
        let mut queue: Vec<String> = in_degree
            .iter()
            .filter(|(_, degree)| **degree == 0)
            .map(|(id, _)| id.clone())
            .collect();

        let mut result = Vec::new();

        while let Some(plugin_id) = queue.pop() {
            result.push(plugin_id.clone());

            if let Some(dependents) = graph.get(&plugin_id) {
                for dependent in dependents {
                    if let Some(degree) = in_degree.get_mut(dependent) {
                        *degree -= 1;
                        if *degree == 0 {
                            queue.push(dependent.clone());
                        }
                    }
                }
            }
        }

        // Check for circular dependencies
        if result.len() != plugins.len() {
            anyhow::bail!("circular dependency detected");
        }

        Ok(result)
    }

    /// Detect circular dependencies
    pub fn has_circular_dependency(
        &self,
        plugins: &[(String, Vec<PluginDependency>)],
    ) -> bool {
        self.topological_sort(plugins).is_err()
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dependency_resolution_success() {
        let mut resolver = DependencyResolver::new();
        resolver.register_plugin("plugin-a".to_string(), "1.0.0".to_string());

        let dependencies = vec![
            PluginDependency {
                plugin_id: "plugin-a".to_string(),
                version: "^1.0.0".to_string(),
                optional: false,
            },
        ];

        assert!(resolver.resolve(&dependencies).await.is_ok());
    }

    #[tokio::test]
    async fn test_dependency_resolution_missing() {
        let resolver = DependencyResolver::new();

        let dependencies = vec![
            PluginDependency {
                plugin_id: "plugin-a".to_string(),
                version: "^1.0.0".to_string(),
                optional: false,
            },
        ];

        assert!(resolver.resolve(&dependencies).await.is_err());
    }

    #[tokio::test]
    async fn test_dependency_resolution_version_mismatch() {
        let mut resolver = DependencyResolver::new();
        resolver.register_plugin("plugin-a".to_string(), "2.0.0".to_string());

        let dependencies = vec![
            PluginDependency {
                plugin_id: "plugin-a".to_string(),
                version: "^1.0.0".to_string(),
                optional: false,
            },
        ];

        assert!(resolver.resolve(&dependencies).await.is_err());
    }

    #[tokio::test]
    async fn test_optional_dependency() {
        let resolver = DependencyResolver::new();

        let dependencies = vec![
            PluginDependency {
                plugin_id: "plugin-a".to_string(),
                version: "^1.0.0".to_string(),
                optional: true,
            },
        ];

        assert!(resolver.resolve(&dependencies).await.is_ok());
    }

    #[test]
    fn test_topological_sort() {
        let resolver = DependencyResolver::new();

        let plugins = vec![
            ("plugin-a".to_string(), vec![]),
            (
                "plugin-b".to_string(),
                vec![PluginDependency {
                    plugin_id: "plugin-a".to_string(),
                    version: "1.0.0".to_string(),
                    optional: false,
                }],
            ),
            (
                "plugin-c".to_string(),
                vec![PluginDependency {
                    plugin_id: "plugin-b".to_string(),
                    version: "1.0.0".to_string(),
                    optional: false,
                }],
            ),
        ];

        let result = resolver.topological_sort(&plugins).unwrap();
        
        // plugin-a should come before plugin-b
        let a_pos = result.iter().position(|x| x == "plugin-a").unwrap();
        let b_pos = result.iter().position(|x| x == "plugin-b").unwrap();
        let c_pos = result.iter().position(|x| x == "plugin-c").unwrap();

        assert!(a_pos < b_pos);
        assert!(b_pos < c_pos);
    }

    #[test]
    fn test_circular_dependency_detection() {
        let resolver = DependencyResolver::new();

        let plugins = vec![
            (
                "plugin-a".to_string(),
                vec![PluginDependency {
                    plugin_id: "plugin-b".to_string(),
                    version: "1.0.0".to_string(),
                    optional: false,
                }],
            ),
            (
                "plugin-b".to_string(),
                vec![PluginDependency {
                    plugin_id: "plugin-a".to_string(),
                    version: "1.0.0".to_string(),
                    optional: false,
                }],
            ),
        ];

        assert!(resolver.has_circular_dependency(&plugins));
    }
}
