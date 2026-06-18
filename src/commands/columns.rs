use parquet::file::reader::FileReader;

pub fn subcommand(filename: &String) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(filename)?;
    let data = parquet::file::reader::SerializedFileReader::new(file)?;
    for col in data.metadata().file_metadata().schema().get_fields() {
        println!("{}", col.name())
    }
    Ok(())
}