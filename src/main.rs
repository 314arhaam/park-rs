mod clitools;
mod commands;
mod iotools;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = clitools::Cli::parse();
    cli.command.run()
}