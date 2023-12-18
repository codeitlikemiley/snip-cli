use crate::models::commands_model::Commands;
use clap::Parser;

#[derive(Parser)]
#[clap(name = "snip")]
#[command(author,version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
