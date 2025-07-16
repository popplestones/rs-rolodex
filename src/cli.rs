use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rolodex")]
#[command(author, version, about)]
pub struct Cli {
    /// Seed fake users into the database
    #[arg(long)]
    pub seed: Option<u32>,

    /// Backup contacts to a JSON file
    #[arg(long)]
    pub backup: Option<PathBuf>,

    /// Restore contacts from a JSON backup file
    #[arg(long)]
    pub restore: Option<PathBuf>,
}
