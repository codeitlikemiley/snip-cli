use crate::models::snippet_model::Snippet;
use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn edit_snippet_in_file(
    file_path: &str,
    key: String,
    new_prefix: Option<String>,
    new_description: Option<String>,
    new_body: Option<Vec<String>>,
) -> anyhow::Result<()> {
    // If all fields are None, do not proceed
    if new_prefix.is_none() && new_description.is_none() && new_body.is_none() {
        return Err(anyhow::anyhow!("No fields provided for update"));
    }

    let path = Path::new(file_path);

    let mut snippets: HashMap<String, Snippet> = if path.exists() && path.metadata()?.len() > 0 {
        let file_contents =
            fs::read_to_string(file_path).context("Failed to read the snippets file")?;
        serde_json::from_str(&file_contents).context("Failed to parse the snippets file")?
    } else {
        return Err(anyhow::anyhow!("Snippet file not found or is empty"));
    };

    // Check for duplicate prefix in other snippets
    if let Some(ref prefix) = new_prefix {
        if snippets
            .iter()
            .any(|(k, s)| k != &key && s.prefix == *prefix)
        {
            return Err(anyhow::anyhow!(
                "A snippet with prefix '{}' already exists",
                prefix
            ));
        }
    }

    // Get mutable reference to the snippet after checking for duplicates
    let snippet = snippets
        .get_mut(&key)
        .ok_or_else(|| anyhow::anyhow!("Snippet with key '{}' not found", key))?;

    // Update the snippet's fields if new values are provided
    if let Some(prefix) = new_prefix {
        snippet.prefix = prefix;
    }
    if let Some(description) = new_description {
        snippet.description = description;
    }
    if let Some(body) = new_body {
        snippet.body = body;
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
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_edit_with_empty_fields() {
        let temp_file = NamedTempFile::new().unwrap();
        let test_path = temp_file.path();

        let test_key = "test_key".to_string();
        let initial_snippet = Snippet {
            prefix: "initial_prefix".to_string(),
            description: "initial_description".to_string(),
            body: vec!["initial_body".to_string()],
        };

        // Setup: Write an initial snippet
        let mut initial_snippets = HashMap::new();
        initial_snippets.insert(test_key.clone(), initial_snippet);
        let initial_contents = serde_json::to_string_pretty(&initial_snippets).unwrap();
        write!(&temp_file, "{}", initial_contents).unwrap();

        // Attempt to edit with empty fields
        let result = edit_snippet_in_file(test_path.to_str().unwrap(), test_key, None, None, None);
        assert!(result.is_err());

        // No need for explicit cleanup, as NamedTempFile automatically deletes the file
    }
    #[test]
    fn test_edit_with_duplicate_prefix() {
        let temp_file = NamedTempFile::new().unwrap();
        let test_path = temp_file.path();

        let test_key_1 = "test_key_1".to_string();
        let test_key_2 = "test_key_2".to_string();
        let duplicate_prefix = "duplicate_prefix".to_string();

        // Setup: Write two initial snippets, one with the duplicate prefix
        let mut initial_snippets = HashMap::new();
        initial_snippets.insert(
            test_key_1.clone(),
            Snippet {
                prefix: duplicate_prefix.clone(),
                description: "description_1".to_string(),
                body: vec!["body_1".to_string()],
            },
        );
        initial_snippets.insert(
            test_key_2.clone(),
            Snippet {
                prefix: "prefix_2".to_string(),
                description: "description_2".to_string(),
                body: vec!["body_2".to_string()],
            },
        );

        let initial_contents = serde_json::to_string_pretty(&initial_snippets).unwrap();
        write!(&temp_file, "{}", initial_contents).unwrap();

        // Attempt to edit second snippet with duplicate prefix
        let result = edit_snippet_in_file(
            test_path.to_str().unwrap(),
            test_key_2.clone(),
            Some(duplicate_prefix),
            Some("new_description".to_string()),
            Some(vec!["new_body".to_string()]),
        );
        assert!(result.is_err());

        // No need for explicit cleanup, as NamedTempFile automatically deletes the file
    }
}
