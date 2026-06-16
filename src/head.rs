use parquet::file::reader::FileReader;
use crate::info;

pub fn subcommand(filename: &String, num: &u32, csv: &bool) -> Result<(), Box<dyn std::error::Error>>{
    let file = std::fs::File::open(std::path::Path::new(filename))?;
    let data = parquet::file::reader::SerializedFileReader::new(file)?;
    let metadata = data.metadata();
    let n_rows = metadata.file_metadata().num_rows();
    /*let rows_to_take = if (*num) as i64 > n_rows {
        n_rows as usize
    } else {
        (*num) as usize
    };*/
    if *csv {
        for col in info::get_column_names(metadata){
            print!("{:?},", col);
        }
        println!("");
    }
    for line in data.into_iter().take((*num) as usize) {
        let v = line?;
        if *csv {
                for val in v.into_columns() {
                    print!("{},", val.1);
                }
            } else {
                print!("{:?}", v);
            }
        println!("");
        }
    Ok(())
}