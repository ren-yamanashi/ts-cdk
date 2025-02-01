extern crate anyhow;
extern crate dialoguer;
extern crate include_dir;

use anyhow::Result;
use std::env;

mod cli;
mod templates;

const AVAILABLE_COMMANDS: [&str; 2] = ["init", "help"];
const USAGE: &str = "Usage: ts-cdk <command> [arguments]

Commands:
    init <directory>    Create a new TypeScript + AWS CDK project in the specified directory
    help                Show this help message

Example:
    `ts-cdk init my-project`    Create a new project in 'my-project' directory";

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("{}", USAGE);
        anyhow::bail!("No command provided");
    }
    let command = &args[1];

    match command.as_str() {
        "init" => {
            if args.len() < 3 {
                println!("{}", USAGE);
                anyhow::bail!("Usage: ts-cdk init <directory>");
            }
            let target_dir = &args[2];
            let config = cli::init::generate_project_config(&vec![target_dir.to_string()])?;
            templates::generate::generate_template_files(&config)?;
            cli::init::install_dependencies(&config)?;
        }
        "help" => {
            println!("{}", USAGE);
        }
        _ => {
            println!("{}", USAGE);
            anyhow::bail!("Invalid command '{}'. Available commands: {}", command, AVAILABLE_COMMANDS.join(", "));
        }
    }

    Ok(())
}
