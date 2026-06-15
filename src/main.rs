use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Head {
        filename: String,

        #[arg(short, long, default_value_t = 5_u32)]
        num: u32,
    },
}
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let cli = Cli::parse();
    match cli.command {
        Commands::Head { filename, num } => {
            let file = std::fs::File::open(std::path::Path::new(&filename))?;
            let data = parquet::file::reader::SerializedFileReader::new(file)?;
            let mut i: u32 = 0;
            for line in data.into_iter() {
                println!("{:?}", line);
                i+=1;
                if i >= num {
                    break
                }
            }
        }
    }
    Ok(())
}
