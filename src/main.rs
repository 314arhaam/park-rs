mod clitools;
mod head;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = clitools::Cli::parse();
    let res = match cli.command {
        clitools::Commands::Head { filename, num } => {
            head::subcommand(filename, num)?;
        }
    };
    Ok(res)
}