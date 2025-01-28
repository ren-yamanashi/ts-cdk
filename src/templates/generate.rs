use crate::cli::init::Formatter;
use crate::cli::init::Linter;
use crate::cli::init::ProjectConfig;
use crate::cli::init::TestTool;
use crate::templates::assets::TEMPLATES;

use anyhow::Result;

#[derive(Debug)]
pub struct TemplateFile {
    pub file_path: String,
    pub content: String,
}

/// Generate template files
///
/// ### Parameters
/// - `config` - ProjectConfig
pub fn generate_template_files(config: &ProjectConfig) -> Result<()> {
    let kebab_case_name = convert_to_kebab_case(&config.name);
    let pascal_case_name = kebab_case_to_pascal_case(&kebab_case_name);

    let tsconfig = generate_tsconfig()?;
    let readme = generate_readme()?;
    let package_json = generate_package_json(config, &kebab_case_name)?;
    let cdk_json = generate_cdk_json(&kebab_case_name)?;
    let gitignore = generate_gitignore(config)?;
    let npmignore_file = generate_npmignore()?;
    let test_file = generate_test_file(&kebab_case_name, &pascal_case_name)?;
    let lib_file = generate_lib_file(&kebab_case_name, &pascal_case_name)?;
    let bin_file = generate_bin_file(&kebab_case_name, &pascal_case_name)?;

    let lint_config_file = generate_lint_config_file(config)?;
    let test_config_file = generate_test_config_file(config)?;
    let formatter_config_file = generate_formatter_config_file(config)?;

    // when directory is not exists, create it
    std::fs::create_dir_all(&config.target_dir_path)?;

    let base_files = [
        &package_json,
        &tsconfig,
        &readme,
        &cdk_json,
        &gitignore,
        &npmignore_file,
        &test_file,
        &lib_file,
        &bin_file,
    ];

    for file in base_files.iter() {
        let target_dir_path = format!(
            "{}/{}",
            config.target_dir_path,
            file.file_path.replace("templates/", "")
        );
        if let Some(parent) = std::path::Path::new(&target_dir_path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&target_dir_path, &file.content)?;
    }

    let optional_files = [&lint_config_file, &test_config_file, &formatter_config_file];

    for optional_file in optional_files.iter() {
        if let Some(file) = optional_file {
            let dir_path = format!(
                "{}/{}",
                config.target_dir_path,
                file.file_path.replace("templates/", "")
            );
            std::fs::write(&dir_path, &file.content)?;
        }
    }

    Ok(())
}

fn generate_tsconfig() -> Result<TemplateFile> {
    let file_path = "templates/tsconfig.json";
    let content = TEMPLATES
        .get_file("tsconfig.json")
        .ok_or_else(|| anyhow::anyhow!("Failed to load tsconfig.json template"))?
        .contents_utf8()
        .ok_or_else(|| anyhow::anyhow!("Failed to read tsconfig.json template as UTF-8"))?
        .to_string();

    Ok(TemplateFile {
        file_path: file_path.to_string(),
        content,
    })
}

fn generate_readme() -> Result<TemplateFile> {
    let file_path = "templates/README.md";
    let content = TEMPLATES
        .get_file("README.md")
        .ok_or_else(|| anyhow::anyhow!("Failed to load README.md template"))?
        .contents_utf8()
        .ok_or_else(|| anyhow::anyhow!("Failed to read README.md template as UTF-8"))?
        .to_string();

    Ok(TemplateFile {
        file_path: file_path.to_string(),
        content,
    })
}

fn generate_npmignore() -> Result<TemplateFile> {
    let file_path = "templates/.npmignore";
    let content = TEMPLATES
        .get_file(".npmignore")
        .ok_or_else(|| anyhow::anyhow!("Failed to load .npmignore template"))?
        .contents_utf8()
        .ok_or_else(|| anyhow::anyhow!("Failed to read .npmignore template as UTF-8"))?
        .to_string();

    Ok(TemplateFile {
        file_path: file_path.to_string(),
        content,
    })
}

fn generate_package_json(config: &ProjectConfig, project_name: &str) -> Result<TemplateFile> {
    let file_path = "templates/package.json";
    let mut content = TEMPLATES
        .get_file("package.json")
        .ok_or_else(|| anyhow::anyhow!("Failed to load package.json template"))?
        .contents_utf8()
        .ok_or_else(|| anyhow::anyhow!("Failed to read package.json template as UTF-8"))?
        .to_string();

    // Replace %project-name%
    content = content.replace("%project-name%", project_name);

    // Replace %test_command%
    let test_command = match config.test_tool {
        TestTool::Vitest => "test\": \"vitest --run",
        TestTool::Jest => "test\": \"jest",
        TestTool::None => "",
    };
    content = content.replace("%test_command%", test_command);

    // Replace %lint_command%
    let lint_command = match config.linter {
        Linter::EsLint => "lint\": \"eslint --config eslint.config.mjs",
        Linter::Biome => "lint\": \"biome lint",
        Linter::None => "",
    };
    content = content.replace("%lint_command%", lint_command);

    // Replace %format_command%
    let format_command = match config.formatter {
        Formatter::Prettier => {
            "format\": \"prettier --write \'**/*.ts\' --ignore-path .prettierignore"
        }
        Formatter::Biome => "format\": \"biome format",
        Formatter::None => "",
    };
    content = content.replace("%format_command%", format_command);

    // Replace %test_module%
    let test_module = match config.test_tool {
        TestTool::Vitest => "vitest\": \"^3.0.4",
        TestTool::Jest => {
            "@types/jest\": \"^29.5.14\",\n    \"jest\": \"^29.7.0\",\n    \"ts-jest\": \"^29.2.5"
        }
        TestTool::None => "",
    };
    content = content.replace("%test_module%", test_module);

    // Replace %lint_module%
    let lint_module = match config.linter {
        Linter::EsLint => "@eslint/js\": \"^9.19.0\",\n    \"typescript-eslint\": \"^8.14.0\",\n    \"eslint-cdk-plugin\": \"^1.1.1",
        Linter::Biome => "biome\": \"^1.6.0",
        Linter::None => "",
    };
    content = content.replace("%lint_module%", lint_module);

    // Replace %format_module%
    let format_module = match config.formatter {
        Formatter::Prettier => "prettier\": \"^3.4.2",
        Formatter::Biome => match config.linter {
            Linter::Biome => "",
            _ => "biome\": \"^1.6.0",
        },
        Formatter::None => "",
    };
    content = content.replace("%format_module%", format_module);

    Ok(TemplateFile {
        file_path: file_path.to_string(),
        content: remove_empty_lines_and_quotes(&content),
    })
}

fn generate_cdk_json(project_name: &str) -> Result<TemplateFile> {
    let file_path = "templates/cdk.json";
    let mut content = TEMPLATES
        .get_file("cdk.json")
        .ok_or_else(|| anyhow::anyhow!("Failed to load cdk.json template"))?
        .contents_utf8()
        .ok_or_else(|| anyhow::anyhow!("Failed to read cdk.json template as UTF-8"))?
        .to_string();

    // Replace %project-name%
    content = content.replace("%project-name%", project_name);

    Ok(TemplateFile {
        file_path: file_path.to_string(),
        content,
    })
}

fn generate_gitignore(config: &ProjectConfig) -> Result<TemplateFile> {
    let file_path = "templates/.gitignore";
    let mut content = TEMPLATES
        .get_file(".gitignore")
        .ok_or_else(|| anyhow::anyhow!("Failed to load .gitignore template"))?
        .contents_utf8()
        .ok_or_else(|| anyhow::anyhow!("Failed to read .gitignore template as UTF-8"))?
        .to_string();

    // Replace %test_file%
    let test_file = match config.test_tool {
        TestTool::Vitest => "vitest.config.mjs",
        TestTool::Jest => "jest.config.mjs",
        TestTool::None => "",
    };
    content = content.replace("%test_file%", test_file);

    Ok(TemplateFile {
        file_path: file_path.to_string(),
        content,
    })
}

fn generate_test_file(kebab_case_name: &str, pascal_case_name: &str) -> Result<TemplateFile> {
    let file_path = "templates/test/%project-name%.test.ts".replace("%project-name%", kebab_case_name);
    let mut content = TEMPLATES
        .get_file("test/%project-name%.test.ts")
        .ok_or_else(|| anyhow::anyhow!("Failed to load test file template"))?
        .contents_utf8()
        .ok_or_else(|| anyhow::anyhow!("Failed to read test file template as UTF-8"))?
        .to_string();

    // Replace %project-name%
    content = content.replace("%project-name%", kebab_case_name);
    content = content.replace("%ProjectName%", pascal_case_name);

    Ok(TemplateFile {
        file_path: file_path.to_string(),
        content,
    })
}

fn generate_lib_file(kebab_case_name: &str, pascal_case_name: &str) -> Result<TemplateFile> {
    let file_path = "templates/lib/%project-name%-stack.ts".replace("%project-name%", kebab_case_name);
    let mut content = TEMPLATES
        .get_file("lib/%project-name%-stack.ts")
        .ok_or_else(|| anyhow::anyhow!("Failed to load lib file template"))?
        .contents_utf8()
        .ok_or_else(|| anyhow::anyhow!("Failed to read lib file template as UTF-8"))?
        .to_string();

    // Replace %project-name%, %ProjectName%
    content = content.replace("%project-name%", kebab_case_name);
    content = content.replace("%ProjectName%", pascal_case_name);

    Ok(TemplateFile {
        file_path: file_path.to_string(),
        content,
    })
}

fn generate_bin_file(kebab_case_name: &str, pascal_case_name: &str) -> Result<TemplateFile> {
    let file_path = "templates/bin/%project-name%.ts".replace("%project-name%", kebab_case_name);
    let mut content = TEMPLATES
        .get_file("bin/%project-name%.ts")
        .ok_or_else(|| anyhow::anyhow!("Failed to load bin file template"))?
        .contents_utf8()
        .ok_or_else(|| anyhow::anyhow!("Failed to read bin file template as UTF-8"))?
        .to_string();

    // Replace %project-name%, %ProjectName%
    content = content.replace("%project-name%", kebab_case_name);
    content = content.replace("%ProjectName%", pascal_case_name);

    Ok(TemplateFile {
        file_path: file_path.to_string(),
        content,
    })
}

fn generate_lint_config_file(config: &ProjectConfig) -> Result<Option<TemplateFile>> {
    let lint_config = match config.linter {
        Linter::EsLint => {
            let file_path = "templates/eslint.config.mjs";
            let content = TEMPLATES
                .get_file("eslint.config.mjs")
                .ok_or_else(|| anyhow::anyhow!("Failed to load eslint.config.mjs template"))?
                .contents_utf8()
                .ok_or_else(|| anyhow::anyhow!("Failed to read eslint.config.mjs template as UTF-8"))?
                .to_string();
            Some(TemplateFile {
                file_path: file_path.to_string(),
                content,
            })
        }
        Linter::Biome => {
            let file_path = "templates/biome.json";
            let content = TEMPLATES
                .get_file("biome.json")
                .ok_or_else(|| anyhow::anyhow!("Failed to load biome.json template"))?
                .contents_utf8()
                .ok_or_else(|| anyhow::anyhow!("Failed to read biome.json template as UTF-8"))?
                .to_string();
            Some(TemplateFile {
                file_path: file_path.to_string(),
                content,
            })
        }
        Linter::None => None,
    };
    Ok(lint_config)
}

fn generate_test_config_file(config: &ProjectConfig) -> Result<Option<TemplateFile>> {
    let test_config = match config.test_tool {
        TestTool::Vitest => {
            let file_path = "templates/vitest.config.mjs";
            let content = TEMPLATES
                .get_file("vitest.config.mjs")
                .ok_or_else(|| anyhow::anyhow!("Failed to load vitest.config.mjs template"))?
                .contents_utf8()
                .ok_or_else(|| anyhow::anyhow!("Failed to read vitest.config.mjs template as UTF-8"))?
                .to_string();
            Some(TemplateFile {
                file_path: file_path.to_string(),
                content,
            })
        }
        TestTool::Jest => {
            let file_path = "templates/jest.config.js";
            let content = TEMPLATES
                .get_file("jest.config.js")
                .ok_or_else(|| anyhow::anyhow!("Failed to load jest.config.js template"))?
                .contents_utf8()
                .ok_or_else(|| anyhow::anyhow!("Failed to read jest.config.js template as UTF-8"))?
                .to_string();
            Some(TemplateFile {
                file_path: file_path.to_string(),
                content,
            })
        }
        TestTool::None => None,
    };
    Ok(test_config)
}

fn generate_formatter_config_file(config: &ProjectConfig) -> Result<Option<TemplateFile>> {
    let formatter_config = match config.formatter {
        Formatter::Prettier => {
            let file_path = "templates/.prettierrc";
            let content = TEMPLATES
                .get_file(".prettierrc")
                .ok_or_else(|| anyhow::anyhow!("Failed to load .prettierrc template"))?
                .contents_utf8()
                .ok_or_else(|| anyhow::anyhow!("Failed to read .prettierrc template as UTF-8"))?
                .to_string();
            Some(TemplateFile {
                file_path: file_path.to_string(),
                content,
            })
        }
        Formatter::Biome => None,
        Formatter::None => None,
    };
    Ok(formatter_config)
}

fn convert_to_kebab_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            result.push('-');
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    result.replace(' ', "-").replace('_', "-")
}

fn kebab_case_to_pascal_case(kebab_case_str: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    for c in kebab_case_str.chars() {
        if c == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c.to_lowercase().next().unwrap());
        }
    }
    result
}

fn remove_empty_lines_and_quotes(content: &str) -> String {
    content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && trimmed != "\"\"" && trimmed != "\"\","
        })
        .collect::<Vec<&str>>()
        .join("\n")
}
