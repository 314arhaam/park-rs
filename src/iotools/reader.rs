use parquet::file::{metadata::{ParquetMetaData}, reader::{FileReader, SerializedFileReader}};
use std::fs::File;

pub fn get_column_names(metadata: &ParquetMetaData) -> Vec<String> {
    let mut columns: Vec<String> = vec![];
    for col in metadata.file_metadata().schema().get_fields() {
        columns.push(String::from(col.name()));
    }
    columns
}

pub fn read_parquet(filename: &String) -> Result<SerializedFileReader<File>, Box<dyn std::error::Error>>{
    let file = File::open(&filename)?;
    let data = SerializedFileReader::new(file)?;
    Ok(data)
}