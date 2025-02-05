use anyhow::{Result, anyhow};
use percent_encoding::percent_decode_str;
use std::path::{Path, Component};

/// Sanitizes the requested path to prevent directory traversal attacks.
/// It decodes URL-encoded characters and ensures the path does not contain any parent directory ("..") references.
pub fn sanitize_path(path: &str) -> Result<String> {
    let decoded = percent_decode_str(path).decode_utf8_lossy();
    let path = Path::new(&*decoded);
    for component in path.components() {
        if let Component::ParentDir = component {
            return Err(anyhow!("Invalid path: directory traversal is not allowed"));
        }
    }
    Ok(path.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sanitize_valid_path() {
        let input = "/about";
        let sanitized = sanitize_path(input).unwrap();
        assert_eq!(sanitized, "/about");
    }
    
    #[test]
    fn test_sanitize_directory_traversal() {
        let input = "/../secret";
        assert!(sanitize_path(input).is_err());
    }
}
