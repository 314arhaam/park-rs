pub fn subcommand(filename: &String, num: &u32, csv: &bool) -> Result<(), Box<dyn std::error::Error>>{
    let file = std::fs::File::open(std::path::Path::new(filename))?;
    let data = parquet::file::reader::SerializedFileReader::new(file)?;
    let mut i: u32 = 0;
    if *csv {
        'row_loop: for line in data.into_iter() {
            let v = line?;
            for val in v.into_columns() {
                match i {
                    0 => {
                        print!("{},", val.0);
                    }
                    i if i > (*num) => {
                        break 'row_loop;
                    }
                    _ => {
                        print!("{},", val.1);
                    }
                }
            }
            println!("");
            i+=1;
        }
    } else {
        'row_loop: for line in data.into_iter() {
            let v = line?;
            println!("{:?}", v);
            if i > (*num) {
                break 'row_loop;
            }
            i+=1;
        }        
    }
    Ok(())
}