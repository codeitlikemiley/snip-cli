use crate::models::snippet_model::Snippet;
use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn update_key_in_file(file_path: &str, old_key: String, new_key: String) -> anyhow::Result<()> {
    if old_key == new_key {
        return Err(anyhow::anyhow!("The new key is the same as the old key"));
    }

    let path = Path::new(file_path);

    let mut snippets: HashMap<String, Snippet> = if path.exists() && path.metadata()?.len() > 0 {
        let file_contents =
            fs::read_to_string(file_path).context("Failed to read the snippets file")?;
        serde_json::from_str(&file_contents).context("Failed to parse the snippets file")?
    } else {
        return Err(anyhow::anyhow!("Snippet file not found or is empty"));
    };

    if snippets.contains_key(&new_key) {
        return Err(anyhow::anyhow!(
            "A snippet with key '{}' already exists",
            new_key
        ));
    }

    let snippet = snippets
        .remove(&old_key)
        .ok_or_else(|| anyhow::anyhow!("Snippet with key '{}' not found for updating", old_key))?;

    snippets.insert(new_key, snippet);

    let updated_contents =
        serde_json::to_string_pretty(&snippets).context("Failed to serialize snippets")?;
    fs::write(file_path, updated_contents).context("Failed to write to the snippets file")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use tempfile::NamedTempFile;

    #[test]
    fn test_update_key_in_file() -> Result<()> {
        // Setup a temporary file
        let temp_file = NamedTempFile::new()?;
        let file_path = temp_file.path().to_str().unwrap();

        // Create a snippet and write it to the temp file
        let initial_key = "old_key";
        let new_key = "new_key";
        let mut snippets = HashMap::new();
        snippets.insert(
            initial_key.to_string(),
            Snippet {
                prefix: "test_prefix".to_string(),
                body: vec!["test_body".to_string()],
                description: "test_description".to_string(),
            },
        );
        let contents = serde_json::to_string(&snippets)?;
        fs::write(file_path, contents)?;

        // Call the function to update the key
        update_key_in_file(file_path, initial_key.to_string(), new_key.to_string())?;

        // Read the file and check if the key was updated
        let updated_contents = fs::read_to_string(file_path)?;
        let updated_snippets: HashMap<String, Snippet> = serde_json::from_str(&updated_contents)?;

        // Assertions
        assert!(updated_snippets.contains_key(new_key));
        assert!(!updated_snippets.contains_key(initial_key));

        Ok(())
    }
}
