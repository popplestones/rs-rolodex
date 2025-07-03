use clap::Parser;

#[derive(Parser)]
#[command(name = "rolodex")]
#[command(author, version, about)]
pub struct Cli {
    /// Seed fake users into the database
    #[arg(long)]
    pub seed: Option<u32>,
}
