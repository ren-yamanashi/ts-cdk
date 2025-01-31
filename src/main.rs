extern crate anyhow;
extern crate dialoguer;
extern crate include_dir;

use anyhow::Result;
use std::env;

mod cli;
mod templates;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        anyhow::bail!("Usage: ts-cdk <command> <directory>");
    }

    let command = &args[1];
    let target_dir = &args[2];

    match command.as_str() {
        "init" => {
            let config = cli::init::generate_project_config(&vec![target_dir.to_string()])?;
            templates::generate::generate_template_files(&config)?;
            cli::init::install_dependencies(&config)?;
        }
        _ => {
            anyhow::bail!("Invalid command. Available commands: init");
        }
    }

    Ok(())
}
