use parquet::file::reader::FileReader;
use crate::info;

pub fn subcommand(filename: &String, num: &u32, csv: &bool) -> Result<(), Box<dyn std::error::Error>>{
    let file = std::fs::File::open(std::path::Path::new(filename))?;
    let data = parquet::file::reader::SerializedFileReader::new(file)?;
    let metadata = data.metadata();
    let _n_rows = metadata.file_metadata().num_rows();
    /*let rows_to_skip = if (*num) as i64 > n_rows {
        0 as usize
    } else {
        (n_rows - ((*num) as i64)) as usize
    };*/
    if *csv {
        for col in info::get_column_names(metadata){
            print!("{:?},", col);
        }
        println!("");
    }
    for line in data.into_iter().skip((_n_rows - ((*num) as i64)) as usize) {
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