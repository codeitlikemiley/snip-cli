use dirs_next::home_dir;
use std::path::PathBuf;

pub fn expand_home_dir(path: &str) -> PathBuf {
    if path.starts_with("~") {
        if let Some(mut home) = home_dir() {
            home.push(&path[2..]);
            home
        } else {
            // Fallback: use a temporary directory or another suitable default
            PathBuf::from("/tmp/snip.json")
        }
    } else {
        PathBuf::from(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_home_dir() {
        let home = home_dir().unwrap();
        assert_eq!(
            expand_home_dir("~/test"),
            home.join("test"),
            "Failed to expand home directory"
        );
        assert_eq!(
            expand_home_dir("/test"),
            PathBuf::from("/test"),
            "Failed to expand home directory"
        );
    }

    #[test]
    fn test_providing_pull_path_on_expand_home_dir() {
        assert_eq!(
            expand_home_dir("/Users/uriah/Code/rustacean/src"),
            PathBuf::from("/Users/uriah/Code/rustacean/src"),
            "Failed to expand home directory"
        )
    }
}
