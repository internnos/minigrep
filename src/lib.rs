use std::fs;
use anyhow::{Context, Result};

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config> {
        if args.len() < 3 {
            return Err(anyhow::anyhow!("not enough arguments"));
        }
        let query = args[1].clone();
        let file_path: String = args[2].clone();
        Ok(Config{query, file_path})
    }

    pub fn run(&self) -> Result<()>{
        let contents =fs::read_to_string(&self.file_path).with_context(
            || format!("could not read file `{}`", &self.file_path))?;
        println!("searching from {}", &self.query);
        println!("in a file {}", &self.file_path);
        println!("With text\n{contents}");
        Ok(())
    }
}

