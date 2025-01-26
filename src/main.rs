use anyhow::Result;
use clap::Parser;

mod commands;
mod templates;

fn main() -> Result<()> {
    println!("Hello, world!");
    match commands::init::execute() {
        Ok(config) => {
            // println!("config: {:?}", config);
            templates::generate_template_file_paths(&config);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    Ok(())
}
