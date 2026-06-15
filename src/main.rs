mod clitools;
mod head;
mod info;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = clitools::Cli::parse();
    let res = cli.command.run()?;
    Ok(res)
}