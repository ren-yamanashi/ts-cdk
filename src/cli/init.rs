use anyhow::Result;
use dialoguer::{Input, Select};

#[derive(Debug)]
pub struct ProjectConfig {
    pub target_dir_path: String,
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

/// Generate ProjectConfig interactively.
///
/// ### Parameters
/// - `args` - Arguments from the command line(target directory path)
///
/// ### Returns
/// - `ProjectConfig` - ProjectConfig
pub fn generate_project_config(args: &Vec<String>) -> Result<ProjectConfig> {
    let target_dir_path = convert_to_dir_path(&args)?;

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
        target_dir_path,
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

/// Install dependencies.
///
/// ### Parameters
/// - `config` - ProjectConfig
pub fn install_dependencies(config: &ProjectConfig) -> Result<()> {
    match config.package_manager {
        PackageManager::Npm => {
            std::process::Command::new("npm")
                .arg("install")
                .current_dir(&config.target_dir_path)
                .status()?;
        }
        PackageManager::Yarn => {
            std::process::Command::new("yarn")
                .arg("install")
                .current_dir(&config.target_dir_path)
                .status()?;
        }
        PackageManager::Pnpm => {
            std::process::Command::new("pnpm")
                .arg("install")
                .current_dir(&config.target_dir_path)
                .status()?;
        }
    }
    Ok(())
}

fn convert_to_dir_path(args: &Vec<String>) -> Result<String> {
    let raw_path = if args.is_empty() { "." } else { &args[0] }.to_string();

    // Check references to parent directories.
    if raw_path.contains("../") {
        anyhow::bail!("Reference to parent directories not permitted");
    }

    let target_dir_path = raw_path.replace("./", "").trim_end_matches('/').to_string();

    Ok(target_dir_path)
}
