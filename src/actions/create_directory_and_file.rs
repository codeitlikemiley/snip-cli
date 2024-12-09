use anyhow::Context;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

pub fn create_directory_and_file(file_path: &str) -> anyhow::Result<()> {
    let path = Path::new(file_path);

    // Create the directory where the file will reside
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).context("Failed to create directory")?;
    }

    // Create the file (or open if it already exists)
    OpenOptions::new()
        .write(true)
        .create(true) // This will create the file if it doesn't exist
        .truncate(false)
        .open(file_path)
        .context("Failed to create or open the file")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_create_directory_and_file() {
        let temp_dir = tempdir().unwrap();
        let test_dir = temp_dir.path().join("test_data");
        let test_file = test_dir.join("test_file.txt");

        // Ensure the test environment is clean
        let _ = fs::remove_dir_all(&test_dir);

        // Run the function
        let result = create_directory_and_file(test_file.to_str().unwrap());
        assert!(result.is_ok());

        // Check if the directory was created
        assert!(Path::new(&test_dir).exists());

        // Check if the file was created
        assert!(Path::new(&test_file).exists());

        // No need for explicit cleanup, as tempdir() automatically deletes the directory
    }
}
