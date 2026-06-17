use parquet::file::{metadata::{ParquetMetaData}, reader::{FileReader, SerializedFileReader}};
use std::fs::File;
use crate::info;

pub fn subcommand(filename: &String) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(&filename)?;
    let data = SerializedFileReader::new(file)?;
    let metadata = data.metadata();
    let cols = info::get_column_names(metadata);
    println!("Filename:\t{}\nColumns:\t{}\nRows:\t\t{}", filename, cols.len(), metadata.file_metadata().num_rows());
    Ok(())
}