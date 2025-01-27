use anyhow::Result;

mod cli;
mod templates;

fn run() -> Result<()> {
    let config = cli::init::convert_project_config()?;
    templates::generate::execute(&config)?;
    cli::init::install_dependencies(&config)?;
    Ok(())
}

fn main() -> Result<()> {
    if let Err(e) = run() {
        println!("Error: {}", e);
    }
    Ok(())
}
