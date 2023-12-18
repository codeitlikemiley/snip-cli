use crate::models::snippet_model::Snippet;
use anyhow::Context;
use prettytable::{row, Table};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn show_snippet(file_path: &str, key: String) -> anyhow::Result<String> {
    let path = Path::new(file_path);

    let snippets: HashMap<String, Snippet> = if path.exists() && path.metadata()?.len() > 0 {
        let file_contents =
            fs::read_to_string(file_path).context("Failed to read the snippets file")?;
        serde_json::from_str(&file_contents).context("Failed to parse the snippets file")?
    } else {
        return Err(anyhow::anyhow!("Snippet file not found or is empty"));
    };

    if let Some(snippet) = snippets.get(&key) {
        let mut table = Table::new();
        table.add_row(row!["Key", key]);
        table.add_row(row!["Prefix", snippet.prefix]);
        table.add_row(row!["Description", snippet.description]);
        table.add_row(row!["Body", snippet.body.join("\n")]);

        Ok(table.to_string())
    } else {
        Err(anyhow::anyhow!("Snippet with key '{}' not found", key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::io::{Seek, SeekFrom, Write};
    use tempfile::NamedTempFile;

    #[test]
    fn test_show_snippet() -> Result<()> {
        // Create a temporary file
        let mut temp_file = NamedTempFile::new()?;

        // Create test data
        let mut test_snippets = HashMap::new();
        test_snippets.insert(
            "test_key".to_string(),
            Snippet {
                prefix: "test_prefix".to_string(),
                body: vec!["test_body".to_string()],
                description: "test_description".to_string(),
            },
        );

        // Write test data to the temporary file
        let test_data = serde_json::to_string(&test_snippets)?;
        write!(temp_file, "{}", test_data)?;
        temp_file.flush()?; // Ensure all data is written to the file
        temp_file.seek(SeekFrom::Start(0))?; // Reset file pointer to the start

        // Get the file path after writing to the file
        let file_path = temp_file.path().to_str().unwrap();

        // Call show_snippet function
        let result = show_snippet(file_path, "test_key".to_string())?;

        // Expected output
        let mut expected_table = Table::new();
        expected_table.add_row(row!["Key", "test_key"]);
        expected_table.add_row(row!["Prefix", "test_prefix"]);
        expected_table.add_row(row!["Description", "test_description"]);
        expected_table.add_row(row!["Body", "test_body"]);
        let expected_output = expected_table.to_string();

        // Assert
        assert_eq!(result, expected_output);

        Ok(())
    }
}
