use clap::{Parser, Subcommand};
use crate::head;
use crate::info;


#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Head {
        filename: String,

        #[arg(short, long, default_value_t = 5_u32)]
        num: u32,
    },

    Info {
        filename: String,
    }
}

impl Commands {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>>{
        let res = match self {
            Commands::Head { filename, num } => {
                head::subcommand(filename, num)?
            }
            Commands::Info { filename } => {
                info::subcommand(filename)?
            }
        };
        Ok(res)
    }
}