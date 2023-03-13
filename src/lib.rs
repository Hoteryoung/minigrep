use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            Err("Not enough arguments")
        } else {
            Ok(Self{query: args[1].clone(), file_path: args[2].clone()})
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    println!("With the content: \n{}", content);
    Ok(())
}