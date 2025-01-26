use anyhow::Result;

mod commands;
mod templates;

fn main() -> Result<()> {
    match commands::init::execute() {
        Ok(config) => {
            templates::execute(&config)?;
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    Ok(())
}
