use crate::helpers::is_fuzzy_match;
use crate::models::identifier_model::Identifier;
use crate::models::snippet_model::Snippet;
use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn search_snippets(
    file_path: &str,
    id: Option<Identifier>,
    name: String,
) -> anyhow::Result<Vec<String>> {
    let path = Path::new(file_path);

    let snippets: HashMap<String, Snippet> = if path.exists() && path.metadata()?.len() > 0 {
        let file_contents =
            fs::read_to_string(file_path).context("Failed to read the snippets file")?;
        serde_json::from_str(&file_contents).context("Failed to parse the snippets file")?
    } else {
        return Err(anyhow::anyhow!("Snippet file not found or is empty"));
    };

    let mut results = Vec::new();

    match id {
        Some(Identifier::Key) => {
            for (key, _) in snippets.iter() {
                if is_fuzzy_match::is_fuzzy_match(key, &name) {
                    results.push(key.clone());
                }
            }
        }
        Some(Identifier::Prefix) => {
            for snippet in snippets.values() {
                if is_fuzzy_match::is_fuzzy_match(&snippet.prefix, &name) {
                    results.push(snippet.prefix.clone());
                }
            }
        }
        None => {
            for (key, snippet) in snippets.iter() {
                if snippet
                    .description
                    .to_lowercase()
                    .contains(&name.to_lowercase())
                {
                    results.push(key.clone());
                }
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::io::{Seek, SeekFrom, Write};
    use tempfile::NamedTempFile;

    #[test]
    fn test_search_snippets() -> Result<()> {
        // Create a temporary file
        let mut temp_file = NamedTempFile::new()?;

        // Create test data
        let mut test_snippets = HashMap::new();
        test_snippets.insert(
            "test_key1".to_string(),
            Snippet {
                prefix: "test_prefix1".to_string(),
                body: vec!["test_body1".to_string()],
                description: "A test description 1".to_string(),
            },
        );
        test_snippets.insert(
            "test_key2".to_string(),
            Snippet {
                prefix: "example_prefix2".to_string(),
                body: vec!["test_body2".to_string()],
                description: "Another test description 2".to_string(),
            },
        );

        // Write test data to the temporary file
        let test_data = serde_json::to_string(&test_snippets)?;
        write!(temp_file, "{}", test_data)?;
        temp_file.flush()?; // Make sure all data is written
        temp_file.seek(SeekFrom::Start(0))?; // Reset file pointer to start

        // Get the file path after writing to the file
        let file_path = temp_file.path().to_str().unwrap();

        // Test search by key
        let result_key = search_snippets(file_path, Some(Identifier::Key), "key1".to_string())?;
        assert_eq!(result_key, vec!["test_key1"]);

        // Test search by prefix
        let result_prefix =
            search_snippets(file_path, Some(Identifier::Prefix), "example".to_string())?;
        assert_eq!(result_prefix, vec!["example_prefix2"]);

        // Test search by description
        let result_desc = search_snippets(file_path, None, "description 2".to_string())?;
        assert_eq!(result_desc, vec!["test_key2"]);

        Ok(())
    }
}
