use clap::Parser;
use anyhow::Result;

mod commands;

fn main() -> Result<()> {
    println!("Hello, world!");
    commands::init::execute();
    Ok(())
}