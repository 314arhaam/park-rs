pub fn subcommand(filename: &String) -> Result<(), Box<dyn std::error::Error>>{
    println!("{}", filename);
    Ok(())
}