use parquet::file::reader::FileReader;
use crate::info;
use crate::clitools;
use comfy_table::{Table, presets::UTF8_FULL};

pub fn subcommand(filename: &String, num: &u32, format: &clitools::Format) -> Result<(), Box<dyn std::error::Error>>{
    let file = std::fs::File::open(std::path::Path::new(filename))?;
    let data = parquet::file::reader::SerializedFileReader::new(file)?;
    let metadata = data.metadata();
    let _n_rows = metadata.file_metadata().num_rows();
    let mut table = Table::new();
    match *format {
        clitools::Format::Csv => {
            for col in info::get_column_names(metadata){
                print!("{:?},", col);
            }
            println!("");
        }
        clitools::Format::Table => {
            table.load_preset(UTF8_FULL);
            table.set_header(info::get_column_names(metadata));
        }
        _ => {}
    }
    for line in data.into_iter().take((*num) as usize) {
        let v = line?;
        match *format {
            clitools::Format::Csv => {
                for val in v.into_columns() {
                        print!("{},", val.1);
                }
                println!("");
            } 
            clitools::Format::Raw => {
                    print!("{:?}", v);
                    println!("");
            }
            clitools::Format::Table => {
                let mut data: Vec<String> = vec![];
                for val in v.into_columns() {
                    data.push(String::from(val.1.to_string()));
                }
                table.add_row(data);
            }
        }
    }
    match *format {
        clitools::Format::Table => {
            print!("{table}");
        }
        _ => {}
    }
    Ok(())
}