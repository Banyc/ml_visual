use clap::Parser;
use xtask::Command;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

fn main() {
    let cli = Cli::parse();
    cli.command.run();
}
