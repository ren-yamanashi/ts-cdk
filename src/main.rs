use anyhow::Result;
use clap::Parser;

mod commands;
mod templates;

fn main() -> Result<()> {
    match commands::init::execute() {
        Ok(config) => {
            // println!("config: {:?}", config);
            match templates::execute(&config) {
                Ok(files) => {
                    println!("templates executed");
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    Ok(())
}
