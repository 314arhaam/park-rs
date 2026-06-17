use clap::{Parser, Subcommand, ValueEnum};
use crate::head;
use crate::tail;
use crate::info;

#[derive(Clone, ValueEnum)]
pub enum Format {
    Csv,
    Raw,
    Table,
}

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

        #[arg(short, long, default_value = "table")]
        format: Format,
    },

    Tail {
        filename: String,

        #[arg(short, long, default_value_t = 5_u32)]
        num: u32,

        #[arg(short, long, default_value = "table")]
        format: Format,
    },

    Info {
        filename: String,
    }
}

impl Commands {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>>{
        let res = match self {
            Commands::Head { filename, num, format } => {
                head::subcommand(filename, num, format)?
            }
            Commands::Info { filename } => {
                info::subcommand(filename)?
            }
            Commands::Tail { filename, num, format } => {
                tail::subcommand(filename, num, format)?
            }
        };
        Ok(res)
    }
}