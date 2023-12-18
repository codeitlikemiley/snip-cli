use crate::models::snippet_model::Snippet;
use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn write_snippet_to_file(
    file_path: &str,
    key: String,
    prefix: String,
    description: String,
    body: Vec<String>,
) -> anyhow::Result<()> {
    let path = Path::new(file_path);

    let mut snippets: HashMap<String, Snippet> = if path.exists() && path.metadata()?.len() > 0 {
        let file_contents =
            fs::read_to_string(file_path).context("Failed to read the snippets file")?;
        serde_json::from_str(&file_contents).context("Failed to parse the snippets file")?
    } else {
        HashMap::new()
    };

    // Check for duplicate key or prefix
    if snippets.contains_key(&key) {
        return Err(anyhow::anyhow!(
            "A snippet with key '{}' already exists",
            key
        ));
    }
    if snippets.values().any(|s| s.prefix == prefix) {
        return Err(anyhow::anyhow!(
            "A snippet with prefix '{}' already exists",
            prefix
        ));
    }

    // Add the new snippet
    let new_snippet = Snippet {
        prefix,
        body,
        description,
    };
    snippets.insert(key, new_snippet);

    // Write the updated HashMap back to the JSON file
    let updated_contents =
        serde_json::to_string_pretty(&snippets).context("Failed to serialize snippets")?;
    fs::write(file_path, updated_contents).context("Failed to write to the snippets file")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use io::Write;
    use serde_json;
    use std::collections::HashMap;
    use std::{fs, io};
    use tempfile::NamedTempFile;

    #[test]
    fn test_write_snippet_to_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let test_path = temp_file.path();

        let test_key = "test_key".to_string();
        let test_prefix = "test_prefix".to_string();
        let test_description = "test_description".to_string();
        let test_body = vec!["test_body".to_string()];

        // Write test data to the file
        write_snippet_to_file(
            test_path.to_str().unwrap(),
            test_key.clone(),
            test_prefix.clone(),
            test_description.clone(),
            test_body.clone(),
        )
        .expect("Failed to write test snippet to file");

        // Read the file and deserialize the JSON content
        let file_contents = fs::read_to_string(test_path).expect("Failed to read test file");
        let snippets: HashMap<String, Snippet> =
            serde_json::from_str(&file_contents).expect("Failed to deserialize JSON content");

        // Check if the test data is correctly written
        assert!(snippets.contains_key(&test_key));
        let snippet = snippets.get(&test_key).unwrap();
        assert_eq!(snippet.prefix, test_prefix);
        assert_eq!(snippet.description, test_description);
        assert_eq!(snippet.body, test_body);

        // No need for explicit cleanup, as NamedTempFile automatically deletes the file
    }
    #[test]
    fn test_write_with_duplicate_key() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let test_path = temp_file.path().to_owned();

        let test_key = "duplicate_key".to_string();
        let test_prefix = "test_prefix".to_string();
        let test_description = "test_description".to_string();
        let test_body = vec!["test_body".to_string()];

        // Setup: Write an initial snippet with the test key
        let mut initial_snippets = HashMap::new();
        initial_snippets.insert(
            test_key.clone(),
            Snippet {
                prefix: test_prefix.clone(),
                description: test_description.clone(),
                body: test_body.clone(),
            },
        );
        let initial_contents = serde_json::to_string_pretty(&initial_snippets).unwrap();
        write!(temp_file, "{}", initial_contents).unwrap();

        // Attempt to write another snippet with the same key
        let result = write_snippet_to_file(
            test_path.to_str().unwrap(),
            test_key,
            "another_prefix".to_string(),
            "another_description".to_string(),
            vec!["another_body".to_string()],
        );
        assert!(result.is_err());

        // No need for explicit cleanup, as NamedTempFile automatically deletes the file
    }

    #[test]
    fn test_write_with_duplicate_prefix() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let test_path = temp_file.path().to_owned();

        let test_prefix = "duplicate_prefix".to_string();

        // Setup: Write an initial snippet with the test prefix
        let mut initial_snippets = HashMap::new();
        initial_snippets.insert(
            "test_key_1".to_string(),
            Snippet {
                prefix: test_prefix.clone(),
                description: "description_1".to_string(),
                body: vec!["body_1".to_string()],
            },
        );
        let initial_contents = serde_json::to_string_pretty(&initial_snippets).unwrap();
        write!(temp_file, "{}", initial_contents).unwrap();

        // Attempt to write another snippet with the same prefix
        let result = write_snippet_to_file(
            test_path.to_str().unwrap(),
            "test_key_2".to_string(),
            test_prefix,
            "new_description".to_string(),
            vec!["new_body".to_string()],
        );
        assert!(result.is_err());

        // No need for explicit cleanup, as NamedTempFile automatically deletes the file
    }
}
