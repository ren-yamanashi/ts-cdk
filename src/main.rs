use anyhow::Result;
use clap::Parser;

mod commands;
mod templates;

fn main() -> Result<()> {
    println!("Hello, world!");
    match commands::init::execute() {
        Ok(config) => {
            // println!("config: {:?}", config);
            templates::execute(&config);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    Ok(())
}
