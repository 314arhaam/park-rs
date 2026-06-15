mod clitools;
mod head;
mod info;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = clitools::Cli::parse();
    cli.command.run()
}