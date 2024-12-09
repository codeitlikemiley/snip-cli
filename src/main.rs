use anyhow::{self, Context, Result};
use clap::Parser;
use snip_cli::actions::create_directory_and_file::create_directory_and_file;
use snip_cli::actions::edit_snippet_in_file::edit_snippet_in_file;
use snip_cli::actions::list_snippets::list_snippets;
use snip_cli::actions::open_file_with::open_file_with;
use snip_cli::actions::remove_snippet_from_file::remove_snippet_from_file;
use snip_cli::actions::search_snippets::search_snippets;
use snip_cli::actions::show_snippet::show_snippet;
use snip_cli::actions::update_key_in_file::update_key_in_file;
use snip_cli::actions::write_snippet_to_file::write_snippet_to_file;
use snip_cli::constants::DEFAULT_SNIPPET_PATH;
use snip_cli::helpers::expand_home_dir::expand_home_dir;
use snip_cli::helpers::get_app_config::get_app_config;
use snip_cli::models::cli_model::Cli;
use snip_cli::models::commands_model::Commands;
use snip_cli::models::snip_config_model::SnipConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = get_app_config();

    create_directory_and_file(&config_path)?;

    let config = match SnipConfig::load(&config_path) {
        Ok(cfg) => cfg,
        Err(_) => {
            let default_snippet_path = expand_home_dir(DEFAULT_SNIPPET_PATH)
                .to_string_lossy()
                .into_owned();
            let new_config = SnipConfig {
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
            println!("{}", output);
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
            println!("{}", output);
        }
        Commands::Search { id, name } => {
            dbg!(id, &name);
            let output = search_snippets(&config.path, id, name)
                .context("Failed to search snippet from file")?;

            for result in output {
                println!("{}\n", result);
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
        Commands::Open { editor } => {
            open_file_with(&config_path, editor)
                .context("Failed to open the configuration file")?;
        }
    }

    Ok(())
}
