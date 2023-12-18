use anyhow::{self, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use dirs_next::home_dir;
use prettytable::{row, Table};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::{
    fs::{self, OpenOptions},
    io::Write,
};

#[derive(Parser)]
#[clap(name = "rsnippet")]
#[command(author,version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Identifier {
    Key,
    Prefix,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Adds entry to Snippet Collection file
    Add {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        prefix: String,
        #[arg(short, long)]
        description: String,
        #[arg(last(true))]
        body: Vec<String>,
    },
    /// Removes entry from Snippet Collection file
    Rm { key: String },
    /// Edits entry in Snippet Collection file
    Edit {
        #[arg(short, long)]
        key: String,
        #[arg(short, long)]
        prefix: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(last(true))]
        body: Option<Vec<String>>,
    },
    /// Lists all entries in Snippet Collection file
    Ls {
        #[arg(value_enum)]
        list_option: Identifier,
    },
    /// Gets entry from Snippet Collection file
    Show { key: String },
    /// Searches for entries in Snippet Collection file
    Search {
        #[arg(value_enum)]
        id: Option<Identifier>,
        #[arg(last(true))]
        name: String,
    },
    /// Configures the Snippet Collection file
    Config { path: Option<String> },
    UpdateKey {
        #[arg(short, long)]
        old_key: String,
        #[arg(short, long)]
        new_key: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Snippet {
    pub prefix: String,
    pub body: Vec<String>,
    pub description: String,
}

const DEFAULT_SNIPPET_PATH: &str = "~/.config/nvim/snippets/rust/rust.json";
const DEFAULT_CONFIG_PATH: &str = "~/.config/rsnippet/config.json";

fn expand_home_dir(path: &str) -> Option<PathBuf> {
    if path.starts_with("~/") {
        home_dir().map(|mut home| {
            home.push(&path[2..]);
            home
        })
    } else {
        Some(PathBuf::from(path))
    }
}

fn create_directory_and_file(file_path: &str) -> Result<()> {
    let path = Path::new(file_path);

    // Create the directory where the file will reside
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).context("Failed to create directory")?;
    }

    // Create the file (or open if it already exists)
    OpenOptions::new()
        .write(true)
        .create(true) // This will create the file if it doesn't exist
        .open(file_path)
        .context("Failed to create or open the file")?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct RSnippetConfig {
    path: String,
}

impl RSnippetConfig {
    fn load(path: &str) -> Result<RSnippetConfig> {
        let config_content = fs::read_to_string(path).context("Failed to read config file")?;
        let config: RSnippetConfig =
            serde_json::from_str(&config_content).context("Failed to parse config file")?;
        writeln!(std::io::stdout(), "{}", &config.path).unwrap();
        Ok(config)
    }

    fn save(&self, path: &str) -> Result<()> {
        let config_content =
            serde_json::to_string_pretty(self).context("Failed to serialize config")?;
        fs::write(path, config_content).context("Failed to write config file")?;
        Ok(())
    }

    fn update_path(&mut self, new_path: String) {
        self.path = expand_home_dir(&new_path)
            .expect("Failed to find home directory")
            .to_string_lossy()
            .into_owned();
    }
}

fn write_snippet_to_file(
    file_path: &str,
    key: String,
    prefix: String,
    description: String,
    body: Vec<String>,
) -> Result<()> {
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

fn remove_snippet_from_file(file_path: &str, key: &str) -> Result<()> {
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

fn edit_snippet_in_file(
    file_path: &str,
    key: String,
    new_prefix: Option<String>,
    new_description: Option<String>,
    new_body: Option<Vec<String>>,
) -> Result<()> {
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

fn list_snippets(file_path: &str, list_option: Identifier) -> Result<String> {
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

pub fn show_snippet(file_path: &str, key: String) -> Result<String> {
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

pub fn search_snippets(
    file_path: &str,
    id: Option<Identifier>,
    name: String,
) -> Result<Vec<String>> {
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
                if is_fuzzy_match(key, &name) {
                    results.push(key.clone());
                }
            }
        }
        Some(Identifier::Prefix) => {
            for snippet in snippets.values() {
                if is_fuzzy_match(&snippet.prefix, &name) {
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

fn is_fuzzy_match(text: &str, pattern: &str) -> bool {
    let mut pattern_chars = pattern.chars().peekable();
    for ch in text.chars() {
        if let Some(&next_pattern_char) = pattern_chars.peek() {
            if ch == next_pattern_char {
                pattern_chars.next();
            }
        }
    }
    pattern_chars.peek().is_none()
}

fn update_key_in_file(file_path: &str, old_key: String, new_key: String) -> Result<()> {
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

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = expand_home_dir(DEFAULT_CONFIG_PATH)
        .expect("Failed to find home directory")
        .to_string_lossy()
        .into_owned();

    // Ensure the config directory exists
    create_directory_and_file(&config_path)?;

    // Load or create the configuration
    let config = match RSnippetConfig::load(&config_path) {
        Ok(cfg) => cfg,
        Err(_) => {
            let default_snippet_path = expand_home_dir(DEFAULT_SNIPPET_PATH)
                .expect("Failed to find home directory")
                .to_string_lossy()
                .into_owned();
            let new_config = RSnippetConfig {
                path: default_snippet_path,
            };
            new_config.save(&config_path)?;
            new_config
        }
    };

    let cli = Cli::parse();
    match cli.command {
        Commands::Add {
            key,
            prefix,
            description,
            body,
        } => {
            dbg!(&key, &prefix, &description, &body);
            write_snippet_to_file(&config.path, key, prefix, description, body)
                .context("Failed to write snippet to file")?;
        }
        Commands::Rm { key } => {
            dbg!(&key);
            remove_snippet_from_file(&config.path, &key)
                .context("Failed to remove snippet from file")?;
        }
        Commands::Ls { list_option } => {
            dbg!(list_option);
            let output = list_snippets(&config.path, list_option)
                .context("Failed to list snippets from file")?;
            writeln!(std::io::stdout(), "{}", output).unwrap();
        }
        Commands::Edit {
            key,
            prefix,
            description,
            body,
        } => {
            dbg!(&key, &prefix, &description, &body);
            edit_snippet_in_file(&config.path, key, prefix, description, body)
                .context("Failed to edit snippet in file")?;
        }
        Commands::Show { key } => {
            dbg!(&key);
            let output =
                show_snippet(&config.path, key).context("Failed to show snippet from file")?;
            writeln!(std::io::stdout(), "{}", output).unwrap();
        }
        Commands::Search { id, name } => {
            dbg!(id, &name);
            let output = search_snippets(&config.path, id, name)
                .context("Failed to search snippet from file")?;

            for result in output {
                writeln!(std::io::stdout(), "{}\n", result).unwrap();
            }
        }
        Commands::UpdateKey { old_key, new_key } => {
            dbg!(&old_key, &new_key);

            update_key_in_file(&config.path, old_key, new_key)
                .context("Failed to update key in file")?;
        }
        Commands::Config { path } => {
            dbg!(&path);
            if let Some(path) = path {
                let mut config = config;
                config.update_path(path);
                config.save(&config_path)?;
                println!("Configuration updated.");
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use prettytable::{row, Table};
    use std::fs;
    use std::io::{Read, Seek, SeekFrom, Write};
    use tempfile::NamedTempFile;

    #[test]
    fn test_create_directory_and_file() {
        let test_path = "test_data/test_file.txt";

        // Ensure the test environment is clean
        let _ = fs::remove_dir_all("test_data");

        // Run the function
        let result = create_directory_and_file(test_path);
        assert!(result.is_ok());

        // Check if the directory was created
        assert!(Path::new("test_data").exists());

        // Check if the file was created
        assert!(Path::new(test_path).exists());

        // Clean up the test environment
        let _ = fs::remove_dir_all("test_data");
    }

    #[test]
    fn test_write_snippet_to_file() {
        let test_path = "test_rsnippet.json";
        let test_key = "test_key".to_string();
        let test_prefix = "test_prefix".to_string();
        let test_description = "test_description".to_string();
        let test_body = vec!["test_body".to_string()];

        // Write test data to the file
        write_snippet_to_file(
            test_path,
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

        // Clean up: remove the test file
        fs::remove_file(test_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_write_with_duplicate_key() {
        let test_path = "test_duplicate_key.json";
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
        fs::write(test_path, initial_contents).unwrap();

        // Attempt to write another snippet with the same key
        let result = write_snippet_to_file(
            test_path,
            test_key,
            "another_prefix".to_string(),
            "another_description".to_string(),
            vec!["another_body".to_string()],
        );
        assert!(result.is_err());

        // Cleanup
        fs::remove_file(test_path).unwrap();
    }

    #[test]
    fn test_write_with_duplicate_prefix() {
        let test_path = "test_duplicate_prefix.json";
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
        fs::write(test_path, initial_contents).unwrap();

        // Attempt to write another snippet with the same prefix
        let result = write_snippet_to_file(
            test_path,
            "test_key_2".to_string(),
            test_prefix,
            "new_description".to_string(),
            vec!["new_body".to_string()],
        );
        assert!(result.is_err());

        // Cleanup
        fs::remove_file(test_path).unwrap();
    }

    #[test]
    fn test_remove_snippet_key_not_found() {
        let test_path = "test_remove_snippet.json";
        let existing_key = "existing_key".to_string();
        let non_existing_key = "non_existing_key".to_string();
        let test_prefix = "test_prefix".to_string();
        let test_description = "test_description".to_string();
        let test_body = vec!["test_body".to_string()];

        // Clean up before test
        let _ = fs::remove_file(test_path);

        // Write a snippet to the file
        let write_result = write_snippet_to_file(
            test_path,
            existing_key,
            test_prefix,
            test_description,
            test_body,
        );
        assert!(write_result.is_ok());

        // Attempt to remove a snippet with a non-existing key
        let remove_result = remove_snippet_from_file(test_path, &non_existing_key);
        assert!(remove_result.is_err());

        // Check error message
        let error_message = remove_result.unwrap_err().to_string();
        assert_eq!(
            error_message,
            format!("Snippet with key '{}' not found", non_existing_key)
        );

        // Clean up after test
        let _ = fs::remove_file(test_path);
    }

    #[test]
    fn test_edit_with_empty_fields() {
        let test_path = "test_edit_empty_fields.json";
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
        fs::write(test_path, initial_contents).unwrap();

        // Attempt to edit with empty fields
        let result = edit_snippet_in_file(test_path, test_key.clone(), None, None, None);
        assert!(result.is_err());

        // Cleanup
        fs::remove_file(test_path).unwrap();
    }

    #[test]
    fn test_edit_with_duplicate_prefix() {
        let test_path = "test_edit_duplicate_prefix.json";
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
        fs::write(test_path, initial_contents).unwrap();

        // Attempt to edit second snippet with duplicate prefix
        let result = edit_snippet_in_file(
            test_path,
            test_key_2,
            Some(duplicate_prefix),
            Some("new_description".to_string()),
            Some(vec!["new_body".to_string()]),
        );
        assert!(result.is_err());

        // Cleanup
        fs::remove_file(test_path).unwrap();
    }

    #[test]
    fn test_list_snippets_success() -> Result<()> {
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

    #[test]
    fn test_rsnippet_config_load() -> Result<()> {
        // Create a temporary file
        let mut temp_file = NamedTempFile::new()?;

        // Mock configuration data
        let mock_config = RSnippetConfig {
            path: "mock/path/to/snippets.json".to_string(),
        };
        let mock_config_json = serde_json::to_string(&mock_config)?;

        // Write mock configuration data to the temporary file
        write!(temp_file, "{}", mock_config_json)?;
        let temp_file_path = temp_file.path().to_str().unwrap();

        // Load configuration using RSnippetConfig::load
        let loaded_config = RSnippetConfig::load(temp_file_path)?;

        // Assert that loaded configuration matches the mock data
        assert_eq!(loaded_config.path, mock_config.path);

        Ok(())
    }

    #[test]
    fn test_rsnippet_config_save() -> Result<()> {
        // Create a temporary file
        let temp_file = NamedTempFile::new()?;
        let temp_file_path = temp_file.path().to_str().unwrap();

        // Create an RSnippetConfig instance
        let config = RSnippetConfig {
            path: "mock/path/to/snippets.json".to_string(),
        };

        // Save the configuration to the temporary file
        config.save(temp_file_path)?;

        // Read the contents of the temporary file
        let mut file = temp_file.reopen()?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // Deserialize the contents back to RSnippetConfig
        let saved_config: RSnippetConfig = serde_json::from_str(&contents)?;

        // Assert that the saved configuration matches the original configuration
        assert_eq!(saved_config.path, config.path);

        Ok(())
    }
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
