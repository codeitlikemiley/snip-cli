use crate::models::identifier_model::Identifier;
use clap::Subcommand;

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
    Open {
        #[arg(short, long)]
        editor: Option<String>,
    },
}
