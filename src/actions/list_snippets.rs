use crate::models::identifier_model::Identifier;
use crate::models::snippet_model::Snippet;
use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn list_snippets(file_path: &str, list_option: Identifier) -> anyhow::Result<String> {
    let path = Path::new(file_path);

    let snippets: HashMap<String, Snippet> = if path.exists() && path.metadata()?.len() > 0 {
        let file_contents =
            fs::read_to_string(file_path).context("Failed to read the snippets file")?;
        serde_json::from_str(&file_contents).context("Failed to parse the snippets file")?
    } else {
        return Err(anyhow::anyhow!("Snippet file not found or is empty"));
    };
    let mut output = String::new();
    match list_option {
        Identifier::Key => {
            for key in snippets.keys() {
                output.push_str(&format!("{}\n", key));
            }
        }
        Identifier::Prefix => {
            for snippet in snippets.values() {
                output.push_str(&format!("{}\n", snippet.prefix));
            }
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::collections::HashMap;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_list_snippets() -> Result<()> {
        let test_file = NamedTempFile::new()?;
        let test_path = test_file.path().to_str().unwrap();

        // Setup test snippets
        let snippets = HashMap::from([
            (
                "key1".to_string(),
                Snippet {
                    prefix: "prefix1".to_string(),
                    description: "desc1".to_string(),
                    body: vec!["body1".to_string()],
                },
            ),
            (
                "key2".to_string(),
                Snippet {
                    prefix: "prefix2".to_string(),
                    description: "desc2".to_string(),
                    body: vec!["body2".to_string()],
                },
            ),
        ]);
        fs::write(test_path, serde_json::to_string(&snippets)?)?;

        // Call the list_snippets function
        let output = list_snippets(test_path, Identifier::Key)?;

        // Assert the results
        assert!(output.contains("key1"));
        assert!(output.contains("key2"));

        Ok(())
    }
}
