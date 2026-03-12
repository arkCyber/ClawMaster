//! Path operations tool for Moltis AI agents.

wit_bindgen::generate!({
    world: "path-ops",
    exports: {
        world: PathOpsTool,
    },
});

struct PathOpsTool;

impl Guest for PathOpsTool {
    fn join(parts: Vec<String>) -> String {
        parts.join("/")
    }
    
    fn basename(path: String) -> String {
        path.rsplit('/').next().unwrap_or("").to_string()
    }
    
    fn dirname(path: String) -> String {
        match path.rfind('/') {
            Some(pos) => path[..pos].to_string(),
            None => String::new(),
        }
    }
    
    fn extension(path: String) -> String {
        let basename = Self::basename(path);
        match basename.rfind('.') {
            Some(pos) if pos > 0 => basename[pos + 1..].to_string(),
            _ => String::new(),
        }
    }
    
    fn normalize(path: String) -> String {
        let mut parts = Vec::new();
        
        for part in path.split('/') {
            match part {
                "" | "." => continue,
                ".." => {
                    parts.pop();
                }
                _ => parts.push(part),
            }
        }
        
        let normalized = parts.join("/");
        if path.starts_with('/') {
            format!("/{}", normalized)
        } else {
            normalized
        }
    }
    
    fn is_absolute(path: String) -> bool {
        path.starts_with('/')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_join() {
        let result = PathOpsTool::join(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        assert_eq!(result, "a/b/c");
    }
    
    #[test]
    fn test_basename() {
        assert_eq!(PathOpsTool::basename("/path/to/file.txt".to_string()), "file.txt");
        assert_eq!(PathOpsTool::basename("file.txt".to_string()), "file.txt");
    }
    
    #[test]
    fn test_dirname() {
        assert_eq!(PathOpsTool::dirname("/path/to/file.txt".to_string()), "/path/to");
        assert_eq!(PathOpsTool::dirname("file.txt".to_string()), "");
    }
    
    #[test]
    fn test_extension() {
        assert_eq!(PathOpsTool::extension("/path/to/file.txt".to_string()), "txt");
        assert_eq!(PathOpsTool::extension("file".to_string()), "");
    }
    
    #[test]
    fn test_normalize() {
        assert_eq!(PathOpsTool::normalize("/a/./b/../c".to_string()), "/a/c");
        assert_eq!(PathOpsTool::normalize("a/./b/../c".to_string()), "a/c");
    }
    
    #[test]
    fn test_is_absolute() {
        assert!(PathOpsTool::is_absolute("/path/to/file".to_string()));
        assert!(!PathOpsTool::is_absolute("path/to/file".to_string()));
    }
}
