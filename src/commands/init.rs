use anyhow::Result;
use dialoguer::{Input, Select};

#[derive(Debug)]
pub enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
}

pub fn execute() {
    collect_user_input().unwrap();
}

fn collect_user_input() -> Result<()> {
    let name = Input::<String>::new()
        .with_prompt("Project name")
        .interact()?;
    let package_managers = &["npm", "yarn", "pnpm"];
    let package_manager = Select::new()
        .with_prompt("Choose a package manager")
        .items(package_managers)
        .interact()?;

    println!("name: {}", name);
    println!("package_manager: {}", package_managers[package_manager]);

    Ok(())
}
