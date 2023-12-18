use dirs_next::home_dir;
use std::path::PathBuf;

pub fn expand_home_dir(path: &str) -> Option<PathBuf> {
    if path.starts_with("~/") {
        home_dir().map(|mut home| {
            home.push(&path[2..]);
            home
        })
    } else {
        Some(PathBuf::from(path))
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
            Some(home.join("test")),
            "Failed to expand home directory"
        );
        assert_eq!(
            expand_home_dir("/test"),
            Some(PathBuf::from("/test")),
            "Failed to expand home directory"
        );
    }

    #[test]
    fn test_providing_pull_path_on_expand_home_dir() {
        assert_eq!(
            expand_home_dir("/Users/uriah/Code/rustacean/src"),
            Some(PathBuf::from("/Users/uriah/Code/rustacean/src")),
            "Failed to expand home directory"
        )
    }
}
