use crate::*;

#[derive(clap::Args)]
pub struct CrunchCommand {

    #[command(subcommand)]
    words: WordFactory,

    /// How many words to score
    #[arg(short='t', long)]
    total: Option<usize>,
}

impl CrunchCommand {
    pub fn run(&self) {

    }
}
