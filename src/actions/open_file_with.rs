use anyhow::{Context, Result};
use opener;
use std::{fs, process::Command};

/// Opens the file specified in the configuration JSON file using the system's default text editor.
///
/// # Arguments
///
/// * `config_path` - A string slice that holds the path to the JSON configuration file.
///
/// # Returns
///
/// * `Result<()>` - Returns `Ok` if the file is successfully opened, otherwise returns an error.
pub fn open_file_with(config_path: &str, editor: Option<String>) -> Result<()> {
    // Read the configuration file
    let config_content = fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read the configuration file at: {}", config_path))?;

    // Parse the JSON to extract the `path` field
    let config: serde_json::Value = serde_json::from_str(&config_content)
        .with_context(|| "Failed to parse the configuration file as JSON")?;

    // Extract the `path` field
    let file_path = config["path"].as_str().ok_or_else(|| {
        anyhow::anyhow!("`path` field is missing or invalid in configuration file")
    })?;

    // Open the file with the default editor
    // If the editor is not provided, use the default editor
    if editor.is_none() {
        opener::open(file_path)
            .with_context(|| format!("Failed to open the file at path: {}", file_path))
            .map_err(|err| anyhow::anyhow!("Failed to open the file: {}", err))?;
    } else {
        // Use the provided editor
        let editor = editor.unwrap();
        // get the path to the editor
        Command::new(editor.clone())
            .arg(file_path)
            .spawn()
            .with_context(|| {
                format!(
                    "Failed to open the file at path: {} with program: {}",
                    file_path, editor
                )
            })?;
    }
    Ok(())
}
