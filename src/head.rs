pub fn subcommand(filename: String, num:u32) -> Result<(), Box<dyn std::error::Error>>{
    let file = std::fs::File::open(std::path::Path::new(&filename))?;
    let data = parquet::file::reader::SerializedFileReader::new(file)?;
    let mut i: u32 = 0;
    for line in data.into_iter() {
        println!("{:?}", line);
        i+=1;
        if i >= num {
            break;
        }
    }
    Ok(())
}