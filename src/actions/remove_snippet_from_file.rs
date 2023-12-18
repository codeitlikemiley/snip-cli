use crate::models::snippet_model::Snippet;
use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn remove_snippet_from_file(file_path: &str, key: &str) -> anyhow::Result<()> {
    let path = Path::new(file_path);

    // Check if file exists and is not empty
    let mut snippets: HashMap<String, Snippet> = if path.exists() && path.metadata()?.len() > 0 {
        let file_contents =
            fs::read_to_string(file_path).context("Failed to read the snippets file")?;
        serde_json::from_str(&file_contents).context("Failed to parse the snippets file")?
    } else {
        return Err(anyhow::anyhow!("No snippet file found or file is empty"));
    };

    // Remove the snippet with the given key
    if snippets.remove(key).is_none() {
        return Err(anyhow::anyhow!("Snippet with key '{}' not found", key));
    }

    // Write the updated HashMap back to the JSON file
    let updated_contents =
        serde_json::to_string_pretty(&snippets).context("Failed to serialize snippets")?;
    fs::write(file_path, updated_contents).context("Failed to write to the snippets file")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::write_snippet_to_file::write_snippet_to_file;
    use tempfile::NamedTempFile;

    #[test]
    fn test_remove_snippet() {
        let temp_file = NamedTempFile::new().unwrap();
        let test_path = temp_file.path();

        let existing_key = "existing_key".to_string();
        let non_existing_key = "non_existing_key".to_string();
        let test_prefix = "test_prefix".to_string();
        let test_description = "test_description".to_string();
        let test_body = vec!["test_body".to_string()];

        // Write a snippet to the file
        let write_result = write_snippet_to_file(
            test_path.to_str().unwrap(),
            existing_key,
            test_prefix,
            test_description,
            test_body,
        );
        assert!(write_result.is_ok());

        // Attempt to remove a snippet with a non-existing key
        let remove_result =
            remove_snippet_from_file(test_path.to_str().unwrap(), &non_existing_key);
        assert!(remove_result.is_err());

        // Check error message
        let error_message = remove_result.unwrap_err().to_string();
        assert_eq!(
            error_message,
            format!("Snippet with key '{}' not found", non_existing_key)
        );

        // No need for explicit cleanup, as NamedTempFile automatically deletes the file
    }
}
