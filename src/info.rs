use parquet::file::reader::{FileReader, SerializedFileReader};
use std::fs::File;

pub fn subcommand(filename: &String) -> Result<(), Box<dyn std::error::Error>>{
    println!("{}", filename);
    let file = File::open(&filename)?;
    let data = SerializedFileReader::new(file)?;
    let metadata = data.metadata();
    println!("{:#?}", metadata.file_metadata().schema().get_fields());
    Ok(())
}