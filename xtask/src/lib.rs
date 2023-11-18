use build::BuildArgs;
use clap::Subcommand;
use decompile::DecompileArgs;

mod build;
mod decompile;

#[derive(Debug, Subcommand)]
pub enum Command {
    Build(BuildArgs),
    Decompile(DecompileArgs),
}

impl Command {
    pub fn run(self) {
        match self {
            Command::Build(args) => args.run(),
            Command::Decompile(args) => args.run(),
        }
    }
}
