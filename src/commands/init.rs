use anyhow::Result;
use dialoguer::{Input, Select};

#[derive(Debug)]
pub struct ProjectConfig {
    pub name: String,
    pub package_manager: PackageManager,
    pub linter: Linter,
    pub formatter: Formatter,
    pub test_tool: TestTool,
}

#[derive(Debug)]
pub enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
}

#[derive(Debug)]
pub enum Linter {
    EsLint,
    Biome,
    None,
}

#[derive(Debug)]
pub enum Formatter {
    Prettier,
    Biome,
    None,
}

#[derive(Debug)]
pub enum TestTool {
    Jest,
    Vitest,
    None,
}

pub fn execute() -> Result<ProjectConfig> {
    let config = collect_user_input()?;
    println!("config: {:?}", config);
    Ok(config)
}

fn collect_user_input() -> Result<ProjectConfig> {
    let name = Input::<String>::new()
        .with_prompt("Project name")
        .interact()?;

    // NOTE: select package manager
    let package_managers = &["npm", "yarn", "pnpm"];
    let package_manager = Select::new()
        .with_prompt("Choose a package manager")
        .items(package_managers)
        .default(0)
        .interact()?;

    // NOTE: select linter
    let linters = &["eslint", "biome", "none"];
    let linter = Select::new()
        .with_prompt("Choose a linter")
        .items(linters)
        .default(0)
        .interact()?;

    // NOTE: select formatter
    let formatters = &["prettier", "biome", "none"];
    let formatter = Select::new()
        .with_prompt("Choose a formatter")
        .items(formatters)
        .default(0)
        .interact()?;

    // NOTE: select test tools
    let test_tools = &["jest", "vitest", "none"];
    let test_tool = Select::new()
        .with_prompt("Choose a test tool")
        .items(test_tools)
        .default(0)
        .interact()?;

    println!("name: {}", name);
    println!("package_manager: {}", package_managers[package_manager]);
    println!("linter: {}", linters[linter]);
    println!("formatter: {}", formatters[formatter]);
    println!("test_tool: {}", test_tools[test_tool]);

    Ok(ProjectConfig {
        name,
        package_manager: match package_managers[package_manager] {
            "npm" => PackageManager::Npm,
            "yarn" => PackageManager::Yarn,
            "pnpm" => PackageManager::Pnpm,
            _ => unreachable!(),
        },
        linter: match linters[linter] {
            "eslint" => Linter::EsLint,
            "biome" => Linter::Biome,
            "none" => Linter::None,
            _ => unreachable!(),
        },
        test_tool: match test_tools[test_tool] {
            "jest" => TestTool::Jest,
            "vitest" => TestTool::Vitest,
            "none" => TestTool::None,
            _ => unreachable!(),
        },
        formatter: match formatters[formatter] {
            "prettier" => Formatter::Prettier,
            "biome" => Formatter::Biome,
            "none" => Formatter::None,
            _ => unreachable!(),
        },
    })
}
