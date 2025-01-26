use crate::commands::init::Formatter;
use crate::commands::init::Linter;
use crate::commands::init::ProjectConfig;
use crate::commands::init::TestTool;

use anyhow::Result;

#[derive(Debug)]
pub struct TemplateFile {
    pub file_path: String,
    pub content: String,
}

pub fn execute(config: &ProjectConfig) -> Result<()> {
    // プロジェクト名をkebab-caseに変換
    let kebab_case_name = to_kebab_case(&config.name);
    let pascal_case_name = to_pascal_case(&config.name);

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

    // distディレクトリが存在しない場合は作成
    std::fs::create_dir_all("dist")?;

    // 基本ファイルを生成
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
        let dist_path = format!("dist/{}", file.file_path.replace("templates/", ""));
        if let Some(parent) = std::path::Path::new(&dist_path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&dist_path, &file.content)?;
    }

    let optional_files = [
        &lint_config_file,
        &test_config_file,
        &formatter_config_file,
    ];

    for optional_file in optional_files.iter() {
        if let Some(file) = optional_file {
            let dist_path = format!("dist/{}", file.file_path.replace("templates/", ""));
            std::fs::write(&dist_path, &file.content)?;
        }
    }
    
    Ok(())
}

fn generate_tsconfig() -> Result<TemplateFile> {
    let tsconfig_path = format!("templates/tsconfig.json");

    // tsconfig.jsonの内容を読み込む
    let content = std::fs::read_to_string(&tsconfig_path)?;

    Ok(TemplateFile {
        file_path: tsconfig_path,
        content,
    })
}

fn generate_readme() -> Result<TemplateFile> {
    let readme_path = format!("templates/README.md");

    // README.mdの内容を読み込む
    let content = std::fs::read_to_string(&readme_path)?;

    Ok(TemplateFile {
        file_path: readme_path,
        content,
    })
}

fn generate_package_json(config: &ProjectConfig, project_name: &str) -> Result<TemplateFile> {
    let package_json_path = format!("templates/package.json");

    // package.jsonの内容を読み込む
    let mut content = std::fs::read_to_string(&package_json_path)?;

    // %project-name%をkebab-caseに変換したconfig.nameで置き換える
    content = content.replace("%project-name%", project_name);

    // テストコマンドの置き換え
    let test_command = match config.test_tool {
        TestTool::Vitest => "test\": \"vitest --run",
        TestTool::Jest => "test\": \"jest",
        TestTool::None => "",
    };
    content = content.replace("%test_command%", test_command);

    // リントコマンドの置き換え
    let lint_command = match config.linter {
        Linter::EsLint => "lint\": \"eslint --config eslint.config.mjs",
        Linter::Biome => "lint\": \"biome lint",
        Linter::None => "",
    };
    content = content.replace("%lint_command%", lint_command);

    // フォーマットコマンドの置き換え
    let format_command = match config.formatter {
        Formatter::Prettier => {
            "format\": \"prettier --write \'**/*.ts\' --ignore-path .prettierignore"
        }
        Formatter::Biome => "format\": \"biome format\"",
        Formatter::None => "",
    };
    content = content.replace("%format_command%", format_command);

    // テストモジュールの置き換え
    let test_module = match config.test_tool {
        TestTool::Vitest => "vitest\": \"^3.0.4",
        TestTool::Jest => {
            "@types/jest\": \"^29.5.14\",\n    \"jest\": \"^29.7.0\",\n    \"ts-jest\": \"^29.2.5"
        }
        TestTool::None => "",
    };
    content = content.replace("%test_module%", test_module);

    // リントモジュールの置き換え
    let lint_module = match config.linter {
        Linter::EsLint => "@eslint/js\": \"^9.19.0\",\n    \"typescript-eslint\": \"^8.14.0\",\n    \"eslint-cdk-plugin\": \"^1.1.1",
        Linter::Biome => "biome\": \"^1.6.0",
        Linter::None => "",
    };
    content = content.replace("%lint_module%", lint_module);

    // フォーマットモジュールの置き換え
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
        file_path: package_json_path,
        content,
    })
}

fn generate_cdk_json(project_name: &str) -> Result<TemplateFile> {
    let cdk_json_path = format!("templates/cdk.json");

    // cdk.jsonの内容を読み込む
    let mut content = std::fs::read_to_string(&cdk_json_path)?;

    // %project-name%をkebab-caseに変換したconfig.nameで置き換える
    content = content.replace("%project-name%", project_name);

    Ok(TemplateFile {
        file_path: cdk_json_path,
        content,
    })
}

fn generate_gitignore(config: &ProjectConfig) -> Result<TemplateFile> {
    let gitignore_path = format!("templates/.gitignore");

    // .gitignoreの内容を読み込む
    let mut content = std::fs::read_to_string(&gitignore_path)?;

    // テストモジュールの置き換え
    let test_file = match config.test_tool {
        TestTool::Vitest => "vitest.config.mjs",
        TestTool::Jest => "jest.config.mjs",
        TestTool::None => "",
    };
    content = content.replace("%test_file%", test_file);

    Ok(TemplateFile {
        file_path: gitignore_path,
        content,
    })
}

fn generate_test_file(kebab_case_name: &str, pascal_case_name: &str) -> Result<TemplateFile> {
    // テンプレートファイルのパスを正しく設定
    let template_path = "templates/test/%project-name%.test.ts";

    // 実際のファイルパスを生成（%project-name%をkebab_case_nameで置換）
    let actual_file_path = template_path.replace("%project-name%", kebab_case_name);

    // テンプレートファイルの内容を読み込む
    let mut content = std::fs::read_to_string(template_path)?;

    // ファイル内の置換を実行
    content = content.replace("%project-name%", kebab_case_name);
    content = content.replace("%ProjectName%", pascal_case_name);

    Ok(TemplateFile {
        file_path: actual_file_path,
        content,
    })
}

fn generate_lib_file(kebab_case_name: &str, pascal_case_name: &str) -> Result<TemplateFile> {
    // テンプレートファイルのパスを正しく設定
    let template_path = "templates/lib/%project-name%-stack.ts";

    // 実際のファイルパスを生成（%project-name%をkebab_case_nameで置換）
    let actual_file_path = template_path.replace("%project-name%", kebab_case_name);

    // テンプレートファイルの内容を読み込む
    let mut content = std::fs::read_to_string(template_path)?;

    // ファイル内の置換を実行
    content = content.replace("%project-name%", kebab_case_name);
    content = content.replace("%ProjectName%", pascal_case_name);

    Ok(TemplateFile {
        file_path: actual_file_path,
        content,
    })
}

fn generate_bin_file(kebab_case_name: &str, pascal_case_name: &str) -> Result<TemplateFile> {
    // テンプレートファイルのパスを正しく設定
    let template_path = "templates/bin/%project-name%.ts";

    // 実際のファイルパスを生成（%project-name%をkebab_case_nameで置換）
    let actual_file_path = template_path.replace("%project-name%", kebab_case_name);

    // テンプレートファイルの内容を読み込む
    let mut content = std::fs::read_to_string(template_path)?;

    // ファイル内の置換を実行
    content = content.replace("%project-name%", kebab_case_name);
    content = content.replace("%ProjectName%", pascal_case_name);

    Ok(TemplateFile {
        file_path: actual_file_path,
        content,
    })
}

fn generate_lint_config_file(config: &ProjectConfig) -> Result<Option<TemplateFile>> {
    let lint_config = match config.linter {
        Linter::EsLint => {
            let lint_config_path = format!("templates/eslint.config.mjs");
            let content = std::fs::read_to_string(&lint_config_path)?;
            Some(TemplateFile {
                file_path: lint_config_path,
                content,
            })
        }
        Linter::Biome => {
            let lint_config_path = format!("templates/biome.json");
            let content = std::fs::read_to_string(&lint_config_path)?;
            Some(TemplateFile {
                file_path: lint_config_path,
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
            let test_config_path = format!("templates/vitest.config.mjs");
            let content = std::fs::read_to_string(&test_config_path)?;
            Some(TemplateFile {
                file_path: test_config_path,
                content,
            })
        }
        TestTool::Jest => {
            let test_config_path = format!("templates/jest.config.js");
            let content = std::fs::read_to_string(&test_config_path)?;
            Some(TemplateFile {
                file_path: test_config_path,
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
            let formatter_config_path = format!("templates/.prettierrc");
            let content = std::fs::read_to_string(&formatter_config_path)?;
            Some(TemplateFile {
                file_path: formatter_config_path,
                content,
            })
        }
        Formatter::Biome => None,
        Formatter::None => None,
    };
    Ok(formatter_config)
}


fn generate_npmignore() -> Result<TemplateFile> {
    let npmignore_path = format!("templates/.npmignore");

    // tsconfig.jsonの内容を読み込む
    let content = std::fs::read_to_string(&npmignore_path)?;

    Ok(TemplateFile {
        file_path: npmignore_path,
        content,
    })
}

fn to_kebab_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            result.push('-');
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    result.replace(' ', "-").replace('_', "-")
}

fn to_pascal_case(s: &str) -> String {
    let words: Vec<String> = s
        .split(|c| c == '-' || c == '_' || c == ' ')
        .filter(|s| !s.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let mut word = first.to_ascii_uppercase().to_string();
                    word.extend(chars.map(|c| c.to_ascii_lowercase()));
                    word
                }
            }
        })
        .collect();

    words.join("")
}
